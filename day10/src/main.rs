use divrem::{DivRem, DivFloor};

fn check(cycle: i32,x: i32, mut sum:  i32 ) -> i32{
    if (cycle == 20 || (cycle-20)%40==0) && cycle <= 220{
        sum += cycle * x;
    }
    sum
}

fn draw(cycle: i32, x: i32, mut display: [[i32;40];6]) -> [[i32;40];6]{
    let i: usize = cycle.div_floor(40).try_into().unwrap();
    let j: usize = (cycle%40).try_into().unwrap();

    println!("{}", j);

    let jt: i32 = (cycle%40).try_into().unwrap();

    if (x-1) == jt || x == jt || (x+1)==jt {
        display[i][j] = 1;
    }

    display
}



fn main() {
    let cmds = include_str!("input.txt").lines();

    let mut x = 1;
    let mut cycle: i32 = 0;
    let mut sum = 0;

    let mut crt: [[i32;40];6] = [[0;40];6];

    for line in cmds {
        if line == "noop" {
            crt = draw(cycle, x, crt);
            cycle += 1 ;
        }
        else {
            crt = draw(cycle, x, crt);
            cycle += 1;
            crt = draw(cycle, x, crt);
            cycle += 1;
            x += line.split_whitespace().last().unwrap().parse::<i32>().unwrap();


        }

    }

    for i in 0..6 {
        println!("{:?}", crt[i]);
    }


}
