
fn main() {
  const strt: &str  = include_str!("input_starting.txt");
  const prcdr: &str = include_str!("input_procedure.txt");
 
  const mxsz: usize = strt.len()/3;
  const stcks: u8 = strt.as_bytes()[strt.len()-1]-1;
  const st: usize = stcks as usize;
  const vect: Vec<u32> = Vec::new();

  //store letters as their 1-26 values from alphabet
  let mut array: [Vec<u32>; st] = [vect; st];  
  println!("crates: {}", stcks);
  
  let col_lngth: usize = (stcks*3 + stcks-1).into();
  println!("length of row: {}", col_lngth);

  let mut lines = strt.lines().enumerate();

  let crt_typs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

  while let Some((j,line)) = lines.next() {
    for i in 0..col_lngth {  

      let line_bytes = line.as_bytes();
      for i in 0..line_bytes.len() {
        let crt_ln = ((i as f32)/4.0).ceil() as usize;
        let crt = line_bytes[i] as char;

        match crt_typs.find(crt) {
         Some(status) => {
            println!("crate {} is going into stack {}", crt, crt_ln);
            array[crt_ln-1].push(status as u32);
          },
          None => println!("not a crate"),
        }
         // println!("{} and its line {}", crt, crt_ln);
      } 
 

 
    }
  }
}
