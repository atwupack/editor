use crate::presenter::*;
use gtk::prelude::*;
use gtk::{TreeView, TreeIter, TreeViewColumn, CellRendererText, TreeStore, Type};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::read_dir;
use std::rc::Rc;
use std::cell::RefCell;

struct FileTreeItem {
    path: PathBuf,
    children_read: bool,
}

impl FileTreeItem {
    fn is_dir(&self) -> bool {
        self.path.is_dir()
    }

    fn load_children(&self) -> Vec<FileTreeItem> {
        let mut result = Vec::new();
        let children = read_dir(self.path.clone()).unwrap();
        for entry in children {
            let entry = entry.unwrap();
            let path= entry.path();
            let file_tree_item = FileTreeItem {
                path,
                children_read: false,
            };
            result.push(file_tree_item);
        }
        result
    }

}

pub struct FileTreePresenter {
    data: HashMap<u32, FileTreeItem>,
    next_index : u32,
    tree: TreeView,
}


fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 1);
    tree.append_column(&column);
}

impl FileTreePresenter {

    fn inc_index(&mut self) -> u32 {
        self.next_index+=1;
        self.next_index
    }

    fn add_node(&mut self, parent: Option<&TreeIter>, node: FileTreeItem) {
        let model = self.tree.get_model().unwrap();
        let model = model.downcast::<TreeStore>().unwrap();

        let index = self.inc_index();
        let tree_iter = model.insert_with_values(parent,None, &[0, 1], &[ &index, &node.path.to_str()]);

        if node.is_dir() {
            let dummy_index: u32 = 0;
            let tree_iter_2 = model.insert_with_values(Some(&tree_iter), None, &[0, 1], &[ &dummy_index, &"Loading..."]);
        }
        self.data.insert(index, node);
    }

    fn find_tree_item(&self, node: &TreeIter) -> &FileTreeItem {
        let model = self.tree.get_model().unwrap();
        let model = model.downcast::<TreeStore>().unwrap();

        let index = model.get_value(node, 0).get::<u32>().unwrap();
        self.data.get(&index).unwrap()
    }

    pub fn add_root_node(&mut self, root: PathBuf) {

        let tree_item = FileTreeItem {
            path: root,
            children_read: false,
        };

        self.add_node(None, tree_item);

    }

    fn register_row_expanded(&self) {
        let _handler_id = self.tree.connect_row_expanded(|tree, tree_iter, tree_path| {
            println!("{:?}", tree);
            println!("{:?}", tree_iter);
            println!("{}", tree_path);
        });
    }


}

fn register_test_expand_row(file_tree: &Rc<RefCell<FileTreePresenter>>) {

    let tree_clone = file_tree.clone();
    let file_tree_ref = file_tree.borrow();
    let _handler_id = file_tree_ref.get_view().connect_test_expand_row( move |tree, tree_iter, tree_path| {

        let mut tree_clone_ref = tree_clone.borrow_mut();
        let tree_item = tree_clone_ref.find_tree_item(tree_iter);

        let children = tree_item.load_children();
        for entry in children {
            tree_clone_ref.add_node(Some(tree_iter), entry)
        }

        Inhibit(false)
    });
}




impl Presenter<TreeView> for FileTreePresenter {

    fn new() -> PresRef<Self> {
        let tree_model = TreeStore::new( &[Type::U32,Type::String]);
        let tree = TreeView::new_with_model(&tree_model);
        append_text_column(&tree);
        tree.set_headers_visible(false);

        let file_tree = FileTreePresenter {
            data: HashMap::new(),
            tree,
            next_index: 0,
        };

        file_tree.register_row_expanded();

        let file_tree_ref = Rc::new(RefCell::new(file_tree));
        register_test_expand_row(&file_tree_ref);

        file_tree_ref
    }

    fn get_view(&self) -> &TreeView {
        &self.tree
    }
}
