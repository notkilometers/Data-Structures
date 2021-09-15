struct Tree {
    val: i32,
    left: Option<Box<Tree>>,
    right: Option<Box<Tree>>,
}

impl Tree {
    fn new(val: i32) -> Self {
        Tree { 
            val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: i32) {
        let next_node = if val >= self.val {
            &mut self.left
        } else {
            &mut self.right
        };

        match next_node {
            Some(node) => {
                node.insert(val)
            },
            None => {
                *next_node = {
                    Some(Box::new(
                        Tree {
                            val,
                            left: None,
                            right: None,
                        }
                    ))
                }
            }
        }
    }

    fn print(self) {
        print!("{} ", self.val);

        match self.left {
            Some(n) => {n.print()},
            None => {},
        };


        match self.right {
            Some(n) => {n.print()},
            None => {},
        };
    }

}

fn main() {
    let mut tree = Tree::new(10);
    tree.insert(15);
    tree.insert(5);

    tree.print();

}
