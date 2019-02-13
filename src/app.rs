use crate::service::{Service, ServiceFactory};
use crate::service::message::MessageService;
use crate::service::task::TaskService;

use std::cell::RefMut;

use gtk::{Window, WindowType, Widget};
use gtk::prelude::*;

#[derive(Clone)]
pub struct QuitApp;

#[derive(Clone)]
pub struct App {
    service_factory: ServiceFactory,
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
        let mut message_service = self.get_service::<MessageService>();
        message_service.register(move |app, _comp, _message: &QuitApp| {
            app.close_app();
        });
    }

    pub fn new() -> App {
        let app = App {
            service_factory: ServiceFactory::new(),
            window: create_window(),
        };

        app.register_quit();

        let _ts = app.get_service::<TaskService>();

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


    pub fn get_service<S: Service>(&self) -> RefMut<S> {
        self.service_factory.get_service(self)
    }

    pub fn with_services<F: FnOnce(&ServiceFactory)>(&self, f:F) {
        f(&self.service_factory)
    }

}
