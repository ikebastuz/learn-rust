pub struct Notification {
    pub message: Option<String>,
}

impl Notification {
    pub fn new() -> Notification {
        Notification { message: None }
    }
    pub fn notify(&mut self, new_message: Option<String>) {
        self.message = new_message;
    }
}

impl AsRef<Option<String>> for Notification {
    fn as_ref(&self) -> &Option<String> {
        &self.message
    }
}
