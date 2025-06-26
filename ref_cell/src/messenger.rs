pub use std::{cell::RefCell, rc::Rc};

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: u32,
    pub max: u32,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a dyn Logger, max: u32) -> Self {
        Self {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let v = Rc::strong_count(value) as u32;
        let perc = (v * 100) / self.max;
        if perc >= 100 {
            self.logger.error("Error: you are over your quota!");
            return;
        } else if perc >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {perc}% of your quota! Proceeds with precaution"
            ));
        }
    }

    pub fn peek<T>(&self, value: &Rc<T>) {
        let v = Rc::strong_count(value) as u32;
        let perc = (v * 100) / self.max;
        self.logger
            .info(&format!("Info: you are using up to {perc}% of your quota"));
    }
}
