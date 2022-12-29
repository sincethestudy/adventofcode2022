fn check(cycle: i32,x: i32, mut sum:  i32 ) -> i32{
    if (cycle == 20 || (cycle-20)%40==0) && cycle <= 220{
        sum += cycle * x;
        println!("clock: {}, x: {}, sum: {}", cycle, x, sum);
    }

    sum
}



fn main() {
    let cmds = include_str!("input.txt").lines();

    let mut x = 1;
    let mut cycle = 0;

    let mut sum = 0;

    for line in cmds {
        if line == "noop" {
            cycle += 1 ;
            sum = check(cycle, x, sum);
        }
        else {
            cycle += 1;
            sum = check(cycle, x, sum);

            cycle += 1;
            sum = check(cycle, x, sum);


            x += line.split_whitespace().last().unwrap().parse::<i32>().unwrap();

            println!("{} {}", cycle, x);

        }

    }

    println!("{}", sum);
}
