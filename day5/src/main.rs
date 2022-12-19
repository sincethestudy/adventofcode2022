
fn main() {
  const strt: &str = include_str!("input_starting.txt");
  const prcdr: &str = include_str!("input_procedure.txt");
 
  const mxsz: usize = strt.len()/3;
  const stcks: u8 = strt.as_bytes()[strt.len()-1]-1;
  
  const st: usize = stcks as usize;  

  //store letters as their 1-26 values from alphabet
  let array: [[usize;mxsz]; st] = [[0;mxsz]; st]; 

 
  println!("crates: {}", stcks);
   
  let mut lines = strt.lines();

  for i in 0..stcks {
    while let Some(line) = lines.next() {
      let crt = &line[0..3];
      array[i]
      println!("Got line: {}", crt);
    }
  }
}
