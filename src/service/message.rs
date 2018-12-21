use downcast_rs::{Downcast, impl_downcast};
use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;

trait MessageObject: Downcast {}
impl_downcast!(MessageObject);


#[derive(Clone)]
pub struct MessageService {
    listeners: Rc<RefCell<Vec<Box<Fn(&str, &str, &Any) + 'static>>>>,
}

impl MessageService {

    pub fn new() -> MessageService {
        MessageService {
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn send(&self, comp_id: &str, message_id: &str, message_obj: &Any) {
        let callbacks = self.listeners.borrow();
        for item in callbacks.iter() {
            item(comp_id, message_id, message_obj);
        }
    }

    pub fn register<F: Fn(&str, &str, &Any) + 'static>(&self, message_id: &str, f: F) {
        self.listeners.borrow_mut().push(Box::new(f));
    }
}