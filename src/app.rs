use crate::service::ServiceFactory;

#[derive(Clone)]
pub struct App {
    pub service_factory: ServiceFactory,
}

impl App {
    pub fn new() -> App {
        App {
            service_factory: ServiceFactory::new(),
        }
    }


}