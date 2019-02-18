use crate::service::{Service};
use crate::app::AppContext;

pub struct ResourceService {}

impl Service for ResourceService {
    fn new(_ctx: &mut AppContext) -> Self {
        ResourceService {}
    }
}
