use crate::service::Service;

#[derive(Clone)]
pub struct ResourceService {

}

impl Service for ResourceService {
    fn new() -> Self {
        ResourceService {}
    }

    fn id() -> &'static str {
        "resource-service"
    }
}