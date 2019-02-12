mod app;
mod service;
mod view;

use crate::app::App;
use crate::view::file_tree::FileTreePresenter;
use crate::view::property::PropertyPresenter;
use crate::view::log::LogPresenter;
use crate::view::menu::MainMenuPresenter;
use crate::view::Presenter;
use gdk::Screen;
use gtk::prelude::*;
use gtk::{
    CssProvider, Orientation, Paned, ScrolledWindow, StyleContext,
    Widget, Box,
};
use std::path::PathBuf;

fn create_ui(app: &App) -> impl IsA<Widget> {

    let vbox = Box::new(Orientation::Vertical, 0);

    let main_menu = MainMenuPresenter::new(app);

    vbox.pack_start(main_menu.get_view(), false, false, 0);

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

    vbox.pack_start(&horiz_split, true, true, 0);

    vbox
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

    let content = create_ui(&app);

    app.set_content(content).run();
}
