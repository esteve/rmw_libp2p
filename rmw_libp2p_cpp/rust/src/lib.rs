use async_std::task;
use std::os::raw::c_void;
use uuid::Uuid;
// use env_logger::{Builder, Env};
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
use std::hash::{Hash, Hasher};
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
}

pub struct Libp2pCustomPublisher {
    gid: Uuid,
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
                mdns : mdns,
            };

            libp2p::Swarm::new(transport, behaviour, peer_id)
        };

        let (stop_sender, mut stop_receiver) = oneshot::channel::<bool>();

        let thread_handle = task::spawn(async move {

        loop {
            select! {
                // use a oneshot future that will be triggered to stop the swarm
                // select! will wait on any future
                _ = stop_receiver => {
                    println!("Exit loop");
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
        }});

        Self {
            thread_handle: thread_handle,
            stop_sender: stop_sender,
        }
    }
}

impl Libp2pCustomPublisher {
    fn new() -> Self {
        Self {
            gid: Uuid::new_v4(),
        }
    }
}
#[no_mangle]
pub extern "C" fn rs_libp2p_custom_publisher_new() -> *mut Libp2pCustomPublisher {
    Box::into_raw(Box::new(Libp2pCustomPublisher::new()))
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
        std::ptr::copy_nonoverlapping(
            gid_bytes.as_ptr(),
            buf as *mut u8,
            count,
        );
    }
    count
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

        node.thread_handle.await});
}
