use std::fs;

fn main() {
    println!("Hello, world!");

    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let items = items.to_string();
   
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path).expect("file not read");
    let split = contents.split("\n");

    let mut length = 0;
    let mut splitpoint = 0;
    
    let mut total = 0;
  
    let mut group = 0;
    let mut rucks = ["temp", "temp", "temp"];

    let mut common = 'a';

    for line in split { 
      rucks[group] = line;
      group += 1;
      
      if group != 3{
        continue;
      }  
      else {
        common = find_common_char(rucks[0], rucks[1], rucks[2]);
        println!("{common}");
        group = 0;                

        let idx = items.find(common);

        match idx {
          Some(p) => {
            total += p+1;
            println!("{p}");
          },
          None => continue,
        }
      }   
    }

  println!("{total}"); 
}


fn find_common_char(f: &str, s: &str, r: &str) -> char {
  let mut ma: char = 'a'; 
  for i in f.chars(){
    for j in s.chars(){
      for k in r.chars(){
        
        if i == j && i == k {
          ma = i;
          println!("{i}");
          return ma;
       

         }
      }
    }
  }

  ma
}
