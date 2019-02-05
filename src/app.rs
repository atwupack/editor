use crate::service::Service;
use crate::service::ServiceFactory;

use std::cell::{RefCell,RefMut};
use std::rc::Rc;

use gtk::{Window, WindowType};
use gtk::prelude::*;

#[derive(Clone)]
pub struct App {
    service_factory: Rc<RefCell<ServiceFactory>>,
}

impl App {
    pub fn new() -> App {
        App {
            service_factory: Rc::new(RefCell::new(ServiceFactory::new())),
        }
    }

    fn create_window(&self) {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("My Little Editor");
        window.set_default_size(350, 70);
    }

    pub fn get_service<T: Service>(&self) -> RefMut<T> {
        let sf = self.service_factory.borrow_mut();
        RefMut::map(sf, |sf| {
            sf.get_service(self)
        })
    }


}
