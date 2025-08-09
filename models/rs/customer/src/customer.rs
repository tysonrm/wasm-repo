use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use taho_macro::DeSerializeFrom;

use crate::bindings;
use crate::bindings::exports::taho::models::crud::GuestCrudApi;
use crate::bindings::exports::taho::models::model::{
    self, Datasource, Direction, ModelSpec, Port as PortConfig, PortEvent, SpecError,
};
use crate::bindings::exports::taho::models::port::{self, GuestPorts, PortData, PortError};
use crate::bindings::exports::taho::models::{crud, subscriber};

#[derive(Default, Debug, Serialize, Deserialize, DeSerializeFrom)]
pub struct Customer {
    first_name: String,
    last_name: String,
    email_addr: String,
    credit_card: String,
    avatar: Option<Vec<u8>>,
}

impl model::Guest for Customer {
    fn get_spec() -> Result<ModelSpec, SpecError> {
        let ports = vec![PortConfig {
            name: "find".into(),
            adapters: vec![],
            direction: Direction::Inbound,
            consumes: vec![PortEvent {
                name: "customer.find".into(),
                wfid: "OrderUp@0.1.0".into(),
                wfstate: "New".into(),
            }],
            produces: vec![
                PortEvent {
                    name: "order.customer_found".into(),
                    wfid: "OrderUp@0.1.0".into(),
                    wfstate: "CustomerFound".into(),
                },
                PortEvent {
                    name: "order.customer_not_found".into(),
                    wfid: "OrderUp@0.1.0".into(),
                    wfstate: "CustomerNotFound".into(),
                },
                PortEvent {
                    name: "order.customer_invalid".into(),
                    wfid: "OrderUp@0.1.0".into(),
                    wfstate: "CustomerInvalid".into(),
                },
            ],
            callback: "".into(),
            enabled: true,
            timeout: 0,
            retry: 0,
            interval: 0,
            circuit_breaker: false,
            retest: 0,
            undo: "".into(),
        }];

        Ok(ModelSpec {
            name: "customer".into(),
            domain: "order_up".into(),
            ports,
            relations: vec![],
            datasource: Datasource {
                name: "surrealdb".to_string(),
                schema: serde_json::to_string_pretty(&Customer::default()).unwrap_or_default(),
            },
            cache_enabled: true,
        })
    }
}

impl port::Guest for Customer {
    type Ports = PortFunctions;
}

type PortFn = Box<dyn FnMut(PortData) -> Result<PortData, PortError> + Send + 'static>;

pub struct PortFunctions {
    functions: RefCell<HashMap<String, PortFn>>,
}

impl From<serde_json::Error> for PortError {
    fn from(value: serde_json::Error) -> Self {
        PortError::Error(value.to_string())
    }
}

impl GuestPorts for PortFunctions {
    fn new() -> Self {
        let mut h = HashMap::new();
        h.insert(
            "find".to_string(),
            Box::new(|mut data: port::PortData| {
                println!("customer.find ****");
                let mut customer = Customer::try_from(&data.data).unwrap_or_default();
                println!("{customer:?}");
                customer.last_name = "Smith".into();
                data.wfstate = "CustomerFound".into();
                data.data = customer.into();
                Ok(data)
            }) as PortFn,
        );
        Self {
            functions: RefCell::new(h),
        }
    }

    fn invoke_port(&self, name: String, data: PortData) -> Result<PortData, PortError> {
        (self
            .functions
            .borrow_mut()
            .get_mut(&name)
            .unwrap_or(&mut (Box::new(|data| Ok(data)) as PortFn)))(data)
    }
}

impl crud::Guest for Customer {
    type CrudApi = CrudFunctions;
}

pub struct CrudFunctions;

impl GuestCrudApi for CrudFunctions {
    fn new(config: crud::KeyValue) -> Self {
        Self {}
    }

    fn create(&self, query: String, data: Vec<u8>) -> Result<Vec<u8>, crud::CrudError> {
        Ok(data)
    }

    fn read(&self, query: String) -> Result<Vec<u8>, crud::CrudError> {
        Ok(vec![])
    }

    fn update(&self, query: String, data: Vec<u8>) -> Result<i32, crud::CrudError> {
        Ok(1)
    }

    fn delete(&self, query: String) -> Result<i32, crud::CrudError> {
        Ok(1)
    }
}

impl subscriber::Guest for Customer {
    type Subscription = SubscriberFunctions;
}

pub struct SubscriberFunctions;

impl subscriber::GuestSubscription for SubscriberFunctions {
    fn new() -> Self {
        Self {}
    }

    fn subscribe(&self) -> Vec<subscriber::Topic> {
        vec![]
    }

    fn handle_event(&self, event: String, message: Vec<u8>) -> () {
        println!("{event} {}", serde_json::from_slice(&message).unwrap_or(""));
    }
}

bindings::export!(Customer with_types_in bindings);
