pub mod messenger;

pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use messenger::*;

pub struct Worker {
    pub track_value: Rc<i32>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: i32) -> Self {
        Self {
            track_value: Rc::new(track_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new())
        }
    }

    pub fn strip_prefix(msg: &str) -> &str {
        if let Some(stripped) = msg.strip_prefix("Error: ") {
            stripped
        } else if let Some(stripped) = msg.strip_prefix("Info: ") {
            stripped
        } else if let Some(stripped) = msg.strip_prefix("Warning: ") {
            stripped
        } else {
            msg
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let stripped = Self::strip_prefix(msg);
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), stripped.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        let stripped = Self::strip_prefix(msg);
        self.mapped_messages.borrow_mut().insert("Info".to_string(), stripped.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        let stripped = Self::strip_prefix(msg);
        self.mapped_messages.borrow_mut().insert("Error".to_string(), stripped.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}