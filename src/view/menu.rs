use crate::view::Presenter;
use crate::app::{App, QuitApp};
use crate::service::message::MessageService;
use crate::service::task::{Task, RunTask};

use gtk::{MenuBar, MenuItem, Menu, SeparatorMenuItem};
use gtk::prelude::*;

use std::any::Any;

#[derive(Clone)]
pub struct MainMenuPresenter {
    menu_bar: MenuBar,
    app: App,
}

pub struct SelectProjectDirectory;

impl Task for SelectProjectDirectory {
    fn run(&self, app: &App) {
        println!("Select project directory");
    }
}

impl MainMenuPresenter {
    fn create_message_item<M: Any>(&self, label: &'static str, message: M) -> MenuItem {
        let mmp_clone = self.clone();

        let mi = MenuItem::new_with_label(label);
        mi.connect_activate(move |_menu_item| {
            let message_service = mmp_clone.app.get_service::<MessageService>();
            message_service.send(label, &message);
        });

        mi
    }
}

impl Presenter<MenuBar> for MainMenuPresenter {
    fn new(app: &App) -> Self {

        let menu_bar = MenuBar::new();

        let mmp = MainMenuPresenter {
            menu_bar,
            app: app.clone(),
        };

        let file_menu = Menu::new();

        let file_mi = MenuItem::new_with_label("File");
        mmp.menu_bar.append(&file_mi);
        file_mi.set_submenu(Some(&file_menu));

        let add_dir_mi = mmp.create_message_item("Add Project Directory...", RunTask(&SelectProjectDirectory));
        let quit_mi = mmp.create_message_item("Quit", QuitApp);

        file_menu.append(&add_dir_mi);
        file_menu.append(&SeparatorMenuItem::new());
        file_menu.append(&quit_mi);

        mmp
    }

    fn get_view(&self) -> &MenuBar {
        &self.menu_bar
    }
}