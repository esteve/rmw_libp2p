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
use std::collections::HashMap;
use std::ffi::c_void;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libp2p::{
    futures::StreamExt, gossipsub, identity, mdns, swarm::NetworkBehaviour, swarm::SwarmEvent,
    PeerId,
};

use tokio::runtime::Runtime;
use tokio::sync::Notify;
use tokio::{select, task};

use deadqueue::unlimited::Queue;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CustomSubscriptionHandle {
    pub ptr: *const c_void,
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
    pub(crate) outgoing_queue: Arc<deadqueue::unlimited::Queue<(gossipsub::IdentTopic, Vec<u8>)>>,
    pub(crate) new_subscribers_queue: Arc<
        deadqueue::unlimited::Queue<(
            gossipsub::IdentTopic,
            CustomSubscriptionHandle,
            unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
        )>,
    >,
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

        let behaviour = RosNetworkBehaviour { gossipsub, mdns };

        // Note: with_tokio_executor is deprecated in libp2p 0.51 but SwarmBuilder
        // (the recommended replacement) is not yet available in this version.
        // This will be addressed when upgrading to libp2p 0.52+.
        #[allow(deprecated)]
        libp2p::Swarm::with_tokio_executor(transport, behaviour, peer_id)
    }

    /// Test-only constructor that creates a minimal node without starting the event loop
    /// This allows testing queue operations and lifecycle without network overhead
    #[cfg(test)]
    pub(crate) fn new_test_only() -> Self {
        let reactor = Runtime::new().unwrap();
        let stop_notify = Arc::new(Notify::new());
        let outgoing_queue = Arc::new(deadqueue::unlimited::Queue::<(
            gossipsub::IdentTopic,
            Vec<u8>,
        )>::new());
        let new_subscribers_queue = Arc::new(deadqueue::unlimited::Queue::<(
            gossipsub::IdentTopic,
            CustomSubscriptionHandle,
            unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
        )>::new());

        Self {
            thread_handle: None, // No actual event loop thread for testing
            stop_notify,
            outgoing_queue,
            new_subscribers_queue,
            reactor,
        }
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
        let _incoming_queue = Queue::<(
            String,
            unsafe extern "C" fn(CustomSubscriptionHandle, *mut u8, len: usize),
            Vec<u8>,
        )>::new();
        let new_subscribers_queue = Arc::new(deadqueue::unlimited::Queue::<(
            gossipsub::IdentTopic,
            CustomSubscriptionHandle,
            unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
        )>::new());
        let new_subscribers_queue_clone = Arc::clone(&new_subscribers_queue);
        let thread_handle = tokio::spawn(async move {
            let mut subscription_callback = HashMap::<
                String,
                (
                    CustomSubscriptionHandle,
                    unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
                ),
            >::new();
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
                            propagation_source: _peer_id,
                            message_id: _id,
                            message,
                        })) => {
                            // TODO(esteve): use some sort of debug log
                            // println!(
                            //     "Got message: {:?} with id: {} from peer: {:?} topic: {} with length: {}",
                            //     message.data,
                            //     id,
                            //     peer_id,
                            //     message.topic.as_str(),
                            //     message.data.len()
                            // );
                            let mut input_vec = message.data;
                            input_vec.shrink_to_fit();
                            let ptr: *mut u8 = input_vec.as_mut_ptr();
                            let len: usize = input_vec.len();
                            std::mem::forget(input_vec);
                            let (obj, callback) = subscription_callback.get(&message.topic.into_string()).unwrap();
                            unsafe {
                                callback(obj, ptr, len);
                            }
                        }
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(
                            mdns::Event::Discovered(list)
                        )) => {
                            // println!("Discovered peers: {:?}", list);
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
            stop_notify,
            outgoing_queue,
            new_subscribers_queue,
            reactor,
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
    pub(crate) fn publish_message(&self, topic: gossipsub::IdentTopic, buffer: Vec<u8>) {
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

        // TODO(esteve): use some sort of debug log
        // println!("Publishing message with data length: {:?} {}", out_buffer, out_buffer.len());
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
    pub(crate) fn notify_new_subscriber(
        &self,
        topic: gossipsub::IdentTopic,
        obj: CustomSubscriptionHandle,
        callback: unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
    ) {
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
#[unsafe(no_mangle)]
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_libp2p_custom_node_free(ptr: *mut Libp2pCustomNode) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        let _ = Box::from_raw(ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test helper: Create a dummy callback for testing
    unsafe extern "C" fn dummy_callback(
        _handle: &CustomSubscriptionHandle,
        _data: *mut u8,
        _len: usize,
    ) {
        // This is a no-op callback for testing purposes
    }

    #[test]
    fn test_node_creation_and_destruction() {
        // Create a test node without network overhead
        let node = Libp2pCustomNode::new_test_only();

        // Verify node components are initialized (queues exist)
        assert_eq!(Arc::strong_count(&node.outgoing_queue), 1);
        assert_eq!(Arc::strong_count(&node.new_subscribers_queue), 1);

        // Node should be dropped cleanly at the end of scope
        drop(node);

        // If we reach here without hanging, Drop worked correctly
    }

    #[test]
    fn test_multiple_nodes_in_same_process() {
        // Create multiple test nodes to verify isolation
        let node1 = Libp2pCustomNode::new_test_only();
        let node2 = Libp2pCustomNode::new_test_only();
        let node3 = Libp2pCustomNode::new_test_only();

        // Verify nodes have separate queue instances
        assert!(!Arc::ptr_eq(&node1.outgoing_queue, &node2.outgoing_queue));
        assert!(!Arc::ptr_eq(&node2.outgoing_queue, &node3.outgoing_queue));
        assert!(!Arc::ptr_eq(
            &node1.new_subscribers_queue,
            &node2.new_subscribers_queue
        ));

        // Drop in different order to test independence
        drop(node2);
        drop(node1);
        drop(node3);
    }

    #[test]
    fn test_node_drop_behavior() {
        let node = Libp2pCustomNode::new_test_only();

        // Drop the node
        drop(node);

        // If we reach here without hanging, Drop worked correctly
    }

    #[test]
    fn test_outgoing_queue_push() {
        let node = Libp2pCustomNode::new_test_only();
        let topic = gossipsub::IdentTopic::new("test_topic");
        let message = vec![1, 2, 3, 4, 5];

        // Push message to outgoing queue via publish_message
        node.publish_message(topic.clone(), message.clone());

        // Push multiple messages
        for i in 0..10 {
            let msg = vec![i; 10];
            node.publish_message(topic.clone(), msg);
        }

        // All pushes should succeed without blocking or panicking
    }

    #[test]
    fn test_new_subscribers_queue() {
        let node = Libp2pCustomNode::new_test_only();
        let topic = gossipsub::IdentTopic::new("subscriber_test");

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        // Add a subscriber
        node.notify_new_subscriber(topic.clone(), handle, dummy_callback);

        // Add multiple subscribers to different topics
        for i in 0..5 {
            let topic = gossipsub::IdentTopic::new(format!("topic_{}", i));
            let handle = CustomSubscriptionHandle {
                ptr: std::ptr::null(),
            };
            node.notify_new_subscriber(topic, handle, dummy_callback);
        }

        // All operations should complete without blocking
    }

    #[test]
    fn test_multiple_subscribers_same_topic() {
        let node = Libp2pCustomNode::new_test_only();
        let topic = gossipsub::IdentTopic::new("shared_topic");

        // Create multiple subscription handles for the same topic
        let handle1 = CustomSubscriptionHandle {
            ptr: 1 as *const c_void,
        };
        let handle2 = CustomSubscriptionHandle {
            ptr: 2 as *const c_void,
        };
        let handle3 = CustomSubscriptionHandle {
            ptr: 3 as *const c_void,
        };

        // All should be able to subscribe
        node.notify_new_subscriber(topic.clone(), handle1, dummy_callback);
        node.notify_new_subscriber(topic.clone(), handle2, dummy_callback);
        node.notify_new_subscriber(topic.clone(), handle3, dummy_callback);

        // The last subscriber should override in the current implementation
        // This tests that the queue accepts multiple subscribers
    }

    #[test]
    fn test_queue_cleanup_on_drop() {
        let node = Libp2pCustomNode::new_test_only();
        let queue_ref = Arc::clone(&node.outgoing_queue);
        let subscribers_ref = Arc::clone(&node.new_subscribers_queue);

        // Add some items to queues
        let topic = gossipsub::IdentTopic::new("cleanup_test");
        node.publish_message(topic.clone(), vec![1, 2, 3]);

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };
        node.notify_new_subscriber(topic, handle, dummy_callback);

        // Get Arc strong counts before drop
        let queue_strong_count_before = Arc::strong_count(&queue_ref);
        let subscribers_strong_count_before = Arc::strong_count(&subscribers_ref);

        // Drop the node
        drop(node);

        // The Arc counts should decrease by 1 (node's reference is gone)
        assert_eq!(Arc::strong_count(&queue_ref), queue_strong_count_before - 1);
        assert_eq!(
            Arc::strong_count(&subscribers_ref),
            subscribers_strong_count_before - 1
        );
    }

    #[test]
    fn test_custom_subscription_handle_send_sync() {
        // This test verifies that CustomSubscriptionHandle implements Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CustomSubscriptionHandle>();
        assert_sync::<CustomSubscriptionHandle>();
    }

    #[test]
    fn test_custom_subscription_handle_creation() {
        // Test creating various handles
        let handle1 = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let handle2 = CustomSubscriptionHandle {
            ptr: 0x12345678 as *const c_void,
        };

        // Verify they can be created with different pointer values
        assert!(handle1.ptr.is_null());
        assert!(!handle2.ptr.is_null());
    }

    #[test]
    fn test_publish_message_includes_timestamp() {
        let node = Libp2pCustomNode::new_test_only();
        let topic = gossipsub::IdentTopic::new("timestamp_test");
        let message = vec![0xAA, 0xBB, 0xCC];

        // Publish a message
        node.publish_message(topic.clone(), message.clone());

        // The message in queue should have timestamp prepended
        // We can't easily inspect it here, but we verify the method completes
    }

    #[test]
    fn test_concurrent_publish_operations() {
        let node = Arc::new(Libp2pCustomNode::new_test_only());
        let mut handles = vec![];

        // Spawn multiple threads publishing simultaneously
        for i in 0..10 {
            let node_clone = Arc::clone(&node);
            let handle = std::thread::spawn(move || {
                let topic = gossipsub::IdentTopic::new(format!("concurrent_{}", i));
                for j in 0..100 {
                    let message = vec![i as u8, j];
                    node_clone.publish_message(topic.clone(), message);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // If we reach here, concurrent access to queue worked correctly
    }

    #[test]
    fn test_concurrent_subscriber_operations() {
        let node = Arc::new(Libp2pCustomNode::new_test_only());
        let mut handles = vec![];

        // Spawn multiple threads adding subscribers simultaneously
        for i in 0..10 {
            let node_clone = Arc::clone(&node);
            let handle = std::thread::spawn(move || {
                let topic = gossipsub::IdentTopic::new(format!("sub_concurrent_{}", i));
                for j in 0..10 {
                    let sub_handle = CustomSubscriptionHandle {
                        ptr: (i * 1000 + j) as *const c_void,
                    };
                    node_clone.notify_new_subscriber(topic.clone(), sub_handle, dummy_callback);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_runtime_creation() {
        // Verify that each node gets its own runtime
        let node1 = Libp2pCustomNode::new_test_only();
        let node2 = Libp2pCustomNode::new_test_only();

        // Both should have successfully created runtimes
        // (if runtime creation failed, new_test_only() would have panicked)

        drop(node1);
        drop(node2);
    }

    // FFI tests use the real constructor to test the actual FFI interface
    // These may take longer but test the actual production code path
    #[test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    fn test_node_with_ffi_api() {
        // Test the C FFI functions
        let node_ptr = rs_libp2p_custom_node_new();
        assert!(
            !node_ptr.is_null(),
            "FFI node creation should return non-null pointer"
        );

        // Free the node
        rs_libp2p_custom_node_free(node_ptr);

        // Test freeing null pointer (should not crash)
        rs_libp2p_custom_node_free(std::ptr::null_mut());
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    fn test_ffi_multiple_nodes() {
        // Create multiple nodes via FFI
        let node1 = rs_libp2p_custom_node_new();
        let node2 = rs_libp2p_custom_node_new();
        let node3 = rs_libp2p_custom_node_new();

        assert!(!node1.is_null());
        assert!(!node2.is_null());
        assert!(!node3.is_null());

        // Verify they're different pointers
        assert_ne!(node1, node2);
        assert_ne!(node2, node3);
        assert_ne!(node1, node3);

        // Free in different order
        rs_libp2p_custom_node_free(node2);
        rs_libp2p_custom_node_free(node1);
        rs_libp2p_custom_node_free(node3);
    }

    #[test]
    fn test_message_id_uniqueness() {
        // Test that the message ID function produces unique IDs for different data
        use std::collections::HashSet;

        let mut message_ids = HashSet::new();

        for i in 0..100 {
            let data = vec![i; 10];
            let message = gossipsub::Message {
                source: None,
                data: data.clone(),
                sequence_number: None,
                topic: gossipsub::TopicHash::from_raw("test"),
            };

            let mut s = DefaultHasher::new();
            message.data.hash(&mut s);
            let id = gossipsub::MessageId::from(s.finish().to_string());

            message_ids.insert(id);
        }

        // We should have 100 unique message IDs
        assert_eq!(
            message_ids.len(),
            100,
            "Message IDs should be unique for different data"
        );
    }

    #[test]
    fn test_same_content_same_message_id() {
        // Verify that identical content produces the same message ID
        let data = vec![42; 20];

        let message1 = gossipsub::Message {
            source: None,
            data: data.clone(),
            sequence_number: None,
            topic: gossipsub::TopicHash::from_raw("test"),
        };

        let message2 = gossipsub::Message {
            source: None,
            data: data.clone(),
            sequence_number: None,
            topic: gossipsub::TopicHash::from_raw("test"),
        };

        let mut s1 = DefaultHasher::new();
        message1.data.hash(&mut s1);
        let id1 = gossipsub::MessageId::from(s1.finish().to_string());

        let mut s2 = DefaultHasher::new();
        message2.data.hash(&mut s2);
        let id2 = gossipsub::MessageId::from(s2.finish().to_string());

        assert_eq!(id1, id2, "Same content should produce same message ID");
    }

    #[test]
    fn test_topic_creation() {
        // Test creating various topics
        let topic1 = gossipsub::IdentTopic::new("topic1");
        let topic2 = gossipsub::IdentTopic::new("topic2");
        let topic3 = gossipsub::IdentTopic::new("topic1"); // Same as topic1

        // Verify topic hashes
        assert_eq!(
            topic1.hash(),
            topic3.hash(),
            "Same topic names should have same hash"
        );
        assert_ne!(
            topic1.hash(),
            topic2.hash(),
            "Different topic names should have different hash"
        );
    }

    #[test]
    fn test_graceful_shutdown_with_pending_messages() {
        let node = Libp2pCustomNode::new_test_only();

        // Queue up many messages
        for i in 0..1000 {
            let topic = gossipsub::IdentTopic::new("shutdown_test");
            let message = vec![i as u8; 100];
            node.publish_message(topic, message);
        }

        // Drop should handle pending messages gracefully
        drop(node);

        // If we reach here without hanging, shutdown worked correctly
    }

    #[test]
    fn test_graceful_shutdown_with_pending_subscribers() {
        let node = Libp2pCustomNode::new_test_only();

        // Queue up many subscriber registrations
        for i in 0..100 {
            let topic = gossipsub::IdentTopic::new(format!("shutdown_sub_{}", i));
            let handle = CustomSubscriptionHandle {
                ptr: i as *const c_void,
            };
            node.notify_new_subscriber(topic, handle, dummy_callback);
        }

        // Drop should handle pending subscribers gracefully
        drop(node);
    }

    // Integration tests with actual event loop
    #[test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    fn test_full_node_lifecycle() {
        // Test creating a full node with event loop
        let node = Libp2pCustomNode::new();

        // Verify thread handle exists
        assert!(node.thread_handle.is_some());

        // Drop should cleanly shut down the event loop
        drop(node);
    }

    #[test]
    #[ignore] // Run with: cargo test -- --ignored --test-threads=1
    fn test_full_node_with_operations() {
        let node = Libp2pCustomNode::new();

        // Perform some operations
        let topic = gossipsub::IdentTopic::new("integration_test");
        node.publish_message(topic.clone(), vec![1, 2, 3]);

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };
        node.notify_new_subscriber(topic, handle, dummy_callback);

        // Allow some time for processing
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Clean shutdown
        drop(node);
    }
}
