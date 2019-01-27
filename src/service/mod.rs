use std::collections::HashMap;

use downcast_rs::{impl_downcast, Downcast};

pub mod file;
pub mod message;
pub mod resource;

pub trait Service: Downcast {
    fn new() -> Self
    where
        Self: Sized;
    fn id() -> &'static str
    where
        Self: Sized;
}
impl_downcast!(Service);

pub struct ServiceFactory {
    services: HashMap<&'static str, Box<dyn Service>>,
}

impl ServiceFactory {
    pub fn get_service<T: Service>(&mut self) -> &mut T {
        let id = T::id();
        let service = self.services.remove(&id).unwrap_or_else(|| {Box::new(T::new())});

        self.services.insert(id, service);

        let new_service = self.services.get_mut(&id).unwrap().as_mut();
        let cast_service: &mut T = new_service.downcast_mut().unwrap();
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
        let ms: &mut MessageService = sr.get_service();
        //ms.send("test-comp", "test-msg", &"test-obj");
    }
}
