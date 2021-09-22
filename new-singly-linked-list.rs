struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { 
            val,
            next: None 
        }
    }

    fn insert(&mut self, val: i32) {
        match &mut self.next {
            Some(n) => {
                n.insert(val)
            },
            None => {
                self.next = Some(Box::new(Node {
                    val,
                    next: None,
                }));
            },
        }
    }

    fn print(&self) {
        println!("{}", self.val);
        match &self.next {
            Some(n) => {
                n.print();
            },
            None => {
                println!("Reached end of linked list");
            },
        };
    }
}
