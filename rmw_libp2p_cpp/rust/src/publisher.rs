use crate::Libp2pCustomNode;

use std::ffi::CStr;
use std::io::Cursor;
use std::os::raw::c_char;

use uuid::Uuid;

use libp2p::gossipsub::IdentTopic;

pub struct Libp2pCustomPublisher {
    gid: Uuid,
    node: *mut Libp2pCustomNode, // We need to store the Node here to have access to the outgoing queue
    topic: IdentTopic,
}

impl Libp2pCustomPublisher {
    fn new(libp2p2_custom_node: *mut Libp2pCustomNode, topic_str: &str) -> Self {
        Self {
            gid: Uuid::new_v4(),
            node: libp2p2_custom_node,
            topic: IdentTopic::new(topic_str),
        }
    }

    fn publish(&self, buffer: Vec<u8>) -> () {
        let libp2p2_custom_node = unsafe {
            assert!(!self.node.is_null());
            &mut *self.node
        };

        libp2p2_custom_node.publish_message(self.topic.clone(), buffer);
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
) -> *mut Libp2pCustomPublisher {
    let topic_str = unsafe {
        assert!(!topic_str_ptr.is_null());
        CStr::from_ptr(topic_str_ptr)
    };

    let libp2p2_custom_publisher =
        Libp2pCustomPublisher::new(ptr_node, topic_str.to_str().unwrap());
    Box::into_raw(Box::new(libp2p2_custom_publisher))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_free(ptr: *mut Libp2pCustomPublisher) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_get_gid(
    ptr: *mut Libp2pCustomPublisher,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    let libp2p2_custom_publisher = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gid_bytes = libp2p2_custom_publisher.gid.as_bytes();
    let count = gid_bytes.len();
    unsafe {
        std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf as *mut u8, count);
    }
    count
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_publish(
    ptr_publisher: *mut Libp2pCustomPublisher,
    ptr_buffer: *const Cursor<Vec<u8>>,
) -> usize {
    let libp2p2_custom_publisher = unsafe {
        assert!(!ptr_publisher.is_null());
        &mut *ptr_publisher
    };
    let buffer = unsafe {
        assert!(!ptr_buffer.is_null());
        &*ptr_buffer
    };
    libp2p2_custom_publisher.publish(buffer.get_ref().to_vec());
    0
}
