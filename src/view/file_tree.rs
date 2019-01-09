use crate::app::App;
use crate::service::message::MessageService;
use crate::service::file::{FileItem, FileService};
use crate::view::Presenter;
use gtk::prelude::*;
use gtk::{CellRendererText, TreeIter, TreeStore, TreeView, TreeViewColumn, Type};
use std::path::{Path};

fn append_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let text_cell = CellRendererText::new();

    column.pack_start(&text_cell, true);
    column.add_attribute(&text_cell, "text", 1);
    tree.append_column(&column);
}

#[derive(Clone)]
pub struct FileTreePresenter {
    tree: TreeView,
    tree_store: TreeStore,
    message_service: MessageService,
    file_service: FileService,
}

impl FileTreePresenter {
    fn add_node(&self, parent: Option<&TreeIter>, item: &FileItem) {

        let tree_iter = if parent == None {
            self.tree_store
                .insert_with_values(None, None, &[0, 1], &[&item.path_str(), &item.path_str()])
        } else {
            self.tree_store.insert_with_values(
                parent,
                None,
                &[0, 1],
                &[&item.path_str(), &item.name()],
            )
        };

        if item.is_dir() {
            let dummy_index: u32 = 0;
            let _tree_iter_2 = self.tree_store.insert_with_values(
                Some(&tree_iter),
                None,
                &[0, 1],
                &[&dummy_index, &"Loading..."],
            );
        }
    }

    fn remove_all_children(&self, parent: &TreeIter) {
        let child_iter = self.tree_store.iter_children(parent).unwrap();
        while self.tree_store.remove(&child_iter) {}
    }

    fn find_tree_item(&self, node: &TreeIter) -> FileItem {
        let path: String = self.tree_store.get_value(node, 0).get().unwrap();
        self.file_service.get_item(path)
    }

    pub fn add_root_node<P: AsRef<Path>>(&self, root: P) {
        let item = self.file_service.get_item(&root);
        self.add_node(None, &item);
    }

    fn register_test_expand_row(&self) {
        let tree_clone = self.clone();
        let _handler_id =
            self.get_view()
                .connect_test_expand_row(move |_tree, tree_iter, _tree_path| {
                    tree_clone.remove_all_children(tree_iter);
                    let tree_item = tree_clone.find_tree_item(tree_iter);
                    let children = tree_clone.file_service.get_children(&tree_item);
                    for child in children {
                        tree_clone.add_node(Some(tree_iter), &child);
                    }
                    Inhibit(false)
                });
    }

    fn register_select_row(&self) {
        let tree_clone = self.clone();
        let _handler_id = self
            .get_view()
            .get_selection()
            .connect_changed(move |selection| {
                let mut data = Vec::new();
                let (_model, iter) = selection.get_selected().unwrap();
                let item = tree_clone.find_tree_item(&iter);
                data.push(("Path", String::from(item.path_str())));
                data.push((
                    "Name",
                    String::from(item.name()),
                ));
                tree_clone
                    .message_service
                    .send("file_tree", "properties_changed", &data);
                tree_clone.message_service.send("file_tree", "append_log", &String::from("Row selected"));
            });
    }
}

impl Presenter<TreeView> for FileTreePresenter {
    fn new(app: &App) -> Self {
        let tree_store = TreeStore::new(&[Type::String, Type::String]);
        let tree = TreeView::new_with_model(&tree_store);
        append_column(&tree);
        tree.set_headers_visible(false);

        let file_tree = FileTreePresenter {
            tree,
            tree_store,
            message_service: app.get_service(),
            file_service: app.get_service(),
        };

        file_tree.register_test_expand_row();
        file_tree.register_select_row();

        file_tree
    }

    fn get_view(&self) -> &TreeView {
        &self.tree
    }
}
