use std::rc::Rc;
use std::cell::RefCell;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: RefCell<usize>,
    pub max: usize,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Self {
            logger,
            value: RefCell::new(0),
            max
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let num_of_refs = Rc::strong_count(value);
        *self.value.borrow_mut() = num_of_refs;
        let pr = (num_of_refs * 100) / self.max;
        if pr >= 100 {
            self.logger.error("Error: you are over your quota!");
        }else if pr >= 70 {
            let msg = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution",pr);
            self.logger.warning(&msg);
        }
    }

    pub fn peek<T>(&self, value: &Rc<T>) {
        let num_of_refs = Rc::strong_count(value);
        let pr = (num_of_refs * 100) / self.max;
        let msg = format!("Info: you are using up to {}% of your quota",pr);
        self.logger.info(&msg);
    }
}