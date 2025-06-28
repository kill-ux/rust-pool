pub use std::{cell::RefCell, rc::Rc};

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a ,T: Logger> {
    pub logger: &'a T,
    pub value: usize,
    pub max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Self {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let v = Rc::strong_count(value);
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

    pub fn peek(&self, value: &Rc<usize>) {
        let v = Rc::strong_count(value);
        let perc = (v * 100) / self.max;
        self.logger
            .info(&format!("Info: you are using up to {perc}% of your quota"));
    }
}
