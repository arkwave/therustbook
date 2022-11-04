use std::cell::RefCell;
use std::rc::{Rc, Weak};


// notes on the type of "children":
// 1. We want a Node to own its children -> Vec containing Rc<Node>
// 2. We want to be able to mutate the children of a given node -> RefCell<Node>
#[derive(Debug)]
struct Node {
    value: i32, 
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}


fn main() {
    let leaf = Rc::new(Node {
        value: 3, 
        children:RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())     
    });

    let branch = Rc::new(Node {
        value: 10, 
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())

    });

    // assign the parent to the child via a weak reference. 
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("leaf value: {}", leaf.value);
    println!("branch child: {:?}", branch.children.borrow());
    println!("branch value: {:?}", branch.value);
}
