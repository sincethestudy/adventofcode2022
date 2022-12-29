use std::{collections::{HashSet, HashMap}};

fn main() {

    let cmds = include_bytes!("input.txt").split(|b| b == &b'\n');

    let parsed = cmds.map(|l| match (l[0], atoi::atoi::<i32>(&l[2..]).unwrap()) {
        (b'U', l) => ((0, 1), l),
        (b'D', l) => ((0, -1), l),
        (b'L', l) => ((-1, 0), l),
        (_, l) => ((1, 0), l),
    });


    let mut knots: [(i32, i32); 10] = [(0, 0); 10];

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    seen.insert((0, 0));

    for (d, l) in parsed {
        for _ in 0..l {
            knots[0] = (knots[0].0 + d.0, knots[0].1 + d.1);

            for i in 0..9 {
                if knots[i].0.abs_diff(knots[i+1].0) > 1 || knots[i].1.abs_diff(knots[i+1].1) > 1 {
                    knots[i+1] = (knots[i+1].0 + (knots[i].0 - knots[i+1].0).signum(), knots[i+1].1 + (knots[i].1 - knots[i+1].1).signum());
                }
            }

            seen.insert(knots[9]);

        }
    }
    println!("{}", seen.len());
}