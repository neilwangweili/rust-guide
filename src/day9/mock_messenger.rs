use std::cell::RefCell;

pub struct MockMessenger {
    pub sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    pub fn new() -> MockMessenger {
        Self {
            sent_messages: RefCell::new(vec![]),
        }
    }

    pub fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}
