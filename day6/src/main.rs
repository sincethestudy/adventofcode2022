use itertools::Itertools;
use std::collections::HashSet;

fn main() {

    
    const npt: &str = include_str!("input.txt");
    
    let mut arr_prev = ['a';14];
    let mut arr_cur = ['a';14];
    //populate first 4

    let mut tmp = npt.chars().enumerate();
    
    for i in 0..14 {
      arr_prev[i] = tmp.next().unwrap().1;
    }
    
    for i in 0..14 {
      arr_cur[i] = tmp.next().unwrap().1;
    }

    let mut found = true;
    
    

    while let Some((l, c)) = tmp.next() {
      found = true;
      for i in 0..14 {
        for j in 0..14 {
          if arr_prev[i] == arr_cur[j]{
            found = false;
          }
        }
      
      }

      
      found = !has_dups(&arr_cur);


      println!("{} {} {} {}", arr_cur[0], arr_cur[1], arr_cur[2], arr_cur[3]);

      if found == true {
        println!("starts at {}", l);
        break;
      }
    
      arr_prev.rotate_left(1);

      arr_prev[13] = arr_cur[0];
      
      arr_cur.rotate_left(1); 
      arr_cur[13] = c;
      
    }
}


fn has_dups(arr: &[char]) -> bool {
  let set: HashSet<_> = arr.iter().collect();
  set.len() != arr.len()

}
