use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libp2p::gossipsub::{
    GossipsubEvent, GossipsubMessage, IdentTopic, MessageAuthenticity, MessageId, ValidationMode,
};
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::{gossipsub, identity, swarm::SwarmEvent, NetworkBehaviour, PeerId};

use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::{select, task};

use futures_util::{FutureExt, StreamExt};

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

pub struct Libp2pCustomNode {
    thread_handle: Option<task::JoinHandle<()>>,
    stop_sender: Option<oneshot::Sender<bool>>,
    outgoing_queue: Arc<deadqueue::unlimited::Queue<(IdentTopic, Vec<u8>)>>,
    reactor: Runtime,
}

impl Libp2pCustomNode {
    fn create_swarm(reactor: &Runtime) -> libp2p::Swarm<RosNetworkBehaviour> {
        let keypair = identity::Keypair::generate_ed25519();

        let peer_id = PeerId::from(keypair.public());

        let transport = reactor
            .block_on(libp2p::development_transport(keypair.clone()))
            .unwrap();

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

        let mdns = reactor.block_on(Mdns::new(MdnsConfig::default())).unwrap();

        let behaviour = RosNetworkBehaviour {
            gossipsub: gossipsub,
            mdns: mdns,
        };

        libp2p::Swarm::new(transport, behaviour, peer_id)
    }

    fn new() -> Self {
        let reactor = Runtime::new().unwrap();

        let (stop_sender, stop_receiver) = tokio::sync::oneshot::channel::<bool>();
        let outgoing_queue = Arc::new(deadqueue::unlimited::Queue::<(IdentTopic, Vec<u8>)>::new());

        let mut swarm = Self::create_swarm(&reactor);

        let outgoing_queue_clone = Arc::clone(&outgoing_queue);
        let mut stop_receiver = Some(stop_receiver);
        let thread_handle = reactor.spawn(async move {
            loop {
                select! {
                    // use a oneshot future that will be triggered to stop the swarm
                    // select! will wait on any future
                    _ = stop_receiver.as_mut().unwrap().fuse() => {
                        println!("Exit loop");
                        break;
                    },
                    // pop messages from the queue and publish them to the network
                    (topic, buffer) = outgoing_queue_clone.pop().fuse() => {
                        println!("Publishing message on topic {} : {:?}", topic, buffer);
                        if let Err(e) = swarm.behaviour_mut().gossipsub.publish(topic.clone(), buffer.clone()) {
                            println!("Publish error: {e:?}");
                        }
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
                        _ => {
                            println!("UNKNOWN EVENT");
                        }
                    }
                }
            }
        });

        Self {
            thread_handle: Some(thread_handle),
            stop_sender: Some(stop_sender),
            outgoing_queue: outgoing_queue,
            reactor: reactor,
        }
    }

    pub(crate) fn publish_message(&self, topic: IdentTopic, buffer: Vec<u8>) -> () {
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
        self.outgoing_queue.push((topic, out_buffer));
    }
}

impl Drop for Libp2pCustomNode {
    fn drop(&mut self) {
        if let Some(stop_sender) = self.stop_sender.take() {
            let _ = stop_sender.send(true);
        }
        self.reactor.block_on(async {
            if let Some(thread_handle) = self.thread_handle.take() {
                let _ = thread_handle.await;
            }
        });
    }
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
    let _ = unsafe { Box::from_raw(ptr) };
}
