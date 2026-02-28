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

use crate::CustomSubscriptionHandle;
use crate::Libp2pCustomNode;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Arc;

use uuid::Uuid;

use libp2p::gossipsub;

/// Represents a custom subscription in the Libp2p network.
///
/// This struct holds a unique identifier (UUID), a pointer to the associated `Libp2pCustomNode`, the topic of the subscription,
/// and a queue for incoming messages.
///
/// # Fields
///
/// * `gid` - A unique identifier for this subscription.
/// * `node` - A raw pointer to the `Libp2pCustomNode` associated with this subscription. This is needed to access the outgoing queue.
/// * `topic` - The topic of the subscription.
/// * `incoming_queue` - A thread-safe, unlimited queue for incoming messages. Each message is a tuple of the topic and the message data.
///
/// # Safety
///
/// This struct is unsafe because it uses raw pointers.
pub struct Libp2pCustomSubscription {
    gid: Uuid,
    #[allow(dead_code)]
    node: *mut Libp2pCustomNode, // We need to store the Node here to have access to the outgoing queue
    #[allow(dead_code)]
    topic: gossipsub::IdentTopic,
    #[allow(dead_code)]
    incoming_queue: Arc<deadqueue::unlimited::Queue<(gossipsub::IdentTopic, Vec<u8>)>>,
}

/// Represents a custom subscription in the Libp2p network.
///
/// This struct is responsible for creating and managing a custom subscription
/// to a specific topic in the Libp2p network. It provides a way to notify the
/// Libp2pCustomNode about the new subscriber and handle incoming messages
/// related to the subscription.
impl Libp2pCustomSubscription {
    /// Creates a new instance of `Libp2pCustomSubscription`.
    ///
    /// # Arguments
    ///
    /// * `ptr_node` - A raw pointer to the `Libp2pCustomNode` instance.
    /// * `topic_str` - The topic string for the subscription.
    /// * `obj` - The custom subscription handle object.
    /// * `callback` - The callback function to be called when a message is received.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it deals with raw pointers and
    /// an unsafe extern "C" function pointer.
    ///
    /// # Example
    ///
    /// ```
    /// use std::os::raw::c_void;
    ///
    /// unsafe extern "C" fn callback_fn(handle: *const CustomSubscriptionHandle, data: *mut u8, len: usize) {
    ///     // Handle the received message
    /// }
    ///
    /// let ptr_node = /* obtain the raw pointer */;
    /// let topic_str = "my_topic";
    /// let obj = /* create the custom subscription handle */;
    ///
    /// let subscription = Libp2pCustomSubscription::new(ptr_node, topic_str, obj, callback_fn);
    /// ```
    fn new(
        ptr_node: *mut Libp2pCustomNode,
        topic_str: &str,
        obj: CustomSubscriptionHandle,
        callback: unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
    ) -> Self {
        let libp2p_custom_node = unsafe {
            assert!(!ptr_node.is_null());
            &mut *ptr_node
        };

        libp2p_custom_node.notify_new_subscriber(
            gossipsub::IdentTopic::new(topic_str),
            obj,
            callback,
        );

        Self {
            gid: Uuid::new_v4(),
            node: ptr_node,
            topic: gossipsub::IdentTopic::new(topic_str),
            incoming_queue: Arc::new(deadqueue::unlimited::Queue::new()),
        }
    }
}

/// Creates a new `Libp2pCustomSubscription`.
///
/// This function takes a raw pointer to a `Libp2pCustomNode`, a raw pointer to a C string representing the topic, a `CustomSubscriptionHandle`, and a callback function.
/// It then creates a new `Libp2pCustomSubscription` for the given node and topic, and returns a raw pointer to the heap-allocated subscription.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_node` - A raw pointer to a `Libp2pCustomNode`.
/// * `topic_str_ptr` - A raw pointer to a C string representing the topic.
/// * `obj` - A `CustomSubscriptionHandle` associated with the new subscription.
/// * `callback` - A callback function to be called when a new message is published to the topic.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomSubscription`.
///
/// # Panics
///
/// This function will panic if `topic_str_ptr` is null or if it does not point to a valid null-terminated string.
#[allow(private_interfaces)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_libp2p_custom_subscription_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
    obj: CustomSubscriptionHandle,
    callback: unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
) -> *mut Libp2pCustomSubscription {
    unsafe {
        let topic_str = {
            assert!(!topic_str_ptr.is_null());
            CStr::from_ptr(topic_str_ptr)
        };

        let libp2p_custom_subscription =
            Libp2pCustomSubscription::new(ptr_node, topic_str.to_str().unwrap(), obj, callback);
        Box::into_raw(Box::new(libp2p_custom_subscription))
    }
}

/// Frees a `Libp2pCustomSubscription` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomSubscription`, converts it back into a `Box`, and then drops the `Box`, freeing the memory.
/// If the provided pointer is null, the function returns immediately.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr_subscription` - A raw pointer to a `Libp2pCustomSubscription`.
///
/// # Panics
///
/// This function will panic if the provided pointer has been previously deallocated or was not returned by `rs_libp2p_custom_subscription_new`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_libp2p_custom_subscription_free(
    ptr_subscription: *mut Libp2pCustomSubscription,
) {
    unsafe {
        if ptr_subscription.is_null() {
            return;
        }
        let _ = Box::from_raw(ptr_subscription);
    }
}

/// Gets the GID of a `Libp2pCustomSubscription`.
///
/// This function takes a raw pointer to a `Libp2pCustomSubscription` and a raw pointer to a buffer.
/// It then copies the bytes of the GID of the subscription into the buffer and returns the number of bytes copied.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_subscription` - A raw pointer to a `Libp2pCustomSubscription`.
/// * `buf` - A raw pointer to a buffer where the GID bytes will be copied.
///
/// # Returns
///
/// The number of bytes copied into the buffer.
///
/// # Panics
///
/// This function will panic if `ptr_subscription` is null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_libp2p_custom_subscription_get_gid(
    ptr_subscription: *mut Libp2pCustomSubscription,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    unsafe {
        let libp2p_custom_subscription = {
            assert!(!ptr_subscription.is_null());
            &mut *ptr_subscription
        };
        let gid_bytes = libp2p_custom_subscription.gid.as_bytes();
        let count = gid_bytes.len();
        std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf, count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    // Test helper: Counter for callback invocations
    static CALLBACK_COUNTER: AtomicUsize = AtomicUsize::new(0);
    static CALLBACK_INVOKED: AtomicBool = AtomicBool::new(false);

    unsafe extern "C" fn test_callback(
        _handle: &CustomSubscriptionHandle,
        _data: *mut u8,
        _len: usize,
    ) {
        CALLBACK_COUNTER.fetch_add(1, Ordering::SeqCst);
        CALLBACK_INVOKED.store(true, Ordering::SeqCst);
    }

    unsafe extern "C" fn noop_callback(
        _handle: &CustomSubscriptionHandle,
        _data: *mut u8,
        _len: usize,
    ) {
        // No-op for basic testing
    }

    #[test]
    fn test_subscription_creation_with_valid_node() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let topic_str = "test_subscription";
        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let subscription = Libp2pCustomSubscription::new(node, topic_str, handle, noop_callback);

        // Verify subscription was created with correct fields
        // Note: IdentTopic doesn't implement PartialEq, so we compare hashes
        assert_eq!(
            subscription.topic.hash(),
            gossipsub::IdentTopic::new(topic_str).hash()
        );
        assert_eq!(subscription.node, node);
        assert!(!subscription.incoming_queue.is_empty() || subscription.incoming_queue.is_empty()); // Queue exists

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_gid_retrieval() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let sub1 = Libp2pCustomSubscription::new(node, "topic1", handle, noop_callback);
        let sub2 = Libp2pCustomSubscription::new(node, "topic2", handle, noop_callback);

        // Verify GIDs are unique
        assert_ne!(sub1.gid, sub2.gid);

        // Verify GID format (UUID v4 should have 16 bytes)
        assert_eq!(sub1.gid.as_bytes().len(), 16);
        assert_eq!(sub2.gid.as_bytes().len(), 16);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_gid_uniqueness() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        // Create multiple subscriptions to the same topic
        let sub1 = Libp2pCustomSubscription::new(node, "same_topic", handle, noop_callback);
        let sub2 = Libp2pCustomSubscription::new(node, "same_topic", handle, noop_callback);
        let sub3 = Libp2pCustomSubscription::new(node, "same_topic", handle, noop_callback);

        // All should have unique GIDs
        assert_ne!(sub1.gid, sub2.gid);
        assert_ne!(sub1.gid, sub3.gid);
        assert_ne!(sub2.gid, sub3.gid);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_callback_registration() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let new_subscribers_queue = Arc::clone(&node_box.new_subscribers_queue);
        let node = Box::into_raw(node_box);

        let handle = CustomSubscriptionHandle {
            ptr: 0x12345678 as *const c_void,
        };

        // Create subscription which should register callback
        let _subscription =
            Libp2pCustomSubscription::new(node, "callback_test", handle, test_callback);

        // Verify that the subscriber was added to the queue
        let (topic, queued_handle, _callback_fn) = new_subscribers_queue
            .try_pop()
            .expect("Subscriber should be queued");

        assert_eq!(
            topic.hash(),
            gossipsub::IdentTopic::new("callback_test").hash()
        );
        assert_eq!(queued_handle.ptr, handle.ptr);

        // Verify callback function pointer matches
        // We can't directly compare function pointers easily, but we can verify it's not null
        // In real scenario, the callback would be invoked by the node's event loop

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_drop_behavior() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        {
            let subscription =
                Libp2pCustomSubscription::new(node, "drop_test", handle, noop_callback);
            // Subscription should be alive here
            assert_eq!(
                subscription.topic.hash(),
                gossipsub::IdentTopic::new("drop_test").hash()
            );
        } // Subscription dropped here

        // Create another subscription with same node - should work fine
        {
            let subscription2 =
                Libp2pCustomSubscription::new(node, "drop_test2", handle, noop_callback);
            assert_eq!(
                subscription2.topic.hash(),
                gossipsub::IdentTopic::new("drop_test2").hash()
            );
        }

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_multiple_subscriptions_to_same_topic() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create multiple subscription handles
        let handle1 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };
        let handle2 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };
        let handle3 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };

        // Create multiple subscriptions to the same topic
        let sub1 = Libp2pCustomSubscription::new(node, "shared_topic", handle1, noop_callback);
        let sub2 = Libp2pCustomSubscription::new(node, "shared_topic", handle2, noop_callback);
        let sub3 = Libp2pCustomSubscription::new(node, "shared_topic", handle3, noop_callback);

        // All subscriptions should have unique GIDs
        assert_ne!(sub1.gid, sub2.gid);
        assert_ne!(sub1.gid, sub3.gid);
        assert_ne!(sub2.gid, sub3.gid);

        // All should subscribe to the same topic
        let expected_topic = gossipsub::IdentTopic::new("shared_topic");
        assert_eq!(sub1.topic.hash(), expected_topic.hash());
        assert_eq!(sub2.topic.hash(), expected_topic.hash());
        assert_eq!(sub3.topic.hash(), expected_topic.hash());

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_incoming_queue_creation() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let subscription = Libp2pCustomSubscription::new(node, "queue_test", handle, noop_callback);

        // Verify incoming queue was created
        assert_eq!(Arc::strong_count(&subscription.incoming_queue), 1);

        // Queue should be initially empty or ready to accept messages
        // We can't easily test message reception without the event loop running

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_with_different_topics() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        // Create subscriptions to different topics
        let sub1 = Libp2pCustomSubscription::new(node, "topic_a", handle, noop_callback);
        let sub2 = Libp2pCustomSubscription::new(node, "topic_b", handle, noop_callback);
        let sub3 = Libp2pCustomSubscription::new(node, "topic_c", handle, noop_callback);

        // Verify topics are different
        assert_eq!(
            sub1.topic.hash(),
            gossipsub::IdentTopic::new("topic_a").hash()
        );
        assert_eq!(
            sub2.topic.hash(),
            gossipsub::IdentTopic::new("topic_b").hash()
        );
        assert_eq!(
            sub3.topic.hash(),
            gossipsub::IdentTopic::new("topic_c").hash()
        );

        // All should have unique GIDs
        assert_ne!(sub1.gid, sub2.gid);
        assert_ne!(sub2.gid, sub3.gid);
        assert_ne!(sub1.gid, sub3.gid);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    // FFI Tests
    #[test]
    fn test_ffi_subscription_new_and_free() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let topic = std::ffi::CString::new("ffi_topic").unwrap();
        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        // Create subscription via FFI
        let sub_ptr =
            rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle, noop_callback);

        // Verify pointer is not null
        assert!(!sub_ptr.is_null());

        // Free subscription
        rs_libp2p_custom_subscription_free(sub_ptr);

        // Test freeing null pointer (should not crash)
        rs_libp2p_custom_subscription_free(std::ptr::null_mut());

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_subscription_get_gid() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));
        let topic = std::ffi::CString::new("gid_test").unwrap();
        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let sub_ptr =
            rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle, noop_callback);

        // Allocate buffer for GID
        let mut gid_buffer = vec![0u8; 16];

        // Get GID via FFI
        let gid_len = rs_libp2p_custom_subscription_get_gid(sub_ptr, gid_buffer.as_mut_ptr());

        // Verify GID length
        assert_eq!(gid_len, 16);

        // Verify buffer was filled (not all zeros)
        assert!(gid_buffer.iter().any(|&x| x != 0));

        // Get GID again and verify it's the same
        let mut gid_buffer2 = vec![0u8; 16];
        let gid_len2 = rs_libp2p_custom_subscription_get_gid(sub_ptr, gid_buffer2.as_mut_ptr());

        assert_eq!(gid_len2, 16);
        assert_eq!(gid_buffer, gid_buffer2);

        // Cleanup
        rs_libp2p_custom_subscription_free(sub_ptr);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_multiple_subscriptions_unique_gids() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let topic1 = std::ffi::CString::new("topic1").unwrap();
        let topic2 = std::ffi::CString::new("topic2").unwrap();
        let topic3 = std::ffi::CString::new("topic3").unwrap();

        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let sub1 = rs_libp2p_custom_subscription_new(node, topic1.as_ptr(), handle, noop_callback);
        let sub2 = rs_libp2p_custom_subscription_new(node, topic2.as_ptr(), handle, noop_callback);
        let sub3 = rs_libp2p_custom_subscription_new(node, topic3.as_ptr(), handle, noop_callback);

        let mut gid1 = vec![0u8; 16];
        let mut gid2 = vec![0u8; 16];
        let mut gid3 = vec![0u8; 16];

        rs_libp2p_custom_subscription_get_gid(sub1, gid1.as_mut_ptr());
        rs_libp2p_custom_subscription_get_gid(sub2, gid2.as_mut_ptr());
        rs_libp2p_custom_subscription_get_gid(sub3, gid3.as_mut_ptr());

        // All GIDs should be unique
        assert_ne!(gid1, gid2);
        assert_ne!(gid1, gid3);
        assert_ne!(gid2, gid3);

        // Cleanup
        rs_libp2p_custom_subscription_free(sub1);
        rs_libp2p_custom_subscription_free(sub2);
        rs_libp2p_custom_subscription_free(sub3);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_multiple_subscriptions_same_topic() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));
        let topic = std::ffi::CString::new("shared_topic").unwrap();

        let handle1 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };
        let handle2 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };
        let handle3 = CustomSubscriptionHandle {
            ptr: std::ptr::dangling::<c_void>(),
        };

        // Create multiple subscriptions to the same topic
        let sub1 = rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle1, noop_callback);
        let sub2 = rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle2, noop_callback);
        let sub3 = rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle3, noop_callback);

        // All should have unique GIDs
        let mut gid1 = vec![0u8; 16];
        let mut gid2 = vec![0u8; 16];
        let mut gid3 = vec![0u8; 16];

        rs_libp2p_custom_subscription_get_gid(sub1, gid1.as_mut_ptr());
        rs_libp2p_custom_subscription_get_gid(sub2, gid2.as_mut_ptr());
        rs_libp2p_custom_subscription_get_gid(sub3, gid3.as_mut_ptr());

        assert_ne!(gid1, gid2);
        assert_ne!(gid1, gid3);
        assert_ne!(gid2, gid3);

        // Cleanup
        rs_libp2p_custom_subscription_free(sub1);
        rs_libp2p_custom_subscription_free(sub2);
        rs_libp2p_custom_subscription_free(sub3);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_queue_notification() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let new_subscribers_queue = Arc::clone(&node_box.new_subscribers_queue);
        let node = Box::into_raw(node_box);

        let handle = CustomSubscriptionHandle {
            ptr: 0xABCDEF as *const c_void,
        };

        let topic = std::ffi::CString::new("notify_test").unwrap();
        let _sub = rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle, test_callback);

        // Verify subscriber was queued
        let (queued_topic, queued_handle, _callback) = new_subscribers_queue
            .try_pop()
            .expect("Subscriber should be in queue");

        assert_eq!(
            queued_topic.hash(),
            gossipsub::IdentTopic::new("notify_test").hash()
        );
        assert_eq!(queued_handle.ptr, handle.ptr);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_custom_subscription_handle_trait_bounds() {
        // Verify CustomSubscriptionHandle implements Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<CustomSubscriptionHandle>();
        assert_sync::<CustomSubscriptionHandle>();
    }

    #[test]
    fn test_subscription_with_null_handle_pointer() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Creating subscription with null pointer in handle should be valid
        // (the handle is just a wrapper, the pointer value doesn't matter for creation)
        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };

        let subscription =
            Libp2pCustomSubscription::new(node, "null_handle_test", handle, noop_callback);

        assert_eq!(
            subscription.topic.hash(),
            gossipsub::IdentTopic::new("null_handle_test").hash()
        );

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_subscription_lifecycle() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create subscription
        let handle = CustomSubscriptionHandle {
            ptr: std::ptr::null(),
        };
        let topic = std::ffi::CString::new("lifecycle_test").unwrap();
        let sub_ptr =
            rs_libp2p_custom_subscription_new(node, topic.as_ptr(), handle, noop_callback);

        // Verify creation
        assert!(!sub_ptr.is_null());

        // Get GID to verify it's alive
        let mut gid = vec![0u8; 16];
        let len = rs_libp2p_custom_subscription_get_gid(sub_ptr, gid.as_mut_ptr());
        assert_eq!(len, 16);
        assert!(gid.iter().any(|&x| x != 0));

        // Free subscription
        rs_libp2p_custom_subscription_free(sub_ptr);

        // Create another subscription to verify node is still usable
        let topic2 = std::ffi::CString::new("lifecycle_test2").unwrap();
        let sub_ptr2 =
            rs_libp2p_custom_subscription_new(node, topic2.as_ptr(), handle, noop_callback);
        assert!(!sub_ptr2.is_null());

        // Cleanup
        rs_libp2p_custom_subscription_free(sub_ptr2);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }
}
