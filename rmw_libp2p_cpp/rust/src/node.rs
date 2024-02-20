use std::collections::hash_map::DefaultHasher;
use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libp2p::{futures::StreamExt, mdns, swarm::NetworkBehaviour};

use libp2p::{
    core::transport::ListenerId,
    gossipsub,
    gossipsub::{
        Gossipsub, GossipsubConfigBuilder, GossipsubEvent, GossipsubMessage, MessageAuthenticity,
        MessageId, Sha256Topic as Topic, TopicHash,
    },
    identity,
    identity::Keypair,
    mdns::{MdnsConfig, MdnsEvent, TokioMdns},
    swarm::behaviour::toggle::Toggle,
    swarm::{SwarmBuilder, SwarmEvent},
    Multiaddr, NetworkBehaviour, PeerId, Swarm,
};

use tokio::runtime::Runtime;
use tokio::sync::Notify;
use tokio::{select, task};

use deadqueue::unlimited::Queue;

#[repr(C)]
struct CustomNodeHandle(*mut c_void);

unsafe impl Send for CustomNodeHandle {}
unsafe impl Sync for CustomNodeHandle {}

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct RosNetworkBehaviour {
    pubsub: Gossipsub,
    mdns: Toggle<TokioMdns>,
}

#[derive(Debug)]
enum OutEvent {
    Gossipsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for OutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<GossipsubEvent> for OutEvent {
    fn from(v: GossipsubEvent) -> Self {
        Self::Gossipsub(v)
    }
}

pub struct Libp2pCustomNode {
    thread_handle: Option<task::JoinHandle<()>>,
    stop_notify: Arc<Notify>,
    outgoing_queue: Arc<deadqueue::unlimited::Queue<(gossipsub::IdentTopic, Vec<u8>)>>,
    reactor: Runtime,
}

impl Libp2pCustomNode {
    fn create_swarm() -> libp2p::Swarm<RosNetworkBehaviour> {
        let keypair = identity::Keypair::generate_ed25519();

        let peer_id = PeerId::from(keypair.public());

        let transport = libp2p::tokio_development_transport(keypair.clone()).unwrap();

        let message_id_fn = |message: &GossipsubMessage| {
            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            gossipsub::MessageId::from(s.finish().to_string())
        };

        let gossipsub_config = GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(message_id_fn)
            // same content will be propagated.
            .build()
            .expect("Valid config");

        let gossipsub: Gossipsub::new(
            MessageAuthenticity::Signed(keypair),
            gossipsub_config,
        );
        // .expect("Correct configuration");

        // Build mDNS network behaviour
        let mdns = if !disable_mdns {
            let mdns = TokioMdns::new(MdnsConfig::default())?;
            Toggle::from(Some(mdns))
        } else {
            Toggle::from(None)
        };

        let behaviour = RosNetworkBehaviour {
            pubsub: gossipsub,
            mdns: mdns,
        };

        libp2p::Swarm::with_tokio_executor(transport, behaviour, peer_id)
    }

    fn new(
        obj: CustomNodeHandle,
        callback: unsafe extern "C" fn(&CustomNodeHandle, *mut u8, usize),
    ) -> Self {
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
            unsafe extern "C" fn(CustomNodeHandle, *mut u8, len: usize),
            Vec<u8>,
        )>::new();
        let new_subscribers_queue = Queue::<String>::new();

        let thread_handle = tokio::spawn(async move {
            loop {
                select! {
                    // use a Notify that will be triggered to stop the swarm
                    // select! will wait on any future
                    _ = stop_notify_clone.notified() => {
                        println!("Exit loop");
                        break;
                    },

                    topic = new_subscribers_queue.pop() => {
                        println!("Subscribing to topic: {}", topic);
                        let topic = gossipsub::IdentTopic::new(&topic);
                        swarm.behaviour_mut().pubsub.subscribe(&topic).unwrap();
                    },

                    // pop messages from the queue and publish them to the network
                    (topic, buffer) = outgoing_queue_clone.pop() => {
                        println!("Publishing message on topic {} : {:?}", topic, buffer);
                        if let Err(e) = swarm.behaviour_mut().pubsub.publish(topic.clone(), buffer.clone()) {
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
                        SwarmEvent::Behaviour(OutEvent::Gossipsub(GossipsubEvent::Message {
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
                            // incoming_queue.push((message.topic.into_string(), callback, message.data));
                            let mut vec = message.data;
                            vec.shrink_to_fit();
                            let ptr: *mut u8 = vec.as_mut_ptr();
                            let len: usize = vec.len();
                            std::mem::forget(vec);
                            unsafe {
                                callback(&obj, ptr, len);
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(
                            MdnsEvent::Discovered(list)
                        )) => {
                            for (peer, _) in list {
                                swarm
                                    .behaviour_mut()
                                    .pubsub
                                    .add_explicit_peer(&peer);
                            }
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
                            list
                        ))) => {
                            for (peer, _) in list {
                                if let Some(mdns) = swarm.behaviour_mut().mdns.as_mut() {
                                    if !mdns.has_node(&peer) {
                                        swarm
                                            .behaviour_mut()
                                            .pubsub
                                            .remove_explicit_peer(&peer);
                                    }
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
pub extern "C" fn rs_libp2p_custom_node_new(
    obj: CustomNodeHandle,
    callback: unsafe extern "C" fn(&CustomNodeHandle, *mut u8, len: usize),
) -> *mut Libp2pCustomNode {
    Box::into_raw(Box::new(Libp2pCustomNode::new(obj, callback)))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_free(ptr: *mut Libp2pCustomNode) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}
