
fn main() {
  const strt: &str  = include_str!("input_starting.txt");
  const prcdr: &str = include_str!("input_procedure.txt");
 
  const mxsz: usize = strt.len()/3;
  const stcks: u8 = strt.as_bytes()[strt.len()-1]-1;
  const st: usize = stcks as usize;
  const vect: Vec<i32> = Vec::new();

  //store letters as their 1-26 values from alphabet
  let mut array: [Vec<i32>; st] = [vect; st];  
  println!("crates: {}", stcks);
  
  let col_lngth: usize = (stcks*3 + stcks-1).into();
  println!("length of row: {}", col_lngth);

  let mut lines = strt.lines();

  let crt_typs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

  for i in 0..col_lngth {
    while let Some(line) = lines.next() {
        let line_bytes = line.as_bytes();
        for i in 0..line_bytes.len() {
          
          println!("{} and its line {}", line_bytes[i] as char, (((i as f32)/4.0)).ceil());
        } 
  

 
      //let crt = line[0..3].chars().next().expect("string empty") as i32;
      //array[i].push(crt);
      //println!("Got line: {}", crt);
    }
  }
}
