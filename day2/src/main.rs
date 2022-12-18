use std::fs;
use std::collections::HashMap;


fn main() {
  let file_path = "src/input.txt";
  let contents = fs::read_to_string(file_path).expect("file not read");

  let split = contents.split("\n");

  let mut outcomes = HashMap::new();
  outcomes.insert(String::from("A X"), 3);
  outcomes.insert(String::from("A Y"), 4);
  outcomes.insert(String::from("A Z"), 8);
  outcomes.insert(String::from("B X"), 1);
  outcomes.insert(String::from("B Y"), 5);
  outcomes.insert(String::from("B Z"), 9);
  outcomes.insert(String::from("C X"), 2);
  outcomes.insert(String::from("C Y"), 6);
  outcomes.insert(String::from("C Z"), 7);

  let mut total = 0;
  let mut mapped = 0;
  for i in split {
      mapped = outcomes.get(i).copied().unwrap_or(0);
      println!("{i} and the score is : {mapped}");

      total += mapped;
  }

  println!("total: {}", total);
}
