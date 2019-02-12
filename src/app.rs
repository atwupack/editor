use crate::service::{Service, ServiceFactory};
use crate::service::message::MessageService;

use std::cell::{RefCell,RefMut};
use std::rc::Rc;

use gtk::{Window, WindowType, Widget};
use gtk::prelude::*;

//use std::borrow::BorrowMut;

pub struct QuitApp;

#[derive(Clone)]
pub struct App {
    service_factory: Rc<RefCell<ServiceFactory>>,
    window: Window,
}

fn create_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("My Little Editor");
    window.set_default_size(350, 70);
    window
}

impl App {

    fn register_quit(&self) {
        let mut message_service = self.get_service::<MessageService>();
        let app_clone = self.clone();
        message_service.register(move |_comp, _message: &QuitApp| {
            app_clone.close_app();
        });
    }

    pub fn new() -> App {
        let app = App {
            service_factory: Rc::new(RefCell::new(ServiceFactory::new())),
            window: create_window(),
        };

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


    pub fn get_service<T: Service>(&self) -> RefMut<T> {
        let sf: RefMut<ServiceFactory> = self.service_factory.borrow_mut();
        RefMut::map(sf, |sf| {
            sf.get_service(self)
        })
    }

    pub fn with_services<F: FnOnce(RefMut<ServiceFactory>)>(&self, f:F) {
        let sf: RefMut<ServiceFactory> = self.service_factory.borrow_mut();
        f(sf)
    }

}
