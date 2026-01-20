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

        let mut out_buffer = Vec::<u8>::new();

        let gid_bytes = self.gid.as_bytes();
        let count = gid_bytes.len();
        for i in 0..count {
            cdr::serialize_into::<_, u8, _, cdr::CdrBe>(
                &mut out_buffer,
                &gid_bytes[i],
                cdr::Infinite,
            )
            .unwrap();
        }
        cdr::serialize_into::<_, _, _, cdr::CdrBe>(
            &mut out_buffer,
            &self.sequence_number,
            cdr::Infinite,
        )
        .unwrap();

        out_buffer.extend(buffer);
        libp2p_custom_node.publish_message(self.topic.clone(), out_buffer);
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

    let libp2p_custom_publisher = Libp2pCustomPublisher::new(ptr_node, topic_str.to_str().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rs_libp2p_custom_node_new;
    use crate::rs_libp2p_custom_node_free;
    use std::ffi::CString;

    #[test]
    fn test_publisher_creation_and_destruction() {
        let node_ptr = rs_libp2p_custom_node_new();
        assert!(!node_ptr.is_null());

        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());
        assert!(!publisher_ptr.is_null());

        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_free_null_pointer() {
        // Should not panic when freeing null pointer
        rs_libp2p_custom_publisher_free(std::ptr::null_mut());
    }

    #[test]
    fn test_publisher_get_gid() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        let mut gid_buffer: [u8; 16] = [0; 16];
        let gid_len = rs_libp2p_custom_publisher_get_gid(publisher_ptr, gid_buffer.as_mut_ptr());

        // UUID is 16 bytes
        assert_eq!(gid_len, 16);
        // GID should not be all zeros (it's a random UUID)
        assert!(gid_buffer.iter().any(|&x| x != 0));

        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_initial_sequence_number() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        let seq_num = rs_libp2p_custom_publisher_get_sequence_number(publisher_ptr);
        assert_eq!(seq_num, 0);

        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_unique_gids() {
        let node_ptr = rs_libp2p_custom_node_new();

        let topic1 = CString::new("topic1").unwrap();
        let topic2 = CString::new("topic2").unwrap();

        let publisher1 = rs_libp2p_custom_publisher_new(node_ptr, topic1.as_ptr());
        let publisher2 = rs_libp2p_custom_publisher_new(node_ptr, topic2.as_ptr());

        let mut gid1: [u8; 16] = [0; 16];
        let mut gid2: [u8; 16] = [0; 16];

        rs_libp2p_custom_publisher_get_gid(publisher1, gid1.as_mut_ptr());
        rs_libp2p_custom_publisher_get_gid(publisher2, gid2.as_mut_ptr());

        // GIDs should be different for different publishers
        assert_ne!(gid1, gid2);

        rs_libp2p_custom_publisher_free(publisher1);
        rs_libp2p_custom_publisher_free(publisher2);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_publish() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        // Create a buffer with some test data
        let test_data: Vec<u8> = vec![1, 2, 3, 4, 5];
        let buffer = Cursor::new(test_data);
        let buffer_ptr = Box::into_raw(Box::new(buffer));

        // Publish should not panic and return 0
        let result = rs_libp2p_custom_publisher_publish(publisher_ptr, buffer_ptr);
        assert_eq!(result, 0);

        // Sequence number should increment after publish
        let seq_num = rs_libp2p_custom_publisher_get_sequence_number(publisher_ptr);
        assert_eq!(seq_num, 1);

        // Clean up
        unsafe { Box::from_raw(buffer_ptr) };
        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_multiple_publishes() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        // Publish multiple times
        for i in 0..5 {
            let test_data: Vec<u8> = vec![i as u8; 10];
            let buffer = Cursor::new(test_data);
            let buffer_ptr = Box::into_raw(Box::new(buffer));

            rs_libp2p_custom_publisher_publish(publisher_ptr, buffer_ptr);

            unsafe { Box::from_raw(buffer_ptr) };
        }

        // Sequence number should reflect the number of publishes
        let seq_num = rs_libp2p_custom_publisher_get_sequence_number(publisher_ptr);
        assert_eq!(seq_num, 5);

        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_multiple_publishers_same_topic() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("shared_topic").unwrap();

        let publisher1 = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());
        let publisher2 = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        assert!(!publisher1.is_null());
        assert!(!publisher2.is_null());
        assert_ne!(publisher1, publisher2);

        rs_libp2p_custom_publisher_free(publisher1);
        rs_libp2p_custom_publisher_free(publisher2);
        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_with_various_topics() {
        let node_ptr = rs_libp2p_custom_node_new();

        // Test with various topic names
        let topics = vec![
            "simple",
            "with/slashes",
            "with_underscores",
            "MixedCase",
            "numbers123",
            "/leading/slash",
        ];

        for topic_name in topics {
            let topic = CString::new(topic_name).unwrap();
            let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());
            assert!(!publisher_ptr.is_null());
            rs_libp2p_custom_publisher_free(publisher_ptr);
        }

        rs_libp2p_custom_node_free(node_ptr);
    }

    #[test]
    fn test_publisher_gid_stability() {
        let node_ptr = rs_libp2p_custom_node_new();
        let topic = CString::new("test_topic").unwrap();
        let publisher_ptr = rs_libp2p_custom_publisher_new(node_ptr, topic.as_ptr());

        let mut gid1: [u8; 16] = [0; 16];
        let mut gid2: [u8; 16] = [0; 16];

        rs_libp2p_custom_publisher_get_gid(publisher_ptr, gid1.as_mut_ptr());
        rs_libp2p_custom_publisher_get_gid(publisher_ptr, gid2.as_mut_ptr());

        // GID should remain the same when queried multiple times
        assert_eq!(gid1, gid2);

        rs_libp2p_custom_publisher_free(publisher_ptr);
        rs_libp2p_custom_node_free(node_ptr);
    }
}
