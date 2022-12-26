use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet, HashMap};


fn main() {
  const input: &str = include_str!("input.txt");
  let mut lines = input.lines();

  let mut root = Node {
    size: 0,
    children: HashMap::new()
  };

  let mut currentNode = &mut root;


  //add files in the current directory
  for line in lines {
    let first = line.split(" ").next().unwrap();

    if line.chars().nth(0).unwrap().is_numeric() {
      let value = line.split(" ").next().unwrap().parse::<usize>().unwrap();
      let name = line.split(" ").nth(1).unwrap();
      println!("its a node with size: {}", first);

      currentNode.add_child(String::from(name), value)
    }

    // check for change directory
    






  }





  println!("{:?}", root);

}

#[derive(Debug)]
struct Node {
  size: usize,
  children: HashMap<String, Node>
}

impl Node {
  fn new(size: usize) -> Node {
    Node {
      size,
      children: HashMap::new()
    }
  }

  fn add_child(&mut self, name: String, size: usize) {
    self.children.insert(name, Node::new(size));
  }
}