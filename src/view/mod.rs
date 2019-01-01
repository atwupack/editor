pub mod file_tree;
pub mod property;
pub mod log;

use gtk::prelude::*;

use crate::app::App;

pub trait Presenter<V: WidgetExt> {
    fn new(app: &App) -> Self;

    fn get_view(&self) -> &V;
}


