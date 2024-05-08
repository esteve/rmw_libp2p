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
use std::io::Cursor;
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
    node: *mut Libp2pCustomNode, // We need to store the Node here to have access to the outgoing queue
    topic: gossipsub::IdentTopic,
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
        let libp2p2_custom_node = unsafe {
            assert!(!ptr_node.is_null());
            &mut *ptr_node
        };

        libp2p2_custom_node.notify_new_subscriber(
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
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
    obj: CustomSubscriptionHandle,
    callback: unsafe extern "C" fn(&CustomSubscriptionHandle, *mut u8, len: usize),
) -> *mut Libp2pCustomSubscription {
    let topic_str = unsafe {
        assert!(!topic_str_ptr.is_null());
        CStr::from_ptr(topic_str_ptr)
    };

    let libp2p2_custom_subscription =
        Libp2pCustomSubscription::new(ptr_node, topic_str.to_str().unwrap(), obj, callback);
    Box::into_raw(Box::new(libp2p2_custom_subscription))
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
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_free(
    ptr_subscription: *mut Libp2pCustomSubscription,
) {
    if ptr_subscription.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr_subscription) };
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
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_get_gid(
    ptr_subscription: *mut Libp2pCustomSubscription,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    let libp2p2_custom_subscription = unsafe {
        assert!(!ptr_subscription.is_null());
        &mut *ptr_subscription
    };
    let gid_bytes = libp2p2_custom_subscription.gid.as_bytes();
    let count = gid_bytes.len();
    unsafe {
        std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf as *mut u8, count);
    }
    count
}
