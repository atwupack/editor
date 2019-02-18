use crate::service::{Service, ServiceFactory};
use crate::service::message::MessageService;
use crate::service::task::TaskService;

use std::cell::RefCell;
use std::rc::Rc;

use gtk::{Window, WindowType, Widget};
use gtk::prelude::*;

pub struct AppContext {
    service_factory: ServiceFactory,
    app: Option<App>,
}

impl AppContext {
    fn new() -> AppContext {
        AppContext {
            service_factory: ServiceFactory::new(),
            app: None,
        }
    }

    fn set_app(&mut self, app: &App) {
        self.app = Some(app.clone());
    }

    pub fn get_service<S: Service>(&mut self) -> &mut S {
        self.service_factory.get_service(self.app.as_ref().unwrap())
    }
}

#[derive(Clone)]
pub struct QuitApp;

#[derive(Clone)]
pub struct App {
    context: Rc<RefCell<AppContext>>,
    window: Window,
}

fn create_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("My Little Editor");
    window.set_default_size(350, 70);
    window
}

impl App {

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn register_quit(&self) {
        self.with_context(|ctx| {
            let message_service = ctx.get_service::<MessageService>();
            message_service.register(move |app, _comp, _message: &QuitApp| {
                app.close_app();
            });
        })
    }

    pub fn new() -> App {

        let app = App {
            window: create_window(),
            context: Rc::new(RefCell::new(AppContext::new())),
        };
        {
            let mut ctx = app.context.borrow_mut();
            ctx.set_app(&app);
        }
        app.with_context(|ctx| {
            let _ts = ctx.get_service::<TaskService>();
        });

        app.register_quit();

        app.clone()
    }

    fn close_app(&self) {
        gtk::main_quit();
    }

    pub fn set_content(self, content: impl IsA<Widget>) -> Self {
        self.window.add(&content);
        self
    }

    pub fn run(&self) {
        self.window.show_all();

        let app_clone = self.clone();
        self.window.connect_delete_event(move |_, _| {
            app_clone.close_app();
            Inhibit(false)
        });

        gtk::main();
    }

    pub fn with_context<F: FnOnce(&mut AppContext)>(&self, f:F) {
        let mut context = self.context.borrow_mut();
        f(&mut context);
    }

}
