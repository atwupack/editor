use crate::service::Service;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct MessageService {
    listeners: HashMap<TypeId, Vec<Box<Fn(&str, &dyn Any)>>>,
}

impl Service for MessageService {
    fn new() -> MessageService {
        MessageService {
            listeners: HashMap::new(),
        }
    }
    fn id() -> &'static str {
        "message-service"
    }
}

impl MessageService {

    pub fn connect<F: Fn(&I) -> O + 'static, I: Any, O: Any>(&mut self, f: F) {
    }

    pub fn send<M: Any>(&self, comp_id: &str, message: &M)
    {
        let type_id = TypeId::of::<M>();
        let recvs = &self.listeners[&type_id];
        for item in recvs.iter() {
            item(comp_id, message);
        }

    }

    fn register_int<F: Fn(&str, &dyn Any) + 'static>(&mut self, type_id: TypeId, f: F) {
        let mut recvs = self.listeners.remove(&type_id).unwrap_or_default();
        recvs.push(Box::new(f));
        self.listeners.insert(type_id, recvs);
    }

    pub fn register<F: Fn(&str, &M) + 'static, M: Any>(&mut self, f: F) {
        let type_id = TypeId::of::<M>();
        self.register_int(type_id, move |comp_id, message| {
            let cast_message = message.downcast_ref::<M>().unwrap();
            f(comp_id, cast_message);
        });
    }
}
