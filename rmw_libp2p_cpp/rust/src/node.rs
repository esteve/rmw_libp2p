// Copyright 2024 Esteve Fernandez
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

/// This module contains the implementation of a custom node in the Libp2p network.
/// The `Libp2pCustomNode` struct represents a custom node and provides methods for creating and interacting with the node.
/// The node uses the `RosNetworkBehaviour` struct as its network behavior, which combines the `gossipsub` and `mdns` behaviors.
/// The node can publish messages to the network, subscribe to topics, and handle incoming messages.
/// The node runs in its own thread and uses a `Swarm` instance to manage the network behavior.
/// The node also uses a `Queue` to store outgoing messages and a `HashMap` to store subscription callbacks.
/// The `Libp2pCustomNode` struct provides methods for creating a new node, publishing messages, and stopping the node.
/// The node is designed to be used in a multithreaded environment and provides thread-safe access to its internal data structures.
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

/// Creates a new instance of the `Libp2pCustomNode`.
/// This method initializes the necessary components for the node, including the network behavior, transport, and swarm.
/// It also starts the node's thread and listens on a random TCP port.
/// Returns the created `Libp2pCustomNode` instance.
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

    /// Creates a new instance of the struct.
    ///
    /// This function initializes a new runtime, creates a new swarm, and sets up various queues and callbacks for handling network events.
    /// It also spawns a new Tokio task that runs an event loop for handling these events.
    ///
    /// # Returns
    ///
    /// A new instance of the struct.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to create a new runtime or if it fails to make the swarm listen on the specified address.
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
                        // println!("Subscribing to topic: {}", topic);
                        swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();
                        subscription_callback.insert(topic.hash().into_string(), (obj, callback));
                    },

                    // pop messages from the queue and publish them to the network
                    (topic, buffer) = outgoing_queue_clone.pop() => {
                        // TODO(esteve): use some sort of debug log
                        // println!("Publishing message on topic {} : {:?}", topic, buffer);
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), buffer.clone()) {
                            println!("Publish error: {e:?}");
                        }
                    },

                    event = swarm.select_next_some() => match event {
                        SwarmEvent::Behaviour(OutEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        })) => {
                            // TODO(esteve): use some sort of debug log
                            // println!(
                            //     "Got message: {:?} with id: {} from peer: {:?} topic: {}",
                            //     message.data,
                            //     id,
                            //     peer_id,
                            //     message.topic.as_str(),
                            // );
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
                            println!("Discovered peers: {:?}", list);
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
                            // TODO(esteve): use some sort of debug log
                            // println!("UNKNOWN EVENT");
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

    /// Publishes a message to a specific topic.
    ///
    /// This function serializes the current system time and a provided buffer into a new buffer,
    /// then pushes the new buffer and the topic into the outgoing queue.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic to publish the message to.
    /// * `buffer` - The message to publish.
    ///
    /// # Panics
    ///
    /// This function will panic if the system time is before the UNIX_EPOCH.
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

    /// Notifies about a new subscriber to a specific topic.
    ///
    /// This function pushes the topic, a `CustomSubscriptionHandle`, and a callback function into the `new_subscribers_queue`.
    ///
    /// # Arguments
    ///
    /// * `topic` - The topic the new subscriber is interested in.
    /// * `obj` - A `CustomSubscriptionHandle` associated with the new subscriber.
    /// * `callback` - A callback function to be called when a new message is published to the topic.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it uses a raw pointer in the callback function.
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

/// Creates a new `Libp2pCustomNode`.
///
/// This function allocates a `Libp2pCustomNode` on the heap, then returns a raw pointer to the heap-allocated object.
///
/// # Safety
///
/// This function is unsafe because it returns a raw pointer to a heap-allocated object. The caller is responsible for freeing this memory.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomNode`.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_new() -> *mut Libp2pCustomNode {
    Box::into_raw(Box::new(Libp2pCustomNode::new()))
}

/// Frees a `Libp2pCustomNode` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomNode`, converts it back into a `Box`, and then drops the `Box`, freeing the memory.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomNode`.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_free(ptr: *mut Libp2pCustomNode) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}
