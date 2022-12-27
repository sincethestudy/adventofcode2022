use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
  value: RefCell<i32>,
  children: RefCell<Vec<Rc<Node>>>,
  parent: RefCell<Weak<Node>>
}

impl Node {
  fn add_parent(&self, node: Weak<Node>){
    *self.parent.borrow_mut() = node;
  }

  fn add_child(&self, node: Rc<Node>){
    self.children.borrow_mut().push(node)
  }

  fn add_value(&self, add: i32){
    *self.value.borrow_mut() += add;
  }
}


fn main() {

  let leaf1 = Rc::new(Node {
    value: RefCell::new(45),
    children: RefCell::new(Vec::new()),
    parent: RefCell::new(Weak::new())
  });

  let leaf2 = Rc::new(Node {
    value: RefCell::new(55),
    children: RefCell::new(Vec::new()),
    parent: RefCell::new(Weak::new())
  });

  let root = Rc::new(Node {
    value: RefCell::new(65),
    children: RefCell::new(Vec::new()),
    parent: RefCell::new(Weak::new())
  });

  leaf1.add_parent(Rc::downgrade(&root));

  root.add_child(Rc::clone(&leaf2));


}