use std::collections::HashMap;

use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use crate::app::App;

pub mod file;
pub mod message;
pub mod resource;
pub mod task;

pub trait Service: Downcast {
    fn new(app: &App) -> Self
    where
        Self: Sized;
}
impl_downcast!(Service);

pub struct ServiceFactory {
    services: HashMap<TypeId, Box<dyn Service>>,
}

impl ServiceFactory {
    pub fn get_service<S: Service>(&mut self, app: &App) -> &mut S {
        let id = TypeId::of::<S>();
        if !self.services.contains_key(&id) {
            let new_service = Box::new(S::new(app));
            self.services.insert(id, new_service);
        }
        let service = self.services.get_mut(&id).unwrap().as_mut();
        let cast_service: &mut S = service.downcast_mut().unwrap();
        cast_service
    }

    pub fn new() -> Self {
        ServiceFactory {
            services: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::message::MessageService;
    use crate::service::ServiceFactory;
    #[test]
    fn get_message_service() {
        let mut sr = ServiceFactory::new();
        let _ms: &mut MessageService = sr.get_service();
        //ms.send("test-comp", "test-msg", &"test-obj");
    }
}
