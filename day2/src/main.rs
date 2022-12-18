use std::fs;
use std::collections::HashMap;


fn main() {
  let file_path = "src/input.txt";
  let contents = fs::read_to_string(file_path).expect("file not read");

  let split = contents.split("\n");


  let mut scores = HashMap::new();
  scores.insert(String::from("X"), 1);
  scores.insert(String::from("Y"), 2);
  scores.insert(String::from("Z"), 3);

  let mut outcomes = HashMap::new();
  outcomes.insert(String::from("A X"), 4);
  outcomes.insert(String::from("A Y"), 8);
  outcomes.insert(String::from("A Z"), 3);
  outcomes.insert(String::from("B X"), 1);
  outcomes.insert(String::from("B Y"), 5);
  outcomes.insert(String::from("B Z"), 9);
  outcomes.insert(String::from("C X"), 7);
  outcomes.insert(String::from("C Y"), 2);
  outcomes.insert(String::from("C Z"), 6);

  let mut total = 0;
  for i in split {
      println!("{i}");

      total += outcomes.get(i).copied().unwrap_or(0);
  }

  println!("total: {}", total);
}
