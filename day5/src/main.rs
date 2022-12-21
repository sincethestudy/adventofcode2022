use regex::Regex;


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
//    for i in 0..col_lngth {  

      let mut line_bytes = line.as_bytes();
      for i in 0..line_bytes.len() {
        let mut crt_ln = ((i as f32)/4.0).ceil() as usize;
        let mut crt = line_bytes[i] as char;

        match crt_typs.find(crt) {
         Some(status) => {
            println!("crate {} is going into stack {}", crt, crt_ln);
            array[crt_ln-1].push(status as u32);
          },
          None => println!("not a crate"),
        }
         // println!("{} and its line {}", crt, crt_ln);
      } 
  //  }
  }


  for r in 0..st {
    array[r].reverse();
  }
  //moves
  
  let mut re = Regex::new(r"\d+").unwrap();
  let mut moov = prcdr.lines();
  
  while let Some(pcr) = moov.next() {
    let mut todo: [usize; 3] = [0;3];
    for (j, mat) in re.find_iter(pcr).enumerate() {
      todo[j] = pcr[mat.start()..mat.end()].parse::<usize>().unwrap();
      println!("{}", todo[j]);
    }

    let mut popped: u32 = 0;
    let mut pushed: u32 = 0;
    //do the move
    for k in 0..todo[0] {
      
      println!("the top of {} is {}", todo[1], array[todo[1]-1].last().unwrap()); 
      popped = array[todo[1]-1].pop().unwrap();
      println!("popped: {}", popped);

      array[todo[2]-1].push(popped);
      
      println!("moved {} from {} to {}", popped, todo[1], todo[2]);
      //println!("moving {} from {} to {}", todo[0], array[todo[1]-1].pop().unwrap(), todo[2]); 
    }
  }
  
  for stacks in 0..st {
    let u32refval = *array[stacks].last().unwrap() as usize;
    let charval = crt_typs.chars().nth(u32refval).unwrap();
    println!("array 1 final: {}", charval);
  }

}
