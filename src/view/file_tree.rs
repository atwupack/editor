use crate::app::App;
use crate::service::message::{MessageService};
use crate::service::file::{FileItem, FileService};
use crate::view::property::PropertiesChanged;
use crate::view::log::AppendLog;
use crate::view::Presenter;
use gtk::prelude::*;
use gtk::{CellRendererText, TreeIter, TreeStore, TreeView, TreeViewColumn, Type};
use std::path::{Path, PathBuf};

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
    app: App,
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
        self.app.get_service::<FileService>().get_item(path)
    }

    pub fn add_root_node<P: AsRef<Path>>(&self, root: P) {
        let item = self.app.get_service::<FileService>().get_item(&root);
        self.add_node(None, &item);
    }

    fn register_test_expand_row(&self) {
        let tree_clone = self.clone();
        let _handler_id =
            self.get_view()
                .connect_test_expand_row(move |_tree, tree_iter, _tree_path| {
                    tree_clone.remove_all_children(tree_iter);
                    let tree_item = tree_clone.find_tree_item(tree_iter);
                    let mut file_service = tree_clone.app.get_service::<FileService>();
                    let children = file_service.get_children(&tree_item);
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
                let (_model, iter) = selection.get_selected().unwrap();
                let item = tree_clone.find_tree_item(&iter);
                let message_service = tree_clone.app.get_service::<MessageService>();
                message_service.send("file-tree", &FileSelected(item));
            });
    }
}


#[derive(Debug, Clone)]
pub struct FileSelected(pub FileItem);
#[derive(Debug, Clone)]
pub struct AddRootNode(pub PathBuf);

impl Presenter<TreeView> for FileTreePresenter {
    fn new(app: &App) -> Self {
        let tree_store = TreeStore::new(&[Type::String, Type::String]);
        let tree = TreeView::new_with_model(&tree_store);
        append_column(&tree);
        tree.set_headers_visible(false);

        let file_tree = FileTreePresenter {
            tree,
            tree_store,
            app: app.clone(),
        };

        file_tree.register_test_expand_row();
        file_tree.register_select_row();

        let mut message_service = app.get_service::<MessageService>();
        message_service.connect(|input: &FileSelected| {
            let FileSelected(item) = input;
            let mut data = Vec::new();
            data.push((String::from("Path"), String::from(item.path_str())));
            data.push((
                String::from("Name"),
                String::from(item.name()),
            ));
            PropertiesChanged(data)
        });

        message_service.connect(|_input: &FileSelected| {
            AppendLog(String::from("Row selected."))
        });

        let tree_clone = file_tree.clone();
        message_service.register(move |app, id, message: &AddRootNode| {
            let AddRootNode(path) = message;
            tree_clone.add_root_node(path);
        });

        file_tree
    }

    fn get_view(&self) -> &TreeView {
        &self.tree
    }
}
