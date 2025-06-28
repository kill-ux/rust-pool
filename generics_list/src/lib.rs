#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
    pub counter: usize,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List {
            head: None,
            counter: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        match self.head.take() {
            Some(head) => {
                self.head = Some(Node {
                    value: value,
                    next: Some(Box::new(head)),
                });
            }
            None => {
                self.head = Some(Node {
                    value: value,
                    next: None,
                });
            }
        }

        self.counter += 1;
    }

    pub fn pop(&mut self) {
        match self.head.take() {
            Some(node) => match node.next {
                Some(next) => {
                    self.head = Some(*next);
                    self.counter -= 1;
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn len(&self) -> usize {
        self.counter
    }
}
