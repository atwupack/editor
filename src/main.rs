mod app;
mod service;
mod view;

use crate::app::App;
use crate::service::task::Task;
use crate::view::file_tree::FileTreePresenter;
use crate::view::property::PropertyPresenter;
use crate::view::log::LogPresenter;
use crate::view::Presenter;
use gdk::Screen;
use gtk::prelude::*;
use gtk::{
    CssProvider, Orientation, Paned, ScrolledWindow, StyleContext,
    Window, WindowType, Widget,
};
use std::path::PathBuf;

struct OpenDirectory;

impl Task for OpenDirectory {
    fn run(&self, app: &App) {
        unimplemented!()
    }
}

fn create_ui(app: &App) -> impl IsA<Widget> {
    let vertical_split = Paned::new(Orientation::Horizontal);
    vertical_split.set_wide_handle(false);

    // create tree
    let root = PathBuf::from(r".").canonicalize().unwrap();
    let file_tree = FileTreePresenter::new(&app);
    file_tree.add_root_node(&root);
    let tree = file_tree.get_view();

    let scroll = ScrolledWindow::new(None, None);
    scroll.add(tree);

    vertical_split.pack1(&scroll, true, false);

    // create properties view
    let props = PropertyPresenter::new(&app);

    let scroll = ScrolledWindow::new(None, None);
    scroll.add(props.get_view());

    vertical_split.pack2(&scroll, true, false);

    // add log view
    let log = LogPresenter::new(&app);
    let log_scroll = ScrolledWindow::new(None, None);
    log_scroll.add(log.get_view());
    let horiz_split = Paned::new(Orientation::Vertical);
    horiz_split.pack1(&vertical_split, true, false);
    horiz_split.pack2(&log_scroll, true, false);
    horiz_split
}

fn create_window() {

}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let screen = Screen::get_default().unwrap();

    let css = CssProvider::new();
    css.load_from_path("resource/css/editor.css").unwrap();

    StyleContext::add_provider_for_screen(&screen, &css, 200);

    let app = App::new();

    // create window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("My Little Editor");
    window.set_default_size(350, 70);

    let content = create_ui(&app);

    window.add(&content);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
