use crate::view::Presenter;
use crate::service::message::MessageService;

use gtk::prelude::*;
use gtk::{TreeView, ListStore, Type, TreeSelection};

#[derive(Clone)]
pub struct PropertyPresenter {
    message_service: MessageService,
    table: TreeView,
    list_store: ListStore,
}

impl Presenter<TreeView> for PropertyPresenter {

    fn new(ms: &MessageService) -> Self {

        let list_store = ListStore::new(&[Type::String, Type::String]);
        let table = TreeView::new_with_model(&list_store);

        let property_view = PropertyPresenter {
            message_service: ms.clone(),
            table,
            list_store,
        };

        ms.register("properties_changed", |_,_,obj| {

            println!("Got property changed event");
            let data = obj.downcast_ref::<Vec<(&str, &str)>>().unwrap();
            for (fst, snd) in data.iter() {
                println!("{}, {}", fst, snd);
            }
        });

        property_view
    }

    fn get_view(&self) -> &TreeView {
        &self.table
    }
}

