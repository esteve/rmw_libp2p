use std::collections::hash_map::DefaultHasher;
use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

use libp2p::{
    futures::StreamExt, gossipsub, identity, mdns, swarm::NetworkBehaviour, swarm::SwarmEvent,
    PeerId,
};

use tokio::runtime::Runtime;
use tokio::sync::Notify;
use tokio::sync::Mutex;
use tokio::{select, task};

use deadqueue::unlimited::Queue;

#[repr(C)]
pub(crate) struct CustomSubscriptionHandle{
    pub ptr: *const c_void
}

unsafe impl Send for CustomSubscriptionHandle {}
unsafe impl Sync for CustomSubscriptionHandle {}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct RosNetworkBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[derive(Debug)]
enum OutEvent {
    Gossipsub(gossipsub::Event),
    Mdns(mdns::Event),
}

impl From<mdns::Event> for OutEvent {
    fn from(v: mdns::Event) -> Self {
        Self::Mdns(v)
    }
}

impl From<gossipsub::Event> for OutEvent {
    fn from(v: gossipsub::Event) -> Self {
        Self::Gossipsub(v)
    }
}

/// Represents a custom node in the Libp2p network.
pub struct Libp2pCustomNode {
    thread_handle: Option<task::JoinHandle<()>>,
    stop_notify: Arc<Notify>,
    outgoing_queue: Arc<deadqueue::unlimited::Queue<(gossipsub::IdentTopic, Vec<u8>)>>,
    new_subscribers_queue: Arc<deadqueue::unlimited::Queue<(
        gossipsub::IdentTopic,
        CustomSubscriptionHandle,
        unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),)
    >>,
    reactor: Runtime,
}


impl Libp2pCustomNode {
    fn create_swarm() -> libp2p::Swarm<RosNetworkBehaviour> {
        let keypair = identity::Keypair::generate_ed25519();

        let peer_id = PeerId::from(keypair.public());

        let transport = libp2p::tokio_development_transport(keypair.clone()).unwrap();

        let message_id_fn = |message: &gossipsub::Message| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            // same content will be propagated.
            .build()
            .expect("Valid config");

        let gossipsub: gossipsub::Behaviour = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(keypair),
            gossipsub_config,
        )
        .expect("Correct configuration");

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id).unwrap();

        let behaviour = RosNetworkBehaviour {
            gossipsub: gossipsub,
            mdns: mdns,
        };

        libp2p::Swarm::with_tokio_executor(transport, behaviour, peer_id)
    }

    fn new() -> Self {
        let reactor = Runtime::new().unwrap();
        let _guard = reactor.enter();

        let stop_notify = Arc::new(Notify::new());
        let outgoing_queue = Arc::new(deadqueue::unlimited::Queue::<(
            gossipsub::IdentTopic,
            Vec<u8>,
        )>::new());

        let mut swarm = Self::create_swarm();

        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .unwrap();

        let stop_notify_clone = Arc::clone(&stop_notify);
        let outgoing_queue_clone = Arc::clone(&outgoing_queue);
        let incoming_queue = Queue::<(
            String,
            unsafe extern "C" fn(CustomSubscriptionHandle, *mut u8, len: usize),
            Vec<u8>,
        )>::new();
        let new_subscribers_queue = Arc::new(deadqueue::unlimited::Queue::<(gossipsub::IdentTopic,
            CustomSubscriptionHandle,
            unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
        )>::new());
        let new_subscribers_queue_clone = Arc::clone(&new_subscribers_queue);
        let thread_handle = tokio::spawn(async move {
            let mut subscription_callback = HashMap::<String, (CustomSubscriptionHandle, unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize))>::new();
            loop {
                select! {
                    // use a Notify that will be triggered to stop the swarm
                    // select! will wait on any future
                    _ = stop_notify_clone.notified() => {
                        println!("Exit loop");
                        break;
                    },

                    (topic, obj, callback) = new_subscribers_queue_clone.pop() => {
                        println!("Subscribing to topic: {}", topic);
                        swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();
                        subscription_callback.insert(topic.hash().into_string(), (obj, callback));
                    },

                    // pop messages from the queue and publish them to the network
                    (topic, buffer) = outgoing_queue_clone.pop() => {
                        println!("Publishing message on topic {} : {:?}", topic, buffer);
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), buffer.clone()) {
                            println!("Publish error: {e:?}");
                        }
                    },

                    // // // pop messages from the queue and trigger the callback
                    // (topic, callback, buffer) = incoming_queue.pop() => {
                    //     let mut vec = buffer;
                    //     vec.shrink_to_fit();
                    //     let ptr: *mut u8 = vec.as_mut_ptr();
                    //     let len: usize = vec.len();
                    //     std::mem::forget(vec);
                    //     unsafe {
                    //         callback(obj, ptr, len);
                    //     }
                    // },

                    event = swarm.select_next_some() => match event {
                        SwarmEvent::Behaviour(OutEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        })) => {
                            println!(
                                "Got message: {:?} with id: {} from peer: {:?} topic: {}",
                                message.data,
                                id,
                                peer_id,
                                message.topic.as_str(),
                            );
                            // // incoming_queue.push((message.topic.into_string(), callback, message.data));
                            let mut vec = message.data;
                            vec.shrink_to_fit();
                            let ptr: *mut u8 = vec.as_mut_ptr();
                            let len: usize = vec.len();
                            std::mem::forget(vec);
                            let (obj, callback) = subscription_callback.get(&message.topic.into_string()).unwrap();
                            unsafe {
                                callback(&obj, ptr, len);
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(
                            mdns::Event::Discovered(list)
                        )) => {
                            for (peer, _) in list {
                                swarm
                                    .behaviour_mut()
                                    .gossipsub
                                    .add_explicit_peer(&peer);
                            }
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(mdns::Event::Expired(
                            list
                        ))) => {
                            for (peer, _) in list {
                                if !swarm.behaviour_mut().mdns.has_node(&peer) {
                                    swarm
                                        .behaviour_mut()
                                        .gossipsub
                                        .remove_explicit_peer(&peer);
                                }
                            }
                        },
                        _ => {
                            println!("UNKNOWN EVENT");
                        }
                    },
                }
            }
        });

        Self {
            thread_handle: Some(thread_handle),
            stop_notify: stop_notify,
            outgoing_queue: outgoing_queue,
            new_subscribers_queue: new_subscribers_queue,
            reactor: reactor,
        }
    }

    pub(crate) fn publish_message(&self, topic: gossipsub::IdentTopic, buffer: Vec<u8>) -> () {
        let mut out_buffer = Vec::<u8>::new();

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let secs = since_the_epoch.as_secs();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut out_buffer, &secs, cdr::Infinite).unwrap();

        let usecs = since_the_epoch.subsec_micros();
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(&mut out_buffer, &usecs, cdr::Infinite).unwrap();

        out_buffer.extend(buffer);
        self.outgoing_queue.push((topic, out_buffer));
    }

    pub(crate) fn notify_new_subscriber(&self, topic: gossipsub::IdentTopic,
        obj: CustomSubscriptionHandle,
        callback: unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
    ) -> () {
        self.new_subscribers_queue.push((topic, obj, callback));
    }
}

impl Drop for Libp2pCustomNode {
    fn drop(&mut self) {
        self.stop_notify.notify_waiters();
        self.reactor.block_on(async {
            if let Some(thread_handle) = self.thread_handle.take() {
                let _ = thread_handle.await;
            }
        });
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_new() -> *mut Libp2pCustomNode {
    Box::into_raw(Box::new(Libp2pCustomNode::new()))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_free(ptr: *mut Libp2pCustomNode) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}
