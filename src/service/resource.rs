use crate::service::{Service, ServiceFactory};
use crate::app::App;

pub struct ResourceService {}

impl Service for ResourceService {
    fn new(_sf: &mut ServiceFactory) -> Self {
        ResourceService {}
    }
}
