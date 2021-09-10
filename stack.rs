// stack containing a vector containing type t
struct Stack<T> {
    body: Vec<T>,
}

// implementation of stack
impl<T> Stack<T> {
    // new func instantiating a stack with a vector 
    fn new() -> Self {
        Stack { body: Vec::<T>::new() }
    }

    // pushes element onto vector
    fn push(&mut self, element: T) {
        self.body.push(element);
    }

    // pops from stack
    fn pop(&mut self) -> Option<T> {
        self.body.pop()
    }

    // peeks onto end of stack
    fn peek(&self) -> Option<&T> {
        self.body.last()
    }
}
