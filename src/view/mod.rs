pub mod file_tree;
pub mod property;

use gtk::prelude::*;
use gtk::ScrolledWindow;

use crate::app::App;

pub trait Presenter<V: WidgetExt> {
    fn new(app: &App) -> Self;

    fn get_view(&self) -> &V;
}

pub fn create_view() {
    let scroll = ScrolledWindow::new(None, None);
    //scroll.add(content.clone());
}
