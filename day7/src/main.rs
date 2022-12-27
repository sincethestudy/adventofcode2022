use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Default)]
struct Node {
  value: RefCell<i32>,
  name: String,
  children: RefCell<Vec<Rc<Node>>>,
  parent: RefCell<Weak<Node>>,
  
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

  fn total<'a>(&self, totalSum: &'a mut i32) -> &'a mut i32 {    

    *totalSum += *self.value.borrow();

    for child in self.children.borrow().iter() {
      child.total(totalSum);
    }

    totalSum
  }


  fn traverse<'a, 'b>(&self, totalSum: &'a mut i32, pos: &'b mut Vec<i32>, need: i32) -> (i32, &'b mut Vec<i32>, i32) {
    let mut localsum = *self.value.borrow();

    for child in self.children.borrow().iter() {
      let temp = child.traverse(totalSum, pos, need);
      localsum += temp.0;
    }

    if localsum >= need {
      // println!("found: {}", self.name);
      pos.push(localsum);
    }

    
    (localsum, pos, need)
  }
}


fn main() {

  let input = include_str!("input.txt");
  let lines = input.lines();

  let root = Rc::new(Node {
    value: RefCell::new(0),
    name: String::from("/"),
    children: RefCell::new(Vec::new()),
    parent: RefCell::new(Weak::new())
  });

  const MAX: i32 = 70000000;
  const REQUIRED: i32 = 30000000;


  let mut currentNode = Rc::clone(&root);

  for line in lines {

    if line.chars().nth(0).unwrap() == '$'{
      if line.chars().nth(2).unwrap() == 'c' {


        //get the third word of the line
        let third = line.split_whitespace().nth(2).unwrap();

        if third == ".."{
          println!("going up");
          let thisnode = Rc::clone(&currentNode);
          currentNode = thisnode.parent.borrow().upgrade().unwrap();
        }
        else {

          let newNode = Rc::new(Node {
            value: RefCell::new(0),
            name: String::from(third),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new())
          });

          newNode.add_parent(Rc::downgrade(&currentNode));
          currentNode.add_child(Rc::clone(&newNode));
          currentNode = newNode;

        }

        println!("third: {}", third);
      }
      else {

      }
    }
    else {
      if line.chars().nth(0).unwrap().is_numeric() {
        let value = line.split_whitespace().nth(0).unwrap().parse::<i32>().unwrap();
        currentNode.add_value(value);
        println!("value: {}", currentNode.value.borrow());
      }
    }
  }


  println!("available filespace {}", MAX);
  println!("required filespace {}", REQUIRED);
  
  

  let mut total = 0;
  let used = root.total(&mut total);

  println!("total used {}", used);

  let left = MAX-*used;

  println!("total left {}", left);

  let todelete = REQUIRED - left;

  println!("total to delete {}", todelete);

  let mut posVecs: Vec<i32> = Vec::new();
  let mut sum = 0;

  let tony2 = root.traverse(&mut sum, &mut posVecs, todelete);
  tony2.1.sort();
  println!("possibles: {:?}", tony2.1);


  // println!("root: {:#?}", root);


}