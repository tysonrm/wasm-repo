use std::{cell::RefCell, collections::HashMap, sync::Mutex, vec};

use serde::{Deserialize, Serialize};
use taho_macro::DeSerializeFrom;

use crate::bindings::{
    self,
    exports::taho::models::{
        crud::{self, GuestCrudApi},
        model::{self, Datasource, Direction, ModelSpec, Port as PortConfig, PortEvent, SpecError},
        port::{self, GuestPorts, PortData, PortError},
        subscriber,
    },
};

#[derive(Default, Serialize, Deserialize, DeSerializeFrom)]
pub enum OrderStatus {
    #[default]
    New,
    Submitted,
    Packaged,
    Shipping,
    Delivered,
    Returned,
    Cancelled,
}

impl From<OrderStatus> for String {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::New => "New".to_string(),
            OrderStatus::Submitted => "Submitted".to_string(),
            OrderStatus::Packaged => "Packaged".into(),
            OrderStatus::Shipping => "Shipping".to_string(),
            OrderStatus::Delivered => "Delivered".to_string(),
            OrderStatus::Returned => "Returned".to_string(),
            OrderStatus::Cancelled => "Cancelled".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, DeSerializeFrom)]
pub struct Order {
    items: Vec<String>,
    price: f32,
    status: OrderStatus,
    payment_approved: bool,
    delivery_accepted: bool,
}

impl model::Guest for Order {
    fn get_spec() -> Result<ModelSpec, SpecError> {
        let ports = vec![
            PortConfig {
                name: "submit".into(),
                adapters: vec![],
                direction: Direction::Inbound,
                consumes: vec![PortEvent {
                    name: "order.submit".into(),
                    wfid: "OrderUp@0.1.0".into(),
                    wfstate: "".to_string(),
                }],
                produces: vec![PortEvent {
                    name: "customer.find".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "New".into(),
                }],
                callback: "".into(),
                enabled: true,
                timeout: 0,
                retry: 0,
                interval: 0,
                circuit_breaker: false,
                retest: 0,
                undo: "".into(),
            },
            PortConfig {
                name: "customer_found".into(),
                adapters: vec![],
                direction: Direction::Inbound,
                consumes: vec![
                    PortEvent {
                        name: "order.customer_found".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "".to_string(),
                    },
                    PortEvent {
                        name: "order.customer_not_found".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "".to_string(),
                    },
                ],
                produces: vec![
                    PortEvent {
                        name: "order.reserve_payment".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "CustomerFound".to_string(),
                    },
                    PortEvent {
                        name: "order.cancel".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "CustomerNotFound".to_string(),
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
            },
            PortConfig {
                name: "reserve_payment".into(),
                adapters: vec!["stripe".into()],
                direction: Direction::Outbound,
                consumes: vec![PortEvent {
                    name: "order.reserve_payment".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "".to_string(),
                }],
                produces: vec![
                    PortEvent {
                        name: "inventory.pick_items".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "PaymentReserved".into(),
                    },
                    PortEvent {
                        name: "order.cancel".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "PaymentNotReserved".into(),
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
            },
            PortConfig {
                name: "cancel".into(),
                adapters: vec!["Email".into()],
                direction: Direction::Inbound,
                consumes: vec![PortEvent {
                    name: "order.cancel".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "PaymentReserved".to_string(),
                }],
                produces: vec![PortEvent {
                    name: "order.notify_customer".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "".to_string(),
                }],
                callback: "".into(),
                enabled: true,
                timeout: 0,
                retry: 0,
                interval: 0,
                circuit_breaker: false,
                retest: 0,
                undo: "".into(),
            },
            PortConfig {
                name: "in_stock".into(),
                adapters: vec![],
                direction: Direction::Inbound,
                consumes: vec![
                    PortEvent {
                        name: "order.in_stock".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "".to_string(),
                    },
                    PortEvent {
                        name: "order.out_of_stock".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "".to_string(),
                    },
                ],
                produces: vec![
                    PortEvent {
                        name: "order.ship".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "InStock".to_string(),
                    },
                    PortEvent {
                        name: "order.cancel".to_string(),
                        wfid: "OrderUp@0.1.0".to_string(),
                        wfstate: "OutOfStock".to_string(),
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
            },
            PortConfig {
                name: "ship".into(),
                adapters: vec!["fedex".into()],
                direction: Direction::Outbound,
                consumes: vec![PortEvent {
                    name: "order.ship".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "".to_string(),
                }],
                produces: vec![PortEvent {
                    name: "order.delivered".to_string(),
                    wfid: "OrderUp@0.1.0".to_string(),
                    wfstate: "Delivered".to_string(),
                }],
                callback: "".into(),
                enabled: true,
                timeout: 10,
                retry: 10,
                interval: 60,
                circuit_breaker: true,
                retest: 120,
                undo: "order.cancel_shipment".into(),
            },
        ];

        let ds = Datasource {
            name: "surrealdb".into(),
            schema: serde_json::to_string_pretty(&Order::default()).unwrap(),
        };

        Ok(ModelSpec {
            name: "order".into(),
            domain: "order_up".into(),
            ports,
            relations: vec![],
            datasource: ds,
            cache_enabled: true,
        })
    }
}

impl port::Guest for Order {
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
            "submit".to_owned(),
            Box::new(|mut data: PortData| {
                println!("submit ****");
                let order = Order::try_from(&data.data).unwrap_or_default();
                data.data = order.into();
                data.wfid = "OrderUp@0.1.0".into();
                data.wfstate = "New".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "customer_found".to_owned(),
            Box::new(|mut data: PortData| {
                println!("customer_found ****");
                data.wfstate = "CustomerFound".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "reserve_payment".to_owned(),
            Box::new(|mut data: PortData| {
                println!("reserve_payment ****");
                let mut order = Order::try_from(&data.data).unwrap_or_default();
                order.items.push("item1".into());
                order.payment_approved = true;
                data.data = order.into();
                data.wfstate = "PaymentReserved".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "in_stock".to_owned(),
            Box::new(|mut data: PortData| {
                println!("in_stock ****");
                data.wfstate = "InStock".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "ship".to_owned(),
            Box::new(|mut data: PortData| {
                println!("ship ****");
                data.wfstate = "Delivered".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "delivered".to_owned(),
            Box::new(|mut data: PortData| {
                println!("track shipment ****");
                let mut order = Order::try_from(&data.data).unwrap_or_default();
                order.delivery_accepted = true;
                data.data = order.into();
                data.wfstate = "Delivered".into();
                Ok(data)
            }) as PortFn,
        );
        h.insert(
            "cancel".to_owned(),
            Box::new(|mut data: PortData| {
                data.wfstate = "Cancelled".into();
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

impl crud::Guest for Order {
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

impl subscriber::Guest for Order {
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

bindings::export!(Order with_types_in bindings);
