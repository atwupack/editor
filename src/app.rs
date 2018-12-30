use crate::service::ServiceFactory;
use crate::service::Service;

#[derive(Clone)]
pub struct App {
    service_factory: ServiceFactory,
}

impl App {
    pub fn new() -> App {
        App {
            service_factory: ServiceFactory::new(),
        }
    }

    pub fn get_service<T: Service+Clone>(&self) -> T {
        self.service_factory.get_service()
    }
}