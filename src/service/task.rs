use crate::service::{Service};
use crate::app::{App, AppContext};
use crate::service::message::MessageService;

#[derive(Clone)]
pub struct RunTask(pub &'static dyn Task);

#[derive(Clone)]
pub struct TaskService {
    app: App,
}

pub trait Task {
    fn run(&self, app: &App);
}

impl TaskService {
    fn run_task(&self, task: &dyn Task) {
        task.run(&self.app)
    }
}

impl Service for TaskService {
    fn new(ctx: &mut AppContext) -> Self {
        let ts = TaskService {
            app: ctx.app().clone(),
        };

        let ts_clone = ts.clone();
        let ms = ctx.get_service::<MessageService>();
        ms.register(move |_app, _comp_id, event: &RunTask | {
            println!("Run task");
            let RunTask(task) = event;
            ts_clone.run_task(*task);
        });

        ts
    }
}
