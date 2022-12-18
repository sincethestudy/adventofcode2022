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

  let mut topthree: [i32; 3] = [0, 0, 0];
  let mut vallow = topthree[0];
  let mut idlow = 0;
  for i in &v {
    //find index of lowest num
    vallow = topthree[0];
    for j in 0..3 {
      if topthree[j] <= vallow {
        vallow = topthree[j];
        idlow = j;
      }
    }
   
    println!("lowest value: {vallow}"); 
    if *i > vallow {
      println!("replacing lowest value with: {i}");
      //replace the lowest index with *i
      topthree[idlow] = *i 
      
    }
  }
  let combined = topthree[0] + topthree[1] + topthree[2];
  println!("{combined}");
}
