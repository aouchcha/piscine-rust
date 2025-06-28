pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element)
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut temp : Vec<Rc<String>> = Vec::new();
        let mut check = false;
        for c in &self.ref_list {
            if !Rc::ptr_eq(c, &element) {
                temp.push(c.clone());
            }
        }
        self.ref_list = temp;
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}