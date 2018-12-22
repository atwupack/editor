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
        if !callbacks.contains_key(message_id) {
            println!("Nothing to do");
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