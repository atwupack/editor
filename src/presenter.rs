use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Presenter<V: WidgetExt> {

    fn new() -> PresRef<Self>;

    fn get_view(&self) -> &V;

}

pub type PresRef<T> = Rc<RefCell<T>>;

