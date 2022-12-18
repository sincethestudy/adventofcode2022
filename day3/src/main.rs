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

    for line in split {
      length = line.len();
      splitpoint = length/2;
      
      let fh = &line[..splitpoint];
      let sh = &line[splitpoint..];
      
      if fh.len() == 0{
        println!("tony");
        continue;
      }
      println!("{fh} {sh}");
      
      let it: char = find_common_char(fh, sh);
      let idx = items.find(it);
      
      match idx {
        Some(p) => {
          total+=p+1;
          println!("{p}");
        },
        None => { 
          println!("not found");
          continue
        },
      }
    } 
  println!("{total}");
}


fn find_common_char(f: &str, s: &str) -> char {
  let mut ma: char = 'a'; 
  for i in f.chars(){
    for j in s.chars(){
      if i == j {
        ma = i;
        println!("{i}");
        return ma;
      }
    }
  }

  ma
}
