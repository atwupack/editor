use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::clone::Clone;

use downcast_rs::{Downcast, impl_downcast};

pub mod message;
pub mod file;
pub mod resource;

pub trait Service: Downcast {
    fn new() -> Self where Self: Sized+Clone;
    fn id() -> &'static str where Self: Sized+Clone;
}
impl_downcast!(Service);

#[derive(Clone)]
pub struct ServiceRegistry {
    services: Rc<RefCell<HashMap<&'static str, Box<dyn Service>>>>,
}

impl ServiceRegistry {
    pub fn get_service<T: Service+Clone>(&self) -> T {
        let id = T::id();
        let mut map = self.services.borrow_mut();
        let service = map.remove(&id).unwrap_or(Box::new(T::new()));
        let cast_service: &T = service.as_ref().downcast_ref().unwrap();

        map.insert(id, Box::new(cast_service.clone()));
        cast_service.clone()
    }

    pub fn new() -> Self {
        ServiceRegistry {
            services: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::ServiceRegistry;
    use crate::service::message::MessageService;
    #[test]
    fn get_message_service() {
        let sr = ServiceRegistry::new();
        let ms :MessageService = sr.get_service();
        ms.send("test-comp", "test-msg", &"test-obj");
    }
}