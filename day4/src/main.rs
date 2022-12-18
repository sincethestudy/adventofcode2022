use std::fs;
use itertools::Itertools;

fn main() {
  let file_path = "src/input.txt";
  let contents = fs::read_to_string(file_path).expect("file not read");
  let split = contents.split("\n");
  let mut s = ',';
  let mut idex = 0;  

  let mut total = 0;

  for pair in split {
     let idx = pair.find(s);
    
    match idx {
      Some(p) => idex = p,
      None => continue,
    }

    let (f, s) = pair.split(',').next_tuple().unwrap();
            
    let (f1, f2) = f.split('-').next_tuple().unwrap();
    let (s1, s2) = s.split('-').next_tuple().unwrap();

    let f1: i32 = f1.parse().unwrap();
    let f2: i32 = f2.parse().unwrap();
    let s1: i32 = s1.parse().unwrap();
    let s2: i32 = s2.parse().unwrap();

    println!("{f1} {f2} {s1} {s2}");
    

    if (s1<=f1 && s2>=f2) || (f1<=s1 && f2>=s2) {
      total += 1;
    } 


    println!("{total}");

  }  
}
