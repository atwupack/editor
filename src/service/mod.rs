use std::cell::RefCell;
use std::clone::Clone;
use std::collections::HashMap;
use std::rc::Rc;

use downcast_rs::{impl_downcast, Downcast};

pub mod file;
pub mod message;
pub mod resource;

pub trait Service: Downcast {
    fn new() -> Self
    where
        Self: Sized + Clone;
    fn id() -> &'static str
    where
        Self: Sized + Clone;
}
impl_downcast!(Service);

#[derive(Clone)]
pub struct ServiceFactory {
    services: Rc<RefCell<HashMap<&'static str, Box<dyn Service>>>>,
}

impl ServiceFactory {
    pub fn get_service<T: Service + Clone>(&self) -> T {
        let id = T::id();
        let mut map = self.services.borrow_mut();
        let service = map.remove(&id).unwrap_or_else(|| {Box::new(T::new())});
        let cast_service: &T = service.as_ref().downcast_ref().unwrap();

        map.insert(id, Box::new(cast_service.clone()));
        cast_service.clone()
    }

    pub fn new() -> Self {
        ServiceFactory {
            services: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::service::message::MessageService;
    use crate::service::ServiceFactory;
    #[test]
    fn get_message_service() {
        let sr = ServiceFactory::new();
        let ms: MessageService = sr.get_service();
        ms.send("test-comp", "test-msg", &"test-obj");
    }
}
