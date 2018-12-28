use crate::service::ServiceRegistry;

#[derive(Clone)]
pub struct App {
    pub service_registry: ServiceRegistry,
}

impl App {
    pub fn new() -> App {
        App {
            service_registry: ServiceRegistry::new(),
        }
    }


}