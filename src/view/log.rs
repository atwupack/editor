use crate::view::Presenter;
use crate::app::App;
use crate::service::message::MessageService;

use gtk::{TextView, TextBuffer};
use gtk::prelude::*;

#[derive(Clone)]
pub struct LogPresenter {
    text_view: TextView,
    text_buffer: TextBuffer,
    message_service: MessageService,
}

impl LogPresenter {
    fn register_append_log(&self) {
        let log_clone = self.clone();
        self.message_service.register("append_log", move |_,_,text| {
            let text_str : &String = text.downcast_ref().unwrap();
            let mut text_iter = log_clone.text_buffer.get_end_iter();
            log_clone.text_buffer.insert(&mut text_iter, text_str.as_str());
            log_clone.text_buffer.insert(&mut text_iter, "\n");
        });
    }
}

impl Presenter<TextView> for LogPresenter {
    fn new(app: &App) -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::new_with_buffer(&buffer);
        view.set_editable(false);
        let log =LogPresenter {
            text_view: view,
            text_buffer: buffer,
            message_service: app.get_service(),
        };

        log.register_append_log();

        log
    }

    fn get_view(&self) -> &TextView {
        &self.text_view
    }
}