use crate::view::Presenter;
use crate::service::message::MessageService;

use gtk::prelude::*;
use gtk::{TreeView, ListStore, Type, TreeViewColumn, CellRendererText};

#[derive(Clone)]
pub struct PropertyPresenter {
    message_service: MessageService,
    table: TreeView,
    list_store: ListStore,
}

fn append_column(tree: &TreeView) {
    let column1 = TreeViewColumn::new();
    column1.set_title("Key");
    let text_cell1 = CellRendererText::new();

    column1.pack_start(&text_cell1, true);
    column1.add_attribute(&text_cell1, "text", 0);
    tree.append_column(&column1);

    let column2 = TreeViewColumn::new();
    column2.set_title("Value");
    let text_cell2 = CellRendererText::new();

    column2.pack_start(&text_cell2, true);
    column2.add_attribute(&text_cell2, "text", 1);
    tree.append_column(&column2);
}

impl PropertyPresenter {
    fn register_properties_changed(&self) {

        let pres_clone = self.clone();
        self.message_service.register("properties_changed", move |_,_,obj| {
            pres_clone.list_store.clear();
            let data = obj.downcast_ref::<Vec<(&str, String)>>().unwrap();
            for (fst, snd) in data.iter() {
                pres_clone.list_store.insert_with_values(None, &[0,1], &[&fst,&snd]);
            }
        });

    }
}

impl Presenter<TreeView> for PropertyPresenter {

    fn new(ms: &MessageService) -> Self {

        let list_store = ListStore::new(&[Type::String, Type::String]);
        let table = TreeView::new_with_model(&list_store);

        append_column(&table);
        table.set_headers_visible(true);
        let property_view = PropertyPresenter {
            message_service: ms.clone(),
            table,
            list_store,
        };

        property_view.register_properties_changed();
        property_view
    }

    fn get_view(&self) -> &TreeView {
        &self.table
    }
}

