use std::cell::{Cell, RefCell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }

    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let thread: Thread<'_> = Thread::new_thread(self.track_worker(), c, self);
        self.states.borrow_mut().push(false);
        (thread.pid, thread)
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        match self.states.borrow().get(id) {
            Some(b) => *b,
            None => false,
        }
    }
    pub fn add_drop(&self, id: usize) {
        let dropped = match self.states.borrow().get(id) {
            Some(b) => (true, *b),
            None => (false, false),
        };
        match dropped {
            (_, true) => panic!("{}", &format!("{id} is already dropped")),
            (true, false) => {
                self.states.borrow_mut()[id] = true;
                self.drops.set(self.drops.get() + 1);
            }
            (false, false) => {
                self.states.borrow_mut().push(true);
                self.drops.set(self.drops.get() + 1);
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Self {
            pid: p,
            cmd: c,
            parent: t,
        }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid);
    }
}
