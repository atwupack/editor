use crate::view::Presenter;
use crate::app::App;
use crate::service::message::MessageService;

use gtk::{TextView, TextBuffer};
use gtk::prelude::*;

#[derive(Clone)]
pub struct LogPresenter {
    text_view: TextView,
    text_buffer: TextBuffer,
    app: App,
}

#[derive(Debug)]
pub struct AppendLog(pub String);

impl LogPresenter {
    fn register_append_log(&self) {
        let log_clone = self.clone();
        self.app.get_service::<MessageService>().register(move |_, text: &AppendLog| {
            log_clone.append_log_callback(text);
        });
    }

    fn append_log_callback(&self, message: &AppendLog) {
        let AppendLog(text_str) = message;
        let mut text_iter = self.text_buffer.get_end_iter();
        self.text_buffer.insert(&mut text_iter, text_str.as_str());
        self.text_buffer.insert(&mut text_iter, "\n");
    }
}

impl Presenter<TextView> for LogPresenter {
    fn new(app: &App) -> Self {
        let buffer = TextBuffer::new(None);
        let view = TextView::new_with_buffer(&buffer);
        view.set_editable(false);
        let log = LogPresenter {
            text_view: view,
            text_buffer: buffer,
            app: app.clone(),
        };

        log.register_append_log();

        log
    }

    fn get_view(&self) -> &TextView {
        &self.text_view
    }
}