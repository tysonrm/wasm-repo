use bindings::exports::taho::models::crud::{Guest as GuestCrud, GuestCrudApi, KeyValue};
use bindings::exports::taho::models::model::{
    Directions, Guest as GuestModel, ModelSpec, Port, PortEvent, SpecError,
};
use bindings::exports::taho::models::port::{
    Guest as GuestPort, GuestPorts as GuestPortResource, PortError,
};
use bindings::exports::taho::models::subscriber::{Guest as GuestSubscriber, GuestSubscription};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::time::Instant;
use taho_macro::DeSerializeFrom;

use crate::bindings;
use crate::bindings::exports::taho::models::crud::CrudError;
use crate::bindings::exports::taho::models::model::Datasource;

type PortFn = Box<dyn FnMut(Vec<u8>) -> Result<Vec<u8>, PortError>>;
#[derive(Debug, Serialize, Deserialize, DeSerializeFrom)]
enum Recurrence {
    Daily,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, DeSerializeFrom)]
struct InitialData {
    pub name: String,
    pub description: String,
    pub done: bool,
    pub recurrence: Recurrence,
    pub rating: i32,
}

fn port1(_data: Vec<u8>) -> Result<Vec<u8>, PortError> {
    let data = InitialData {
        name: "Send email to Adrian".into(),
        description: "Confirm if you have passed the subject\nHereby ..".into(),
        done: true,
        recurrence: Recurrence::Daily,
        rating: 3,
    };

    Ok(data.into())
}

fn port2(_data: Vec<u8>) -> Result<Vec<u8>, PortError> {
    println!("\n 🚀 port2 was invoked 🦀\n");
    Ok(b"port2".to_vec())
}

#[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
struct PortData {
    field1: String,
    field2: i32,
}

fn port3(data: Vec<u8>) -> Result<Vec<u8>, PortError> {
    let port3_data = PortData::from(&data);
    dbg!(&port3_data);
    let html: &str = r#"<html><div style="text-align:center"><h1>&#128512;</h1><h2>port3</h2>
                        <button onclick="document.getElementById('time').innerHTML = Date()">
                        click me</button><p id="time"></p></div></html>"#;
    let bytes: &[u8] = html.as_bytes();
    println!("\n 🚀 port3 was invoked 🦀\n");
    let p3: Vec<u8> = port3_data.into();
    println!("data = {:?}", p3);
    Ok(bytes.to_vec())
}

#[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
struct Fibonacci {
    nth: i32,
    result: i64,
    duration: f64,
}

fn fibonacci(value: Vec<u8>) -> Result<Vec<u8>, PortError> {
    let mut f = Fibonacci::from(&value);
    let start = Instant::now();
    f.result = fib(f.nth);
    f.duration = start.elapsed().as_secs_f64();
    Ok(f.into())
}

fn fib(n: i32) -> i64 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

pub struct Component {}

impl GuestModel for Component {
    fn get_spec() -> Result<ModelSpec, SpecError> {
        let port1_event1 = PortEvent {
            name: "model1.port1".to_owned(),
            wfid: "order-system@0.1.0".to_owned(),
            state: "default".to_owned(),
        };

        let port = Port {
            name: "port1".to_string(),
            adapters: vec![],
            consumes: vec![port1_event1],
            produces: vec![],
            direction: Directions::Inbound,
            undo: "".to_string(),
            callback: "".to_string().into(),
            enabled: true,
            timeout: 0,
            retry: 0,
            interval: 0,
            circuit_breaker: false,
            retest: 0,
        };

        Ok(ModelSpec {
            name: "model1".to_owned(),
            domain: "domain1".to_owned(),
            endpoint: "model1s".to_owned(),
            ports: vec![port],
            relations: vec![],
            datasource: Datasource {
                name: "native_datasource".to_string(),
                schema: r#" {
                    field1:'foo',
                    field2: 42,
                } "#
                .to_owned(),
            },
            cache_enabled: true,
        })
    }
}

pub struct PortResource {
    pub functions: RefCell<Vec<(String, PortFn)>>,
}

impl GuestPortResource for PortResource {
    fn new() -> Self {
        PortResource {
            functions: RefCell::new(vec![
                ("port1".to_string(), Box::new(port1)),
                ("port2".to_string(), Box::new(port2)),
                ("port3".to_string(), Box::new(port3)),
                ("fibonacci".to_string(), Box::new(fibonacci)),
            ]),
        }
    }

    fn invoke_port(&self, port: String, data: Vec<u8>) -> Result<Vec<u8>, PortError> {
        if Some(&port).is_none() {
            return Err(PortError::Error("no port provided".to_string()));
        }
        let mut ports = self.functions.borrow_mut();
        let portfn = ports.iter_mut().find(|p| p.0 == port).unwrap();
        if Some(&portfn).is_none() {
            return Err(PortError::Error("port not found".to_string()));
        }
        (portfn.1)(data)
    }
}

impl GuestPort for Component {
    type Ports = PortResource;
}

pub struct SubscriptionResource {}

impl GuestSubscription for SubscriptionResource {
    fn new() -> Self {
        Self {}
    }

    fn subscribe(&self) -> Vec<bindings::exports::taho::models::subscriber::Topic> {
        vec!["*".to_string()]
    }

    fn handle_event(&self, event: String, message: Vec<u8>) {
        println!("event published {:?}: {:?}", event, message)
    }
}

impl GuestSubscriber for Component {
    type Subscription = SubscriptionResource;
}

pub struct CrudResource {}

#[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
pub struct ModelData {
    field1: String,
    field2: i64,
}

impl GuestCrudApi for CrudResource {
    fn new(_config: KeyValue) -> Self {
        Self {}
    }

    fn create(&self, _query: String, data: Vec<u8>) -> Result<Vec<u8>, CrudError> {
        println!("model.create({:?})", &data);
        Ok(data)
    }

    fn read(&self, _query: String) -> Result<Vec<u8>, CrudError> {
        let data = InitialData {
            name: "Send email to Adrian".into(),
            description: "Confirm if you have passed the subject\nHereby ..".into(),
            done: true,
            recurrence: Recurrence::Daily,
            rating: 3,
        };

        Ok(data.into())
    }

    fn update(&self, _query: String, _data: Vec<u8>) -> Result<i32, CrudError> {
        Ok(1)
    }

    fn delete(&self, _query: String) -> Result<i32, CrudError> {
        Ok(1)
    }
}

impl GuestCrud for Component {
    type CrudApi = CrudResource;
}

bindings::export!(Component with_types_in bindings);

// use bindings::exports::taho::models::crud::{Guest as GuestCrud, GuestCrudApi, KeyValue};
// use bindings::exports::taho::models::model::{
//     Directions, Guest as GuestModel, ModelSpec, Port, PortEvent, SpecError,
// };
// use bindings::exports::taho::models::port::{
//     Guest as GuestPort, GuestPorts as GuestPortResource, PortError,
// };
// use bindings::exports::taho::models::subscriber::{Guest as GuestSubscriber, GuestSubscription};
// use serde::{Deserialize, Serialize};
// use std::cell::RefCell;
// use std::time::Instant;
// use taho_macro::DeSerializeFrom;

// use crate::bindings;
// use crate::bindings::exports::taho::models::crud::CrudError;

// type PortFn = Box<dyn FnMut(Vec<u8>) -> Result<Vec<u8>, PortError>>;

// fn port1(data: Vec<u8>) -> Result<Vec<u8>, PortError> {
//     println!("\n 😎 port1 was invoked 🦀 \n");
//     dbg!(&data);
//     println!("data = {}", String::from_utf8(data).unwrap());
//     Ok(b"port1".to_vec())
// }

// fn port2(data: Vec<u8>) -> Result<Vec<u8>, PortError> {
//     println!("\n 🚀 port2 was invoked 🦀\n");
//     Ok(b"port2".to_vec())
// }

// #[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
// struct PortData {
//     field1: String,
//     field2: i32,
// }

// fn port3(data: Vec<u8>) -> Result<Vec<u8>, PortError> {
//     let port3_data = PortData::from(&data);
//     dbg!(&port3_data);
//     let html: &str = r#"<html><div style="text-align:center"><h1>&#128512;</h1><h2>port3</h2>
//                         <button onclick="document.getElementById('time').innerHTML = Date()">
//                         click me</button><p id="time"></p></div></html>"#;
//     let bytes: &[u8] = html.as_bytes();
//     println!("\n 🚀 port3 was invoked 🦀\n");
//     let p3: Vec<u8> = port3_data.into();
//     println!("data = {:?}", p3);
//     Ok(bytes.to_vec())
// }

// #[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
// struct Fibonacci {
//     nth: i32,
//     result: i64,
//     duration: f64,
// }

// fn fibonacci(value: Vec<u8>) -> Result<Vec<u8>, PortError> {
//     let mut f = Fibonacci::from(&value);
//     let start = Instant::now();
//     f.result = fib(f.nth);
//     f.duration = start.elapsed().as_secs_f64();
//     Ok(f.into())
// }

// fn fib(n: i32) -> i64 {
//     if n <= 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }

// pub struct Component {}

// impl GuestModel for Component {
//     fn get_spec() -> Result<ModelSpec, SpecError> {
//         let port1_event1 = PortEvent {
//             name: "model1.port1".to_owned(),
//             wfid: "order-system@0.1.0".to_owned(),
//             state: "default".to_owned(),
//         };

//         let port = Port {
//             name: "port1".to_string(),
//             adapter: None,
//             consumes: vec![port1_event1],
//             produces: vec![],
//             direction: Directions::Inbound,
//             undo: "".to_string(),
//             callback: "".to_string().into(),
//             enabled: true,
//             timeout: 0,
//             retry: 0,
//             interval: 0,
//             circuit_breaker: false,
//             retest: 0,
//         };

//         Ok(ModelSpec {
//             name: "model1".to_owned(),
//             domain: "domain1".to_owned(),
//             endpoint: "model1s".to_owned(),
//             ports: vec![port],
//             relations: vec![],
//             datasource: "native_datasource".to_string(),
//         })
//     }
// }

// pub struct PortResource {
//     pub functions: RefCell<Vec<(String, PortFn)>>,
// }

// impl GuestPortResource for PortResource {
//     fn new() -> Self {
//         PortResource {
//             functions: RefCell::new(vec![
//                 ("port1".to_string(), Box::new(port1)),
//                 ("port2".to_string(), Box::new(port2)),
//                 ("port3".to_string(), Box::new(port3)),
//                 ("fibonacci".to_string(), Box::new(fibonacci)),
//             ]),
//         }
//     }

//     fn invoke_port(&self, port: String, data: Vec<u8>) -> Result<Vec<u8>, PortError> {
//         if Some(&port).is_none() {
//             return Err(PortError::Error("no port provided".to_string()));
//         }
//         let mut ports = self.functions.borrow_mut();
//         let portfn = ports.iter_mut().find(|p| p.0 == port).unwrap();
//         if Some(&portfn).is_none() {
//             return Err(PortError::Error("port not found".to_string()));
//         }
//         (portfn.1)(data)
//     }
// }

// impl GuestPort for Component {
//     type Ports = PortResource;
// }

// pub struct SubscriptionResource {}

// impl GuestSubscription for SubscriptionResource {
//     fn new() -> Self {
//         Self {}
//     }

//     fn subscribe(&self) -> Vec<bindings::exports::taho::models::subscriber::Topic> {
//         vec!["*".to_string()]
//     }

//     fn callback(&self, topic: String, message: String) {
//         print!("suscriber called with topic {}, message {}", topic, message);
//     }
// }

// impl GuestSubscriber for Component {
//     type Subscription = SubscriptionResource;
// }

// pub struct CrudResource {}

// #[derive(Serialize, Deserialize, DeSerializeFrom, Debug)]
// pub struct ModelData {
//     field1: String,
//     field2: i64,
// }

// impl GuestCrudApi for CrudResource {
//     fn new(_config: KeyValue) -> Self {
//         Self {}
//     }

//     fn create(&self, _query: String, data: Vec<u8>) -> Result<Vec<u8>, CrudError> {
//         println!("model.create({:?})", &data);
//         Ok(data)
//     }

//     fn read(&self, _query: String) -> Result<Vec<u8>, CrudError> {
//         Ok(b"foo".to_vec())
//     }

//     fn update(&self, _query: String, _data: Vec<u8>) -> Result<i32, CrudError> {
//         Ok(1)
//     }

//     fn delete(&self, _query: String) -> Result<i32, CrudError> {
//         Ok(1)
//     }
// }

// impl GuestCrud for Component {
//     type CrudApi = CrudResource;
// }

// bindings::export!(Component with_types_in bindings);
