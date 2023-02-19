use async_std::task;
use std::os::raw::c_char;
use std::os::raw::c_void;
use uuid::Uuid;
// use env_logger::{Builder, Env};
use deadqueue;
use futures::channel::oneshot;
use futures::{future, prelude::*, select};
use libp2p::gossipsub::MessageId;
use libp2p::gossipsub::{
    GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity, ValidationMode,
};
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::{gossipsub, identity, swarm::SwarmEvent, Multiaddr, NetworkBehaviour, PeerId};
use std::boxed::Box;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct RosNetworkBehaviour {
    gossipsub: gossipsub::Gossipsub,
    mdns: Mdns,
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

// #[no_mangle]
// pub extern "C" fn rs_rmw_init() -> *const c_void {
//     let handle = task::spawn(async { event_loop().await });
//     Box::into_raw(Box::new(handle)) as *const c_void
// }

pub struct Libp2pCustomNode {
    thread_handle: task::JoinHandle<()>,
    stop_sender: oneshot::Sender<bool>,
    outgoing_queue: Arc<deadqueue::unlimited::Queue<(Topic, Vec<u8>)>>,
}

pub struct Libp2pCustomPublisher {
    gid: Uuid,
    node: Arc<Libp2pCustomNode>, // We need to store the Node here to have access to the outgoing queue
    topic: Topic,
}

impl Libp2pCustomNode {
    fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();

        let peer_id = PeerId::from(keypair.public());

        let transport = task::block_on(libp2p::development_transport(keypair.clone())).unwrap();

        let mut swarm = {
            let message_id_fn = |message: &GossipsubMessage| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                // same content will be propagated.
                .build()
                .expect("Valid config");

            let gossipsub: gossipsub::Gossipsub =
                gossipsub::Gossipsub::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
                    .expect("Correct configuration");

            let mdns = task::block_on(Mdns::new(MdnsConfig::default())).unwrap();

            let behaviour = RosNetworkBehaviour {
                gossipsub: gossipsub,
                mdns: mdns,
            };

            libp2p::Swarm::new(transport, behaviour, peer_id)
        };

        let (stop_sender, mut stop_receiver) = oneshot::channel::<bool>();

        let outgoing_queue = Arc::new(deadqueue::unlimited::Queue::<(Topic, Vec<u8>)>::new());

        let outgoing_queue_clone = Arc::clone(&outgoing_queue);

        let thread_handle = task::spawn(async move {
            loop {
                select! {
                    // use a oneshot future that will be triggered to stop the swarm
                    // select! will wait on any future
                    _ = stop_receiver => {
                        println!("Exit loop");
                    },
                    // pop messages from the queue and publish them to the network
                    (topic, buffer) = outgoing_queue_clone.pop().fuse() => {
                        swarm.behaviour_mut().gossipsub.publish(topic, buffer);
                    },
                    event = swarm.select_next_some() => match event {
                        SwarmEvent::Behaviour(OutEvent::Gossipsub(GossipsubEvent::Message {
                            propagation_source: peer_id,
                            message_id: id,
                            message,
                        })) => {
                            println!(
                                "Got message: {} with id: {} from peer: {:?}",
                                String::from_utf8_lossy(&message.data),
                                id,
                                peer_id
                            );
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
                                    .gossipsub
                                    .add_explicit_peer(&peer);
                            }
                        }
                        SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
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
                        _ => {}
                    }
                }
            }
        });

        Self {
            thread_handle: thread_handle,
            stop_sender: stop_sender,
            outgoing_queue: outgoing_queue,
        }
    }

    fn publish_message(&self, topic: Topic, buffer: Vec<u8>) -> () {
        self.outgoing_queue.push((topic, buffer));
    }
}

impl Libp2pCustomPublisher {
    fn new(libp2p2_custom_node: Arc<Libp2pCustomNode>, topic_str: &str) -> Self {
        Self {
            gid: Uuid::new_v4(),
            node: libp2p2_custom_node,
            topic: Topic::new(topic_str),
        }
    }

    fn publish(&self, buffer: Vec<u8>) -> () {
        self.node.publish_message(self.topic.clone(), buffer);
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_new(
    ptr_node: *mut Libp2pCustomNode,
    topic_str_ptr: *const c_char,
) -> *mut Libp2pCustomPublisher {
    let libp2p2_custom_node = Arc::from(unsafe {
        assert!(!ptr_node.is_null());
        Box::from_raw(ptr_node)
    });
    let topic_str = unsafe {
        assert!(!topic_str_ptr.is_null());
        CStr::from_ptr(topic_str_ptr)
    };

    let libp2p2_custom_publisher =
        Libp2pCustomPublisher::new(libp2p2_custom_node, topic_str.to_str().unwrap());
    Box::into_raw(Box::new(libp2p2_custom_publisher))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_free(ptr: *mut Libp2pCustomPublisher) {
    if ptr.is_null() {
        return;
    }
    let publisher = unsafe { Box::from_raw(ptr) };
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
    // libp2p2_custom_publisher.node_ptr.swarm.behaviour_mut().gossipsub.publish(libp2p2_custom_publisher.topic.clone(), *buffer);
    0
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_new() -> *mut Libp2pCustomNode {
    Box::into_raw(Box::new(Libp2pCustomNode::new()))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_custom_node_free(ptr: *mut Libp2pCustomNode) {
    if ptr.is_null() {
        return;
    }
    let node = unsafe { Box::from_raw(ptr) };
    task::block_on(async {
        node.stop_sender.send(true).unwrap();

        node.thread_handle.await
    });
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_new() -> *mut Cursor<Vec<u8>> {
    let libp2p2_cdr_buffer = Cursor::new(Vec::new());
    Box::into_raw(Box::new(libp2p2_cdr_buffer))
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free(ptr: *mut Cursor<Vec<u8>>) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint64(ptr: *mut Cursor<Vec<u8>>, n: *mut u64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint32(ptr: *mut Cursor<Vec<u8>>, n: *mut u32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_uint8(ptr: *mut Cursor<Vec<u8>>, n: *mut u8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u8, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int64(ptr: *mut Cursor<Vec<u8>>, n: *mut i64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int32(ptr: *mut Cursor<Vec<u8>>, n: *mut i32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int16(ptr: *mut Cursor<Vec<u8>>, n: *mut i16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_int8(ptr: *mut Cursor<Vec<u8>>, n: *mut i8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, i8, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char(ptr: *mut Cursor<Vec<u8>>, n: *mut c_char) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, c_char, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_char16(ptr: *mut Cursor<Vec<u8>>, n: *mut u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, u16, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_float(ptr: *mut Cursor<Vec<u8>>, n: *mut f32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f32, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_double(ptr: *mut Cursor<Vec<u8>>, n: *mut f64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, f64, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_bool(ptr: *mut Cursor<Vec<u8>>, n: *mut bool) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    unsafe {
        *n = cdr::deserialize_from::<_, bool, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const c_char,
    size: *mut usize,
) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, CString, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.as_bytes().len();

        if *size != 0 {
            *s = cs.into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_read_u16string(
    ptr: *mut Cursor<Vec<u8>>,
    s: *mut *const u16,
    size: *mut usize,
) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    let cs = cdr::deserialize_from::<_, Vec<u16>, _>(libp2p2_cdr_buffer, cdr::Infinite).unwrap();
    unsafe {
        *size = cs.len();

        if *size != 0 {
            *s = cs.as_ptr();
        }
    }
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint64(ptr: *mut Cursor<Vec<u8>>, n: u64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint32(ptr: *mut Cursor<Vec<u8>>, n: u32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_uint8(ptr: *mut Cursor<Vec<u8>>, n: u8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int64(ptr: *mut Cursor<Vec<u8>>, n: i64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int32(ptr: *mut Cursor<Vec<u8>>, n: i32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int16(ptr: *mut Cursor<Vec<u8>>, n: i16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_int8(ptr: *mut Cursor<Vec<u8>>, n: i8) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char(ptr: *mut Cursor<Vec<u8>>, n: c_char) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_char16(ptr: *mut Cursor<Vec<u8>>, n: u16) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_float(ptr: *mut Cursor<Vec<u8>>, n: f32) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_double(ptr: *mut Cursor<Vec<u8>>, n: f64) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}

#[no_mangle]
pub extern "C" fn rs_libp2p_cdr_buffer_write_bool(ptr: *mut Cursor<Vec<u8>>, n: bool) {
    let libp2p2_cdr_buffer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    cdr::serialize_into::<_, _, _, cdr::CdrBe>(libp2p2_cdr_buffer, &n, cdr::Infinite).unwrap();
}
