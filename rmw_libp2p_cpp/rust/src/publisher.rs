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

use crate::Libp2pCustomNode;

use std::ffi::CStr;
use std::io::Cursor;
use std::os::raw::c_char;

use uuid::Uuid;

use libp2p::gossipsub;

/// Represents a custom publisher using the Libp2p protocol.
pub struct Libp2pCustomPublisher {
    gid: Uuid,
    node: *mut Libp2pCustomNode, // We need to store the Node here to have access to the outgoing queue
    topic: gossipsub::IdentTopic,
    sequence_number: u64,
}

/// Represents a custom publisher for the Libp2p network.
///
/// This struct is responsible for publishing messages to a specific topic on the Libp2p network.
impl Libp2pCustomPublisher {
    /// Creates a new instance of `Libp2pCustomPublisher`.
    ///
    /// # Arguments
    ///
    /// * `libp2p_custom_node` - A pointer to the Libp2p custom node.
    /// * `topic_str` - The string representation of the topic to publish to.
    ///
    /// # Returns
    ///
    /// A new instance of `Libp2pCustomPublisher`.
    fn new(libp2p_custom_node: *mut Libp2pCustomNode, topic_str: &str) -> Self {
        Self {
            gid: Uuid::new_v4(),
            node: libp2p_custom_node,
            topic: gossipsub::IdentTopic::new(topic_str),
            sequence_number: 0,
        }
    }

    /// Publishes a message to the Libp2p network.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer containing the message to be published.
    fn publish(&mut self, buffer: Vec<u8>) -> () {
        let libp2p_custom_node = unsafe {
            assert!(!self.node.is_null());
            &mut *self.node
        };
        libp2p_custom_node.publish_message(self.topic.clone(), buffer);
        self.sequence_number += 1;
    }
}

/// Creates a new `Libp2pCustomPublisher`.
///
/// This function takes a raw pointer to a `Libp2pCustomNode` and a raw pointer to a C string representing the topic.
/// It then creates a new `Libp2pCustomPublisher` for the given node and topic, and returns a raw pointer to the heap-allocated publisher.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_node` - A raw pointer to a `Libp2pCustomNode`.
/// * `topic_str_ptr` - A raw pointer to a C string representing the topic.
///
/// # Returns
///
/// A raw pointer to a `Libp2pCustomPublisher`.
///
/// # Panics
///
/// This function will panic if `topic_str_ptr` is null or if it does not point to a valid null-terminated string.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
) -> *mut Libp2pCustomPublisher {
    let topic_str = unsafe {
        assert!(!topic_str_ptr.is_null());
        CStr::from_ptr(topic_str_ptr)
    };

    let libp2p_custom_publisher =
        Libp2pCustomPublisher::new(ptr_node, topic_str.to_str().unwrap());
    Box::into_raw(Box::new(libp2p_custom_publisher))
}

/// Frees a `Libp2pCustomPublisher` from memory.
///
/// This function takes a raw pointer to a `Libp2pCustomPublisher`, converts it back into a `Box`, and then drops the `Box`, freeing the memory.
/// If the provided pointer is null, the function returns immediately.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomPublisher`.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_free(ptr: *mut Libp2pCustomPublisher) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}

/// Gets the GID of a `Libp2pCustomPublisher`.
///
/// This function takes a raw pointer to a `Libp2pCustomPublisher` and a raw pointer to a buffer.
/// It then copies the bytes of the GID of the publisher into the buffer and returns the number of bytes copied.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr` - A raw pointer to a `Libp2pCustomPublisher`.
/// * `buf` - A raw pointer to a buffer where the GID bytes will be copied.
///
/// # Returns
///
/// The number of bytes copied into the buffer.
///
/// # Panics
///
/// This function will panic if `ptr` is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_get_gid(
    ptr: *mut Libp2pCustomPublisher,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    let libp2p_custom_publisher = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gid_bytes = libp2p_custom_publisher.gid.as_bytes();
    let count = gid_bytes.len();
    unsafe {
        std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf as *mut u8, count);
    }
    count
}

/// Publishes a message using a `Libp2pCustomPublisher`.
///
/// This function takes raw pointers to a `Libp2pCustomPublisher` and a `Cursor<Vec<u8>>`.
/// It then publishes the contents of the `Cursor<Vec<u8>>` using the `Libp2pCustomPublisher`.
///
/// # Safety
///
/// This function is unsafe because it uses raw pointers and calls unsafe functions.
///
/// # Arguments
///
/// * `ptr_publisher` - A raw pointer to a `Libp2pCustomPublisher`.
/// * `ptr_buffer` - A raw pointer to a `Cursor<Vec<u8>>` containing the message to publish.
///
/// # Returns
///
/// Currently, this function always returns 0.
///
/// # Panics
///
/// This function will panic if either `ptr_publisher` or `ptr_buffer` is null.
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_publish(
    ptr_publisher: *mut Libp2pCustomPublisher,
    ptr_buffer: *const Cursor<Vec<u8>>,
) -> usize {
    let libp2p_custom_publisher = unsafe {
        assert!(!ptr_publisher.is_null());
        &mut *ptr_publisher
    };
    let buffer = unsafe {
        assert!(!ptr_buffer.is_null());
        &*ptr_buffer
    };
    libp2p_custom_publisher.publish(buffer.get_ref().to_vec());
    // TODO(esteve): return the number of bytes published
    0
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_get_sequence_number(
    ptr: *mut Libp2pCustomPublisher,
) -> u64 {
    let libp2p_custom_publisher = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    libp2p_custom_publisher.sequence_number
}

