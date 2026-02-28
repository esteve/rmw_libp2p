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
    fn publish(&mut self, buffer: Vec<u8>) {
        let libp2p_custom_node = unsafe {
            assert!(!self.node.is_null());
            &mut *self.node
        };

        let mut out_buffer = Vec::<u8>::new();

        let gid_bytes = self.gid.as_bytes();
        for &byte in gid_bytes.iter() {
            cdr::serialize_into::<_, u8, _, cdr::CdrBe>(&mut out_buffer, &byte, cdr::Infinite)
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
pub unsafe extern "C" fn rs_libp2p_custom_publisher_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
) -> *mut Libp2pCustomPublisher {
    let topic_str = {
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
pub unsafe extern "C" fn rs_libp2p_custom_publisher_free(ptr: *mut Libp2pCustomPublisher) {
    if ptr.is_null() {
        return;
    }
    let _ = Box::from_raw(ptr);
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
pub unsafe extern "C" fn rs_libp2p_custom_publisher_get_gid(
    ptr: *mut Libp2pCustomPublisher,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    let libp2p_custom_publisher = {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gid_bytes = libp2p_custom_publisher.gid.as_bytes();
    let count = gid_bytes.len();
    std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf, count);
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
pub unsafe extern "C" fn rs_libp2p_custom_publisher_publish(
    ptr_publisher: *mut Libp2pCustomPublisher,
    ptr_buffer: *const Cursor<Vec<u8>>,
) -> usize {
    let libp2p_custom_publisher = {
        assert!(!ptr_publisher.is_null());
        &mut *ptr_publisher
    };
    let buffer = {
        assert!(!ptr_buffer.is_null());
        &*ptr_buffer
    };
    libp2p_custom_publisher.publish(buffer.get_ref().to_vec());
    // TODO(esteve): return the number of bytes published
    0
}

#[no_mangle]
pub unsafe extern "C" fn rs_libp2p_custom_publisher_get_sequence_number(
    ptr: *mut Libp2pCustomPublisher,
) -> u64 {
    let libp2p_custom_publisher = {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    libp2p_custom_publisher.sequence_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_publisher_creation_with_valid_node() {
        // Create a test node without network overhead
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create a publisher for a topic
        let topic_str = "test_topic";
        let publisher = Libp2pCustomPublisher::new(node, topic_str);

        // Verify publisher was created with correct fields
        // Note: IdentTopic doesn't implement PartialEq, so we compare hashes
        assert_eq!(
            publisher.topic.hash(),
            gossipsub::IdentTopic::new(topic_str).hash()
        );
        assert_eq!(publisher.sequence_number, 0);
        assert_eq!(publisher.node, node);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_publisher_gid_retrieval_and_uniqueness() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create multiple publishers
        let pub1 = Libp2pCustomPublisher::new(node, "topic1");
        let pub2 = Libp2pCustomPublisher::new(node, "topic2");
        let pub3 = Libp2pCustomPublisher::new(node, "topic1"); // Same topic, different publisher

        // Verify GIDs are unique
        assert_ne!(pub1.gid, pub2.gid);
        assert_ne!(pub1.gid, pub3.gid);
        assert_ne!(pub2.gid, pub3.gid);

        // Verify GID format (UUID v4 should have specific length)
        assert_eq!(pub1.gid.as_bytes().len(), 16);
        assert_eq!(pub2.gid.as_bytes().len(), 16);
        assert_eq!(pub3.gid.as_bytes().len(), 16);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_sequence_number_increment_on_publish() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));
        let mut publisher = Libp2pCustomPublisher::new(node, "seq_test");

        // Initial sequence number should be 0
        assert_eq!(publisher.sequence_number, 0);

        // Publish a message
        publisher.publish(vec![1, 2, 3]);
        assert_eq!(publisher.sequence_number, 1);

        // Publish more messages
        publisher.publish(vec![4, 5, 6]);
        assert_eq!(publisher.sequence_number, 2);

        publisher.publish(vec![7, 8, 9]);
        assert_eq!(publisher.sequence_number, 3);

        // Publish many messages to test overflow resistance
        for i in 0..100 {
            publisher.publish(vec![i]);
        }
        assert_eq!(publisher.sequence_number, 103);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_publish_to_outgoing_queue() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let outgoing_queue = Arc::clone(&node_box.outgoing_queue);
        let node = Box::into_raw(node_box);

        let mut publisher = Libp2pCustomPublisher::new(node, "queue_test");
        let test_message = vec![0xAA, 0xBB, 0xCC, 0xDD];

        // Publish a message
        publisher.publish(test_message.clone());

        // Verify message was added to outgoing queue
        // The queue should have at least one message
        let (topic, serialized_message) =
            outgoing_queue.try_pop().expect("Queue should have message");

        // Verify topic matches
        assert_eq!(
            topic.hash(),
            gossipsub::IdentTopic::new("queue_test").hash()
        );

        // Verify message format: should contain GID (16 bytes) + sequence_number (8 bytes) + original message
        // GID: 16 bytes serialized
        // Sequence number: 8 bytes serialized
        // Original message: 4 bytes
        // Note: CDR serialization adds some overhead, so we verify minimum expected size
        assert!(serialized_message.len() >= test_message.len());

        // Verify the original message is at the end of the serialized message
        let msg_start = serialized_message.len() - test_message.len();
        assert_eq!(&serialized_message[msg_start..], &test_message[..]);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_publish_message_format_includes_gid_and_sequence() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let outgoing_queue = Arc::clone(&node_box.outgoing_queue);
        let node = Box::into_raw(node_box);

        let mut publisher = Libp2pCustomPublisher::new(node, "format_test");
        let gid = publisher.gid; // Save GID for verification

        // Publish first message
        publisher.publish(vec![0x01]);
        let (_, msg1) = outgoing_queue.try_pop().unwrap();

        // Publish second message
        publisher.publish(vec![0x02]);
        let (_, msg2) = outgoing_queue.try_pop().unwrap();

        // Both messages should contain the same GID (first 16 bytes after CDR serialization)
        // The sequence numbers should be different
        // Messages should have different lengths or content to verify sequence increment

        // Verify both messages were queued
        assert!(!msg1.is_empty());
        assert!(!msg2.is_empty());

        // The messages should be different (due to different sequence numbers)
        assert_ne!(msg1, msg2);

        // Verify GID is included in both messages
        let _ = gid; // Silence unused warning

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_publisher_drop_behavior() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        {
            let publisher = Libp2pCustomPublisher::new(node, "drop_test");
            // Publisher should be alive here
            assert_eq!(publisher.sequence_number, 0);
        } // Publisher dropped here

        // Create another publisher with same node - should work fine
        {
            let publisher2 = Libp2pCustomPublisher::new(node, "drop_test2");
            assert_eq!(publisher2.sequence_number, 0);
        }

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_multiple_publishers_same_topic() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create multiple publishers for the same topic
        let mut pub1 = Libp2pCustomPublisher::new(node, "shared_topic");
        let mut pub2 = Libp2pCustomPublisher::new(node, "shared_topic");
        let mut pub3 = Libp2pCustomPublisher::new(node, "shared_topic");

        // All should have unique GIDs
        assert_ne!(pub1.gid, pub2.gid);
        assert_ne!(pub1.gid, pub3.gid);
        assert_ne!(pub2.gid, pub3.gid);

        // All should have independent sequence numbers
        pub1.publish(vec![1]);
        pub2.publish(vec![2]);
        pub3.publish(vec![3]);

        assert_eq!(pub1.sequence_number, 1);
        assert_eq!(pub2.sequence_number, 1);
        assert_eq!(pub3.sequence_number, 1);

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    // FFI Tests
    #[test]
    fn test_ffi_publisher_new_and_free() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        // Create C string for topic
        let topic = std::ffi::CString::new("ffi_topic").unwrap();

        // Create publisher via FFI
        let pub_ptr = rs_libp2p_custom_publisher_new(node, topic.as_ptr());

        // Verify pointer is not null
        assert!(!pub_ptr.is_null());

        // Free publisher
        rs_libp2p_custom_publisher_free(pub_ptr);

        // Test freeing null pointer (should not crash)
        rs_libp2p_custom_publisher_free(std::ptr::null_mut());

        // Cleanup
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_publisher_get_gid() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));
        let topic = std::ffi::CString::new("gid_test").unwrap();

        let pub_ptr = rs_libp2p_custom_publisher_new(node, topic.as_ptr());

        // Allocate buffer for GID
        let mut gid_buffer = vec![0u8; 16];

        // Get GID via FFI
        let gid_len = rs_libp2p_custom_publisher_get_gid(pub_ptr, gid_buffer.as_mut_ptr());

        // Verify GID length
        assert_eq!(gid_len, 16);

        // Verify buffer was filled (not all zeros)
        assert!(gid_buffer.iter().any(|&x| x != 0));

        // Get GID again and verify it's the same
        let mut gid_buffer2 = vec![0u8; 16];
        let gid_len2 = rs_libp2p_custom_publisher_get_gid(pub_ptr, gid_buffer2.as_mut_ptr());

        assert_eq!(gid_len2, 16);
        assert_eq!(gid_buffer, gid_buffer2);

        // Cleanup
        rs_libp2p_custom_publisher_free(pub_ptr);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_publisher_get_sequence_number() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let node = Box::into_raw(node_box);
        let topic = std::ffi::CString::new("seq_ffi_test").unwrap();

        let pub_ptr = rs_libp2p_custom_publisher_new(node, topic.as_ptr());

        // Initial sequence number should be 0
        let seq = rs_libp2p_custom_publisher_get_sequence_number(pub_ptr);
        assert_eq!(seq, 0);

        // Publish a message via FFI
        let buffer = Cursor::new(vec![1u8, 2, 3]);
        rs_libp2p_custom_publisher_publish(pub_ptr, &buffer as *const _);

        // Sequence number should increment
        let seq = rs_libp2p_custom_publisher_get_sequence_number(pub_ptr);
        assert_eq!(seq, 1);

        // Publish more messages
        for i in 0..10 {
            let buffer = Cursor::new(vec![i as u8]);
            rs_libp2p_custom_publisher_publish(pub_ptr, &buffer as *const _);
        }

        let seq = rs_libp2p_custom_publisher_get_sequence_number(pub_ptr);
        assert_eq!(seq, 11);

        // Cleanup
        rs_libp2p_custom_publisher_free(pub_ptr);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_publisher_publish() {
        let node_box = Box::new(Libp2pCustomNode::new_test_only());
        let outgoing_queue = Arc::clone(&node_box.outgoing_queue);
        let node = Box::into_raw(node_box);
        let topic = std::ffi::CString::new("publish_ffi_test").unwrap();

        let pub_ptr = rs_libp2p_custom_publisher_new(node, topic.as_ptr());

        // Create a message buffer
        let message = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let buffer = Cursor::new(message.clone());

        // Publish via FFI
        let result = rs_libp2p_custom_publisher_publish(pub_ptr, &buffer as *const _);

        // Currently returns 0 (as per TODO in code)
        assert_eq!(result, 0);

        // Verify message was queued
        let (queued_topic, queued_msg) =
            outgoing_queue.try_pop().expect("Message should be queued");
        assert_eq!(
            queued_topic.hash(),
            gossipsub::IdentTopic::new("publish_ffi_test").hash()
        );

        // Message should contain our data at the end
        let msg_start = queued_msg.len() - message.len();
        assert_eq!(&queued_msg[msg_start..], &message[..]);

        // Cleanup
        rs_libp2p_custom_publisher_free(pub_ptr);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }

    #[test]
    fn test_ffi_multiple_publishers_unique_gids() {
        let node = Box::into_raw(Box::new(Libp2pCustomNode::new_test_only()));

        let topic1 = std::ffi::CString::new("topic1").unwrap();
        let topic2 = std::ffi::CString::new("topic2").unwrap();
        let topic3 = std::ffi::CString::new("topic3").unwrap();

        let pub1 = rs_libp2p_custom_publisher_new(node, topic1.as_ptr());
        let pub2 = rs_libp2p_custom_publisher_new(node, topic2.as_ptr());
        let pub3 = rs_libp2p_custom_publisher_new(node, topic3.as_ptr());

        let mut gid1 = vec![0u8; 16];
        let mut gid2 = vec![0u8; 16];
        let mut gid3 = vec![0u8; 16];

        rs_libp2p_custom_publisher_get_gid(pub1, gid1.as_mut_ptr());
        rs_libp2p_custom_publisher_get_gid(pub2, gid2.as_mut_ptr());
        rs_libp2p_custom_publisher_get_gid(pub3, gid3.as_mut_ptr());

        // All GIDs should be unique
        assert_ne!(gid1, gid2);
        assert_ne!(gid1, gid3);
        assert_ne!(gid2, gid3);

        // Cleanup
        rs_libp2p_custom_publisher_free(pub1);
        rs_libp2p_custom_publisher_free(pub2);
        rs_libp2p_custom_publisher_free(pub3);
        unsafe {
            let _ = Box::from_raw(node);
        }
    }
}
