struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

trait PilaAritmetrica {
    fn new() -> Self;
    fn push(&mut self, item: String);
    fn pop(&mut self) -> Option<String>;
    fn peek(&self) -> Option<&String>;
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn solve(&mut self);
}

struct PilaAritmetricaCustom {
    head: Option<Box<Node<String>>>,
    size: usize,
}

impl PilaAritmetrica for PilaAritmetricaCustom {
    fn new() -> Self {
        PilaAritmetricaCustom {
            head: None,
            size: 0,
        }
    }
    fn push(&mut self, item: String) {
        let new_node = Node {
            value: item,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<String> {
        if self.head.is_none() {
            return None;
        }
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }
    fn peek(&self) -> Option<&String> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn solve(&mut self) {
        if !self.is_empty() {
            let mut operation_value_vector = Vec::new();
            while let Some(value) = self.pop() {
                match value.as_str() {
                    "+" => {
                        let sum = operation_value_vector.iter().sum();
                        operation_value_vector.clear();
                        operation_value_vector.push(sum);
                    }
                    "-" => {
                        let mut iter = operation_value_vector.iter();
                        let first = *iter.next().unwrap();
                        let difference = iter.fold(first, |acc, &x| acc - x);
                        operation_value_vector.clear();
                        operation_value_vector.push(difference);
                    }
                    "*" => {
                        let product = operation_value_vector.iter().product();
                        operation_value_vector.clear();
                        operation_value_vector.push(product);
                    }
                    "/" => {
                        let mut iter = operation_value_vector.iter();
                        let first = *iter.next().unwrap();
                        let quotient = iter.fold(first, |acc, &x| acc / x);
                        operation_value_vector.clear();
                        operation_value_vector.push(quotient);
                    }
                    _ => {
                        operation_value_vector.push(value.parse::<f64>().unwrap());
                    }
                }
            }
            self.push(operation_value_vector[0].to_string());
        }
    }
}
