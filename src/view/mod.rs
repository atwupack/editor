pub mod file_tree;
pub mod property;
pub mod log;
pub mod menu;

use gtk::prelude::*;
use gtk::Widget;

use crate::app::App;

pub trait Presenter<V: IsA<Widget>> {
    fn new(app: &App) -> Self;

    fn get_view(&self) -> &V;
}


