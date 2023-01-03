#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: (char, usize),
    test: usize,
    pass: usize,
    fail: usize
}

impl Monkey {
    fn new(details: &str) -> Self {
        let mut instrs = details.split("\n");

        let items = instrs
                                    .nth(1)
                                    .unwrap()
                                    .replace(" ", "")
                                    .split(":")
                                    .last()
                                    .unwrap()
                                    .split(",")
                                    .map(|x| x.parse().unwrap())
                                    .collect::<Vec<usize>>();

        let togo = instrs
                            .next()
                            .unwrap()
                            .split("Operation: new = old ")
                            .last()
                            .unwrap()
                            .replace(" ", "");
                            
        let mut op_prep = togo.chars();

        let test = instrs.next().unwrap().split("Test: divisible by ").last().unwrap().parse::<usize>().unwrap();
        let pass = instrs.next().unwrap().split("If true: throw to monkey ").last().unwrap().parse::<usize>().unwrap();
        let fail = instrs.next().unwrap().split("If false: throw to monkey ").last().unwrap().parse::<usize>().unwrap();

        Self {
            items: items,
            operation: (op_prep.next().unwrap(), match op_prep.next().unwrap().to_digit(10){
                Some(T) => T.try_into().unwrap(),
                None => 0
            }),

            test: test,
            pass: pass,
            fail: fail,
        }
    }
}


fn main() {
    let input = include_str!("input.txt");
    let mut splitter = input.split("\n\n");

    let monkeys: Vec<_> = splitter.map(|a| Monkey::new(a)).collect();

    println!("{:?}", monkeys);



}
