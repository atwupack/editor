use crate::service::Service;
use std::any::TypeId;
use std::collections::HashMap;

pub struct Task {

}

pub struct TaskService {
    tasks: HashMap<TypeId, Task>,
}

impl Service for TaskService {
    fn new() -> Self {
        TaskService {
            tasks: HashMap::new(),
        }
    }
}
