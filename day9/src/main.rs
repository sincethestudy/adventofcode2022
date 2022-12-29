use std::{collections::{HashSet, HashMap}};

fn main() {

    let cmds = include_bytes!("input.txt").split(|b| b == &b'\n');

    let parsed = cmds.map(|l| match (l[0], atoi::atoi::<i32>(&l[2..]).unwrap()) {
        (b'U', l) => ((0, 1), l),
        (b'D', l) => ((0, -1), l),
        (b'L', l) => ((-1, 0), l),
        (_, l) => ((1, 0), l),
    });

    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    seen.insert((0, 0));

    for (d, l) in parsed {
        for _ in 0..l {
            head = (head.0 + d.0, head.1 + d.1);

            if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                tail = (head.0 - d.0, head.1 - d.1);
                seen.insert(tail);
            }
        }
    }


    println!("{}", seen.len());






    
    // for cmd in cmds {
    //     println!("{:?}", atoi::atoi::<i32>(&cmd[2..]).unwrap());
    // }

}