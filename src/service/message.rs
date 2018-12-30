use crate::service::Service;
use downcast_rs::{impl_downcast, Downcast};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct MessageService {
    listeners: Rc<RefCell<HashMap<&'static str, Vec<Box<Fn(&str, &str, &Any) + 'static>>>>>,
}

impl Service for MessageService {
    fn new() -> MessageService {
        MessageService {
            listeners: Rc::new(RefCell::new(HashMap::new())),
        }
    }
    fn id() -> &'static str {
        "message-service"
    }
}

impl MessageService {
    pub fn send(&self, comp_id: &str, message_id: &str, message_obj: &Any) {
        let callbacks = self.listeners.borrow();
        if !callbacks.contains_key(message_id) {
            return;
        }
        let recvs = callbacks.get(message_id).unwrap();
        for item in recvs.iter() {
            item(comp_id, message_id, message_obj);
        }
    }

    pub fn register<F: Fn(&str, &str, &Any) + 'static>(&self, message_id: &'static str, f: F) {
        let mut list = self.listeners.borrow_mut();
        let mut recvs = list.remove(message_id).unwrap_or(Vec::new());
        recvs.push(Box::new(f));
        list.insert(message_id, recvs);
    }
}
