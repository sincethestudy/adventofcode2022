use std::borrow::Borrow;
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

  fn traverse<'a>(&self, totalSum: &'a mut i32) -> (i32,&'a mut i32) {
    // println!("traversing: {}", self.name);
    let mut localsum = *self.value.borrow();


    for child in self.children.borrow().iter() {
      // println!("child: {}", child.name);
      // println!("child value: {}", child.value.borrow());
      let temp = child.traverse(totalSum);
      localsum += temp.0;
    }

    let mut localsumsen = totalSum;

    if localsum <= 100000 {
      println!("return localsum: {}", localsum);
      // *totalSum.borrow_mut() += localsum;
      *localsumsen += localsum;
    }

   
    println!("return localsumsen: {}", localsumsen);

    (localsum, localsumsen)
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
          // if *currentNode.value.borrow() <= 100000{
          //   println!("adding value: {}", *currentNode.value.borrow());
          //   sum += *currentNode.value.borrow();
          // }

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

  let mut sum = 0;

  let tony = root.traverse(&mut sum);

  println!("total: {}", tony.1);

  // println!("root: {:#?}", root);


}