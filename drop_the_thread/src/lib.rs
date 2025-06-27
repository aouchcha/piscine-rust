use std::cell::{RefCell, Cell};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>
}

impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new())
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.track_worker();
        let thread = Thread::new_thread(pid, c, self);
        self.states.borrow_mut().push(false);
        (pid, thread)
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow().get(id).copied().unwrap_or(false)
    }
    pub fn add_drop(&self, id: usize) {
        let mut states = self.states.borrow_mut();
        if let Some(value) = states.get_mut(id) {
            if *value {
                panic!("{} is already dropped", id);
            }else {
                *value = true;
                let current_drops = self.drops.get();
                self.drops.set(current_drops +1); 
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
    pub pid : usize,
    pub cmd: String,
    pub parent: &'a Workers
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Self {
            pid: p,
            cmd: c,
            parent: t
        }
    }
    pub fn skill(self) {
        drop(self)
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.parent.add_drop(self.pid)
    }
}