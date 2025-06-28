#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None
        }
    }

    pub fn push(&mut self, value: T) {
        match self.head.take() {
            Some(data) => {
                self.head = Some(
                    Node {
                        value,
                        next: Some(Box::new(data)),
                    }
                );
                return
            },
            _ => {
                self.head = Some (
                    Node {
                        value,
                        next : None,
                    }
                );
                return
            },
        };
        
    }

    pub fn pop(&mut self) {
        match self.head.take() {
            Some(mut data) => {
                match data.next.take() {
                    Some(b) => {
                        self.head = Some(*b);
                        return
                    },
                    _ => {
                        self.head = None;
                        return
                    }
                };
            },
            _ => {
                self.head = None;
                return
            },
        };
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut node =  match &self.head{
            Some(n)=> Some(n),
               _ => None
        };
               while let Some(n) = node {
                count += 1;
                node = match &n.next{
                      Some(nn)=> Some(&nn),
                        _=> None
                };
            }

        count
    }
}