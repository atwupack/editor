use crate::service::Service;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct MessageService {
    listeners: HashMap<TypeId, Vec<Box<Fn(&str, &dyn Any)>>>,
    connections: HashMap<TypeId, HashMap<TypeId, Box<Fn(&dyn Any) -> Box<dyn Any>>>>,
}

impl Service for MessageService {
    fn new() -> MessageService {
        MessageService {
            listeners: HashMap::new(),
            connections: HashMap::new(),
        }
    }
}

impl MessageService {

    pub fn connect<F: Fn(&I) -> O + 'static, I: Any, O: Any>(&mut self, f: F) {
        let type_id_in = TypeId::of::<I>();
        let type_id_out = TypeId::of::<O>();
        self.add_connection(type_id_in, type_id_out, move |message_in| {
            let cast_message_in: &I = message_in.downcast_ref().unwrap();
            Box::new(f(cast_message_in))
        });
    }

    fn add_connection<F: Fn(&dyn Any) -> Box<dyn Any> + 'static>(&mut self, type_id_in: TypeId, type_id_out: TypeId, f: F) {
        let mut cons = self.connections.remove(&type_id_in).unwrap_or_default();
        cons.insert(type_id_out, Box::new(f));
        self.connections.insert(type_id_in, cons);
    }

    pub fn send<M: Any>(&self, comp_id: &str, message: &M)
    {
        let type_id = TypeId::of::<M>();
        self.notify_listeners(comp_id, type_id, message);

        let cons = &self.connections.get(&type_id);
        if cons.is_some() {
            let h = cons.unwrap();
            for (type_id_out, item) in h.iter() {
                let message_out = item(message);
                self.notify_listeners(comp_id, *type_id_out, message_out.as_ref());
            }
        }
    }

    fn notify_listeners(&self, comp_id: &str, type_id: TypeId, message: &dyn Any) {
        let recvs = self.listeners.get(&type_id);
        if recvs.is_some() {
            let v = recvs.unwrap();
            for item in v.iter() {
                item(comp_id, message);
            }
        }
    }

    fn add_listener<F: Fn(&str, &dyn Any) + 'static>(&mut self, type_id: TypeId, f: F) {
        let mut recvs = self.listeners.remove(&type_id).unwrap_or_default();
        recvs.push(Box::new(f));
        self.listeners.insert(type_id, recvs);
    }

    pub fn register<F: Fn(&str, &M) + 'static, M: Any>(&mut self, f: F) {
        let type_id = TypeId::of::<M>();
        self.add_listener(type_id, move |comp_id, message| {
            let cast_message: &M = message.downcast_ref().unwrap();
            f(comp_id, cast_message);
        });
    }
}
