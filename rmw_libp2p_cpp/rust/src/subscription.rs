use crate::Libp2pCustomNode;

use std::ffi::CStr;
use std::io::Cursor;
use std::os::raw::c_char;

use uuid::Uuid;

use libp2p::gossipsub;

pub struct Libp2pCustomSubscription {
    gid: Uuid,
    node: *mut Libp2pCustomNode, // We need to store the Node here to have access to the outgoing queue
    topic: gossipsub::IdentTopic,
    incoming_queue: Arc<deadqueue::unlimited::Queue<(gossipsub::IdentTopic, Vec<u8>)>>,
}

impl Libp2pCustomSubscription {
    fn new(
        libp2p2_custom_node: *mut Libp2pCustomNode,
        topic_str: &str,
    ) -> Self {
        Self {
            gid: Uuid::new_v4(),
            node: libp2p2_custom_node,
            topic: gossipsub::IdentTopic::new(topic_str),
            incoming_queue: Arc::new(deadqueue::unlimited::Queue::new()),
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
) -> *mut Libp2pCustomSubscription {
    let topic_str = unsafe {
        assert!(!topic_str_ptr.is_null());
        CStr::from_ptr(topic_str_ptr)
    };

    let libp2p2_custom_subscription =
        Libp2pCustomSubscription::new(ptr_node, topic_str.to_str().unwrap());
    Box::into_raw(Box::new(libp2p2_custom_subscription))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_free(ptr: *mut Libp2pCustomSubscription) {
    if ptr.is_null() {
        return;
    }
    let _ = unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_subscription_get_gid(
    ptr: *mut Libp2pCustomSubscription,
    buf: *mut std::os::raw::c_uchar,
) -> usize {
    let libp2p2_custom_subscription = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let gid_bytes = libp2p2_custom_subscription.gid.as_bytes();
    let count = gid_bytes.len();
    unsafe {
        std::ptr::copy_nonoverlapping(gid_bytes.as_ptr(), buf as *mut u8, count);
    }
    count
}
