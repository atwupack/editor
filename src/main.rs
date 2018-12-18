
mod presenter;
mod file_tree;

use gtk::prelude::*;
use std::path::PathBuf;

use gtk::{Window, WindowType, Paned, Orientation, Frame};
use crate::file_tree::FileTreePresenter;
use crate::presenter::Presenter;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("First GTK+ Program");
    window.set_default_size(350, 70);
    let vertical_split = Paned::new(Orientation::Horizontal);
    vertical_split.set_wide_handle(true);

    let root = PathBuf::from(r"C:\Tools");
    let file_tree = FileTreePresenter::new();
    {
        file_tree.borrow_mut().add_root_node(root);
        let tree_ref = file_tree.borrow();
        let tree = tree_ref.get_view();

        vertical_split.pack1(  tree, true, false);

    }


    //let frame1 = Frame::new("Frame 1");

    let frame2 = Frame::new("Frame 2");
    vertical_split.pack2(&frame2, false, false);

    window.add(&vertical_split);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
