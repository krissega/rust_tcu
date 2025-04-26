#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match self.left {
                Some(ref mut left_child) => left_child.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right_child) => right_child.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
        // Si es igual, no lo insertamos (para evitar duplicados)
    }

    fn print_inorder(&self) {
        if let Some(ref left) = self.left {
            left.print_inorder();
        }

        println!("{}", self.value);

        if let Some(ref right) = self.right {
            right.print_inorder();
        }
    }
}

fn main() {
    let mut root = Node::new(15);
    root.insert(22);
    root.insert(45);
    root.insert(18);
    root.insert(80);
    root.insert(5);
    root.insert(20);
    root.insert(8);
    root.insert(2);
    println!("Árbol en orden:");
    root.print_inorder(); // Debe imprimir: 2, 5, 8, 15, 18, 20, 22, 45, 80
}
