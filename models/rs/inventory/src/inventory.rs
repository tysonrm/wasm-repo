use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::bindings;
use crate::bindings::exports::taho::models::crud::{self, GuestCrudApi};
use crate::bindings::exports::taho::models::model::{
    self, Datasource, Direction, ModelSpec, Port as PortConfig, PortEvent, SpecError,
};
use crate::bindings::exports::taho::models::port::{self, GuestPorts, PortData, PortError};
use crate::bindings::exports::taho::models::subscriber;

#[derive(Default, Serialize, Deserialize)]
pub enum ItemStatus {
    #[default]
    InStock,
    OutOfStock,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Inventory {
    name: String,
    sku: String,
    status: ItemStatus,
}

impl model::Guest for Inventory {
    fn get_spec() -> Result<ModelSpec, SpecError> {
        let ports = vec![PortConfig {
            name: "pick_items".into(),
            adapters: vec![],
            direction: Direction::Inbound,
            consumes: vec![PortEvent {
                name: "inventory.pick_items".into(),
                wfid: "OrderUp@0.1.0".into(),
                wfstate: "".into(),
            }],
            produces: vec![
                PortEvent {
                    name: "order.in_stock".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "InStock".to_string(),
                },
                PortEvent {
                    name: "order.out_of_stock".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "OutOfStock".to_string(),
                },
            ],
            callback: "".to_string(),
            enabled: true,
            timeout: 0,
            retry: 0,
            interval: 0,
            circuit_breaker: false,
            retest: 0,
            undo: "".to_string(),
        }];

        Ok(ModelSpec {
            ports,
            name: "inventory".into(),
            domain: "order_up".into(),
            relations: vec![],
            datasource: Datasource {
                name: "surrealdb".to_string(),
                schema: serde_json::to_string_pretty(&Inventory::default()).unwrap_or_default(),
            },
            cache_enabled: true,
        })
    }
}

impl port::Guest for Inventory {
    type Ports = PortFunctions;
}

type PortFn = Box<dyn FnMut(PortData) -> Result<PortData, PortError> + Send + 'static>;

pub struct PortFunctions {
    functions: RefCell<HashMap<String, PortFn>>,
}

impl GuestPorts for PortFunctions {
    fn new() -> Self {
        let mut h = HashMap::new();
        h.insert(
            "pick_items".to_string(),
            Box::new(|mut data: PortData| {
                println!("pick_items");
                data.wfstate = "InStock".to_string();
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

impl crud::Guest for Inventory {
    type CrudApi = CrudFunctions;
}

pub struct CrudFunctions;

impl GuestCrudApi for CrudFunctions {
    fn new(config: crud::KeyValue) -> Self {
        let _ = config;
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

impl subscriber::Guest for Inventory {
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

bindings::export!(Inventory with_types_in bindings);
