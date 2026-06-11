use spinners::{Spinner, Spinners};

pub(crate) struct SpinnerHandle(Spinner);

impl SpinnerHandle {
    pub fn new(message: String) -> Self {
        let spinner = Spinner::new(Spinners::Dots9, message);
        Self(spinner)
    }

    pub fn stop(mut self, msg: String) {
        self.0.stop_with_message(msg);
    }
}
