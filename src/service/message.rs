use crate::service::Service;
use std::any::Any;
use std::collections::HashMap;

pub struct MessageService {
    listeners: HashMap<&'static str, Vec<Box<Fn(&str, &str, &Any)>>>,
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

//    pub fn connect(&mut self, source_msg_id: &'static str, target_msg_id: &'static str) {
//        //let msg_service_clone = self.clone();
//        self.register(source_msg_id, |_,_,obj| {
//            self.send(MessageService::id(), target_msg_id, obj);
//        })
//    }

    pub fn send(&self, comp_id: &str, message_id: &str, message_obj: &Any) {
        if !self.listeners.contains_key(message_id) {
            return;
        }
        let recvs = &self.listeners[message_id];
        for item in recvs.iter() {
            item(comp_id, message_id, message_obj);
        }
    }

    pub fn register<F: Fn(&str, &str, &Any) + 'static>(&mut self, message_id: &'static str, f: F) {
        let mut recvs = self.listeners.remove(message_id).unwrap_or_default();
        recvs.push(Box::new(f));
        self.listeners.insert(message_id, recvs);
    }
}
