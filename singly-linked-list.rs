// source: "A Singly Linked List in Rust" by Ryan Levick on YouTube

// node struct containing some type T and a reference to next link
struct Node<T> {
	element: T,
	next: Link<T>,
}

// type specifier to make things prettier
type Link<T> = Option<Box<Node<T>>>;

// container for linked list containing a head and the length of how many links there are
pub struct LinkedList<T> {
	head: Link<T>,
	length: i32,
}

// implementation of linked list
impl<T> LinkedList<T> {
	// new func instantiating a linked list with no head and a length of 0
	fn new() -> LinkedList<T> {
		LinkedList { head: None, length: 0, }
	}

	// push func to push new link onto linked list
	fn push(&mut self, element: T) {
		// takes old head from self
		let old_head = self.head.take();
		// instantiates new head link object
		let new_head = Box::new(Node {
			element,
			next: old_head,
		});
		// sets self.head to the new head
		self.head = Some(new_head);
		// increment length
		self.length += 1;
	}

	// pop func to pop link off of linked list
	fn pop(&mut self) -> Option<T> {
		// decrement length
		self.length -= 1;
		// take old head from self and map to return element 
		self.head.take().map(|n| {
			// sets head to next element in list 
			self.head = n.next;
			// returns popped element
			n.element
		})
	}

	// peek func to preview whats at the head
	fn peek(&self) -> Option<&T> {
		// returns reference to element if exists
		self.head.as_ref().map(|n| &n.element)
	}
}
