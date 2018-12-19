use gtk::prelude::*;

pub trait Presenter<V: WidgetExt> {

    fn new() -> Self;

    fn get_view(&self) -> &V;

}

