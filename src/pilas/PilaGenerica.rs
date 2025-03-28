struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

trait Pila<T> {
    fn new() -> Self;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
}

struct PilaCustom<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> Pila<T> for PilaCustom<T> {
    fn new() -> Self {
        PilaCustom {
            head: None,
            size: 0,
        }
    }

    fn push(&mut self, item: T) {
        let new_node = Node {
            value: item,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn size(&self) -> usize {
        self.size
    }
}
