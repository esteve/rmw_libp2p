// pub struct Libp2pCustomNode {
//     thread_handle: task::JoinHandle<()>,
//     stop_sender: oneshot::Sender<bool>,
//     outgoing_queue: Arc<deadqueue::unlimited::Queue<(Topic, Vec<u8>)>>,
// }

// pub struct Libp2pCustomPublisher {
//     gid: Uuid,
//     node: Arc<Libp2pCustomNode>, // We need to store the Node here to have access to the outgoing queue
//     topic: Topic,
// }

// impl Libp2pCustomNode {
//     fn new() -> Self {
//         let keypair = identity::Keypair::generate_ed25519();

//         let peer_id = PeerId::from(keypair.public());

//         let transport = task::block_on(libp2p::development_transport(keypair.clone())).unwrap();

//         let mut swarm = {
//             let message_id_fn = |message: &GossipsubMessage| {
//                 let mut s = DefaultHasher::new();
//                 message.data.hash(&mut s);
//                 MessageId::from(s.finish().to_string())
//             };

//             let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
//                 .heartbeat_interval(Duration::from_secs(10))
//                 .validation_mode(ValidationMode::Strict)
//                 .message_id_fn(message_id_fn)
//                 // same content will be propagated.
//                 .build()
//                 .expect("Valid config");

//             let gossipsub: gossipsub::Gossipsub =
//                 gossipsub::Gossipsub::new(MessageAuthenticity::Signed(keypair), gossipsub_config)
//                     .expect("Correct configuration");

//             let mdns = task::block_on(Mdns::new(MdnsConfig::default())).unwrap();

//             let behaviour = RosNetworkBehaviour {
//                 gossipsub: gossipsub,
//                 mdns: mdns,
//             };

//             libp2p::Swarm::new(transport, behaviour, peer_id)
//         };

//         let (stop_sender, mut stop_receiver) = oneshot::channel::<bool>();

//         let outgoing_queue = Arc::new(deadqueue::unlimited::Queue::<(Topic, Vec<u8>)>::new());

//         let outgoing_queue_clone = Arc::clone(&outgoing_queue);

//         let thread_handle = task::spawn(async move {
//             loop {
//                 select! {
//                     // use a oneshot future that will be triggered to stop the swarm
//                     // select! will wait on any future
//                     _ = stop_receiver => {
//                         println!("Exit loop");
//                     },
//                     // pop messages from the queue and publish them to the network
//                     (topic, buffer) = outgoing_queue_clone.pop().fuse() => {
//                         swarm.behaviour_mut().gossipsub.publish(topic, buffer);
//                     },
//                     event = swarm.select_next_some() => match event {
//                         SwarmEvent::Behaviour(OutEvent::Gossipsub(GossipsubEvent::Message {
//                             propagation_source: peer_id,
//                             message_id: id,
//                             message,
//                         })) => {
//                             println!(
//                                 "Got message: {} with id: {} from peer: {:?}",
//                                 String::from_utf8_lossy(&message.data),
//                                 id,
//                                 peer_id
//                             );
//                         }
//                         SwarmEvent::NewListenAddr { address, .. } => {
//                             println!("Listening on {:?}", address);
//                         }
//                         SwarmEvent::Behaviour(OutEvent::Mdns(
//                             MdnsEvent::Discovered(list)
//                         )) => {
//                             for (peer, _) in list {
//                                 swarm
//                                     .behaviour_mut()
//                                     .gossipsub
//                                     .add_explicit_peer(&peer);
//                             }
//                         }
//                         SwarmEvent::Behaviour(OutEvent::Mdns(MdnsEvent::Expired(
//                             list
//                         ))) => {
//                             for (peer, _) in list {
//                                 if !swarm.behaviour_mut().mdns.has_node(&peer) {
//                                     swarm
//                                         .behaviour_mut()
//                                         .gossipsub
//                                         .remove_explicit_peer(&peer);
//                                 }
//                             }
//                         },
//                         _ => {}
//                     }
//                 }
//             }
//         });

//         Self {
//             thread_handle: thread_handle,
//             stop_sender: stop_sender,
//             outgoing_queue: outgoing_queue,
//         }
//     }

//     fn publish_message(&self, topic: Topic, buffer: Vec<u8>) -> () {
//         self.outgoing_queue.push((topic, buffer));
//     }
// }

// impl Libp2pCustomPublisher {
//     fn new(libp2p2_custom_node: Arc<Libp2pCustomNode>, topic_str: &str) -> Self {
//         Self {
//             gid: Uuid::new_v4(),
//             node: libp2p2_custom_node,
//             topic: Topic::new(topic_str),
//         }
//     }

//     fn publish(&self, buffer: Vec<u8>) -> () {
//         self.node.publish_message(self.topic.clone(), buffer);
//     }
// }
