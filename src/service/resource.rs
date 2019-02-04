use crate::service::Service;
use crate::app::App;

pub struct ResourceService {}

impl Service for ResourceService {
    fn new(_app: &App) -> Self {
        ResourceService {}
    }
}
