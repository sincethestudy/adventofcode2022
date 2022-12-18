use std::env;
use std::fs;
use std::cmp;

fn main() {
  let mut v: Vec<i32> = Vec::new();
  
  let file_path = "src/input.txt";
  let contents = fs::read_to_string(file_path).expect("file not read");
    
  let mut curper = 0;  
  let mut curcal = 0;
  for line in contents.split("\n") {
    if line == "" {
      v.push(curper);
      curper = 0;
      continue
    }
    
    curcal = line.trim().parse().expect("broke");
    curper += curcal
  }

  let mut most = 0;

  for i in &v {
    println!("{i}");
    most = cmp::max(most, *i);
  }
  
  println!("the highest calories is: {most}"); 
}
