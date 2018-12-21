use gtk::prelude::*;
use gtk::{TreeView, TreeIter, TreeViewColumn, CellRendererText, TreeStore, Type, TreeSelection};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::read_dir;
use std::rc::Rc;
use std::cell::RefCell;
use crate::service::message::MessageService;
use crate::view::Presenter;

/// Single file tree entry
#[derive(Clone)]
struct FileTreeItem {
    index: u32,
    path: PathBuf,
    children_read: bool,
}

impl FileTreeItem {
    fn load_children(&self) -> Vec<PathBuf> {
        let mut result = Vec::new();
        let children = read_dir(self.path.clone()).unwrap();
        for entry in children {
            let entry = entry.unwrap();
            let path= entry.path();
            result.push(path);
        }
        result
    }
}

struct FileTreeModel {
    data: HashMap<u32, FileTreeItem>,
    next_index : u32,
}

impl FileTreeModel {
    fn inc_index(&mut self) -> u32 {
        self.next_index+=1;
        self.next_index
    }

    fn add_item(&mut self, path: &PathBuf) -> u32 {
        let index = self.inc_index();
        let item = FileTreeItem {
            index,
            path: path.clone(),
            children_read: false,
        };
        let _old = self.data.insert(index, item);
        index
    }

    fn get_item(&self, index: u32) -> &FileTreeItem {
        self.data.get(&index).unwrap()
    }

    fn update_item(&mut self, item: FileTreeItem) {
        let _old = self.data.insert(item.index, item);
    }
}

fn append_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let text_cell = CellRendererText::new();

    column.pack_start(&text_cell, true);
    column.add_attribute(&text_cell, "text", 1);
    tree.append_column(&column);
}

#[derive(Clone)]
pub struct FileTreePresenter {
    model: Rc<RefCell<FileTreeModel>>,
    tree: TreeView,
    tree_store: TreeStore,
    message_service: MessageService,
}

impl FileTreePresenter {

    fn add_node(&self, parent: Option<&TreeIter>, path: &PathBuf) {
        let mut model = self.model.borrow_mut();
        let index = model.add_item(&path);

        let tree_iter = if parent==None {
            self.tree_store.insert_with_values(None, None, &[0, 1], &[&index, &path.to_str()])
        }
        else {
            self.tree_store.insert_with_values(parent, None, &[0, 1], &[&index, &path.file_name().unwrap().to_str().unwrap()])
        };

        if path.is_dir() {
            let dummy_index: u32 = 0;
            let _tree_iter_2 = self.tree_store.insert_with_values(Some(&tree_iter), None, &[0, 1], &[ &dummy_index, &"Loading..."]);
        }

    }

    fn find_tree_item(&self, node: &TreeIter) -> FileTreeItem {
        let index = self.tree_store.get_value(node, 0).get::<u32>().unwrap();

        let model = self.model.borrow();
        let item =  model.get_item(index);
        item.clone()
    }

    fn update_tree_item(&self, item: FileTreeItem) {
        let mut model = self.model.borrow_mut();
        model.update_item(item);
    }

    pub fn add_root_node(&self, root: &PathBuf) {
        self.add_node(None, root);
    }

    fn register_test_expand_row(&self) {
        let tree_clone = self.clone();
        let _handler_id = self.get_view().connect_test_expand_row( move |_tree, tree_iter, _tree_path| {

            let mut tree_item = tree_clone.find_tree_item(tree_iter);

            if !tree_item.children_read {

                let dummy_child = tree_clone.tree_store.iter_children(tree_iter).unwrap();
                let _result = tree_clone.tree_store.remove(&dummy_child);

                let children = tree_item.load_children();
                for entry in children {
                    tree_clone.add_node(Some(tree_iter), &entry)
                }
                tree_item.children_read = true;
                tree_clone.update_tree_item(tree_item);
            }
            Inhibit(false)
        });
    }

    fn register_select_row(&self) {
        let tree_clone = self.clone();
        let _handler_id = self.get_view().get_selection().connect_changed(move |selection| {
            println!("Selection changed");
            let data = vec![("1", "1"), ("2", "2")];
            tree_clone.message_service.send("file_tree", "properties_changed", &data);
            println!("Selection changed sent");
        });
    }

}

impl Presenter<TreeView> for FileTreePresenter {

    fn new(ms: &MessageService) -> Self {
        let tree_store = TreeStore::new( &[Type::U32,Type::String]);
        let tree = TreeView::new_with_model(&tree_store);
        append_column(&tree);
        tree.set_headers_visible(false);

        let model = FileTreeModel {
            data: HashMap::new(),
            next_index: 0,
        };

        let file_tree = FileTreePresenter {
            model: Rc::new(RefCell::new(model)),
            tree,
            tree_store,
            message_service: ms.clone(),
        };

        file_tree.register_test_expand_row();
        file_tree.register_select_row();

        let ft_clone= file_tree.clone();
        ms.register("tree.set-root", move |caller, id, obj|{
            let path = obj.downcast_ref::<PathBuf>().unwrap();
        });

        file_tree
    }

    fn get_view(&self) -> &TreeView {
        &self.tree
    }

}
