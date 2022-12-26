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
    let mut firstIter = line.split(" ");
    let first = firstIter.next().unwrap();
    let second = firstIter.next().unwrap();
    
    match firstIter.next() {
      Some(third) => {
        // check for command
        if line.chars().nth(2).unwrap() == 'c' {
          println!("change directory: {} {} {}",first, second, third);

          if third == ".." {
            println!("implement up a dir");
          }
          else {
            currentNode.add_child(third.to_string(), 0);
            currentNode = currentNode.children.get_mut(third).unwrap();
          }
        }
      },
      None => //do nothinig
        ()
    }


    if line.chars().nth(0).unwrap().is_numeric() {
      currentNode.size += first.parse::<usize>().unwrap();
    }

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