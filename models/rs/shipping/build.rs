// use std::vec;

// // use code_sign::sign;
// use kameo::prelude::*;
// use kameo::remote::dial_opts::DialOpts;

// struct UpdateComponentCache {
//     components: Vec<(String, Vec<u8>)>,
// }

// #[derive(Debug, Default, RemoteActor, Actor)]
// struct BuildScript;

// #[tokio::main]
fn main() {}
//     let actor = BuildScript::default();
//     let actor_ref = kameo::spawn(actor);
//     actor_ref.register("build_script");
//     let swarm = ActorSwarm::bootstrap().unwrap();
//     swarm
//         .listen_on(
//             "/ip4/0.0.0.0/udp/8020/quic-v1"
//                 .parse()
//                 .expect("failed to join swarm"),
//         )
//         .await
//         .unwrap();

//     let addr = DialOpts::unknown_peer_id().address(
//         "/ip4/ec2-50-18-21-144.us-west-1.compute.amazonaws.com/udp/8020/quic-v1"
//             .parse()
//             .unwrap(),
//     );
//     swarm.dial(addr.build()).await.unwrap();
//     let comp = std::fs::read(
//         "/Users/tysonmidboe/opnbook/taho/target/wasm32-wasip1/release/taho_model1.wasm",
//     )
//     .unwrap();
//     // let fabric =
//     RemoteActorRef::<_>::lookup("fabric_node")
//         .await
//         .unwrap()
//         .unwrap()
//         .tell(&UpdateComponentCache {
//             components: vec![("model1".to_owned(), comp)],
//         });
//     //sign("../../target/wasm32-wasip1/release/taho_model1.wasm");
// }
