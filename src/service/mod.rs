use std::collections::HashMap;

use downcast_rs::{impl_downcast, Downcast};
use std::any::TypeId;
use crate::app::{App, AppContext};

pub mod file;
pub mod message;
pub mod resource;
pub mod task;

pub trait Service: Downcast {
    fn new(ctx: &mut AppContext) -> Self
    where
        Self: Sized;
}
impl_downcast!(Service);

pub struct ServiceFactory {
    services: HashMap<TypeId, Box<dyn Service>>,
}

impl ServiceFactory {

    fn has_type_id(&self, type_id: TypeId) -> bool {
        self.services.contains_key(&type_id)
    }

    pub fn get_service<S: Service>(&mut self, app: &App) -> &mut S {
        let id = TypeId::of::<S>();
        let known_service =self.has_type_id(id);
        if !known_service {
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
    use crate::service::task::TaskService;
    use crate::service::message::MessageService;
    use crate::service::ServiceFactory;
    use crate::app::App;
    use std::cell::RefMut;

    #[test]
    fn get_simple_service() {
        let app = App::new();
        let mut sr = ServiceFactory::new();
        let _ms = sr.get_service::<MessageService>(&app);
    }

    #[test]
    fn get_service_using_other_aervice() {
        let app = App::new();
        let mut sr = ServiceFactory::new();
        let _ts = sr.get_service::<TaskService>(&app);
    }
}
