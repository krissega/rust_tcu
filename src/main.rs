#[derive(Debug)]
struct Node {
    value: i32,
    priority: u32,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
struct PriorityQueue {
    head: Option<Box<Node>>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: i32, priority: u32) {
        let mut new_node = Box::new(Node {
            value,
            priority,
            next: None,
        });

        if self.head.is_none() || self.head.as_ref().unwrap().priority > priority {
            new_node.next = self.head.take();
            self.head = Some(new_node);
            return;
        }

        let mut current = self.head.as_mut().unwrap();

        loop {
            let insert_here = match current.next.as_ref() {
                Some(next_node) => next_node.priority > priority,
                None => true,
            };

            if insert_here {
                new_node.next = current.next.take();
                current.next = Some(new_node);
                break;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.value
        })
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

fn main() {
    let mut queue = PriorityQueue::new();

    queue.push(10, 5);
    queue.push(99, 1);
    queue.push(20, 3);

    while !queue.is_empty() {
        match queue.pop() {
            Some(val) => println!("Valor: {}", val), // Prints: 99, 20, 10
            None => break, // Exit the loop if `None` is encountered
        }
    }
}
