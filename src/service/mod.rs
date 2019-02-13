use std::collections::HashMap;

use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use crate::app::App;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

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

#[derive(Clone)]
pub struct ServiceFactory {
    services: Rc<RefCell<HashMap<TypeId, Box<dyn Service>>>>,
}

impl ServiceFactory {

    fn has_type_id(&self, type_id: TypeId) -> bool {
        let services = self.services.borrow();
        services.contains_key(&type_id)
    }

    pub fn get_service<S: Service>(&self, app: &App) -> RefMut<S> {
        let id = TypeId::of::<S>();
        let known_service =self.has_type_id(id);
        if !known_service {
            let new_service = Box::new(S::new(app));
            let mut services=self.services.borrow_mut();
            services.insert(id, new_service);
        }

        let services=self.services.borrow_mut();
        RefMut::map(services, |services| {
            let service = services.get_mut(&id).unwrap().as_mut();
            let cast_service: &mut S = service.downcast_mut().unwrap();
            cast_service
        })
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
    use std::cell::RefMut;

    #[test]
    fn get_message_service() {
        let mut sr = ServiceFactory::new();
        //let _ms: RefMut<MessageService> = sr.get_service();
        //ms.send("test-comp", "test-msg", &"test-obj");
    }
}
