pub mod file_tree;
pub mod property;

use gtk::prelude::*;
use crate::service::message::MessageService;

pub trait Presenter<V: WidgetExt> {

    fn new(ms: &MessageService) -> Self;

    fn get_view(&self) -> &V;

}
