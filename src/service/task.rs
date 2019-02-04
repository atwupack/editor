use crate::service::Service;
use crate::app::App;
use crate::service::message::MessageService;

pub struct RunTask(&'static dyn Task);

#[derive(Clone)]
pub struct TaskService {
    app: App,
}


pub trait Task {
    fn run(&self, app: &App);
}

impl TaskService {
    fn run_task(&self, task: &dyn Task) {

    }
}

impl Service for TaskService {
    fn new(app: &App) -> Self {
        let ts = TaskService {
            app: app.clone(),
        };

        let ts_clone = ts.clone();
        let mut ms = app.get_service::<MessageService>();
        ms.register(move |_comp_id, event: &RunTask | {
            let RunTask(task) = event;
            ts_clone.run_task(*task);
        });

        ts
    }
}