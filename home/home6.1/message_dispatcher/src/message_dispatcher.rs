use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::messages::{BlueMessage, GreenMessage, OrangeMessage};

pub type GreenMessageHandler = Arc<dyn Fn(&GreenMessage) + Send + Sync>;
pub type BlueMessageHandler = Arc<dyn Fn(&BlueMessage) + Send + Sync>;
pub type OrangeMessageHandler = Arc<dyn Fn(&OrangeMessage) + Send + Sync>;

pub struct MessageDispatcher {
    green_message_handlers: Mutex<HashMap<Uuid, GreenMessageHandler>>,
    blue_message_handlers: Mutex<HashMap<Uuid, BlueMessageHandler>>,
    orange_message_handlers: Mutex<HashMap<Uuid, OrangeMessageHandler>>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher {
            green_message_handlers: Mutex::new(HashMap::new()),
            blue_message_handlers: Mutex::new(HashMap::new()),
            orange_message_handlers: Mutex::new(HashMap::new()),
        }
    }

    pub fn subscribe_to_green_message(&self, handler: GreenMessageHandler) -> Uuid {
        let id = Uuid::new_v4();
        self.green_message_handlers.lock().unwrap().insert(id, handler);
        id
    }

    pub fn subscribe_to_blue_message(&self, handler: BlueMessageHandler) -> Uuid {
        let id = Uuid::new_v4();
        self.blue_message_handlers.lock().unwrap().insert(id, handler);
        id
    }

    pub fn subscribe_to_orange_message(&self, handler: OrangeMessageHandler) -> Uuid {
        let id = Uuid::new_v4();
        self.orange_message_handlers.lock().unwrap().insert(id, handler);
        id
    }

    pub fn unsubscribe(&self, id: Uuid) {
        self.green_message_handlers.lock().unwrap().remove(&id);
        self.blue_message_handlers.lock().unwrap().remove(&id);
        self.orange_message_handlers.lock().unwrap().remove(&id);
    }

    pub fn publish_green_message(&self, message: GreenMessage) {
        for handler in self.green_message_handlers.lock().unwrap().values() {
            handler(&message);
        }
    }

    pub fn publish_blue_message(&self, message: BlueMessage) {
        for handler in self.blue_message_handlers.lock().unwrap().values() {
            handler(&message);
        }
    }

    pub fn publish_orange_message(&self, message: OrangeMessage) {
        for handler in self.orange_message_handlers.lock().unwrap().values() {
            handler(&message);
        }
    }
}
