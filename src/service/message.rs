use downcast_rs::{Downcast, impl_downcast};
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

trait MessageObject: Downcast {}
impl_downcast!(MessageObject);

#[derive(Clone)]
pub struct MessageService {
    listeners: Rc<RefCell<HashMap<&'static str, Vec<Box<Fn(&str, &str, &Any) + 'static>>>>>,
}

impl MessageService {

    pub fn new() -> MessageService {
        MessageService {
            listeners: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn send(&self, comp_id: &str, message_id: &str, message_obj: &Any) {
        let callbacks = self.listeners.borrow();
        for item in callbacks.iter() {
//            item(comp_id, message_id, message_obj);
            println!("Test")
        }
    }

    pub fn register<F: Fn(&str, &str, &Any) + 'static>(&self, message_id: &str, f: F) {
        let mut list = self.listeners.borrow_mut();
        let recvs = list.(message_id).unwrap_or(Vec::new());
        recvs.push(Box::new(f));
    }
}