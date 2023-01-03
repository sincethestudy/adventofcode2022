use divrem::{DivRem, DivFloor, RemFloor};

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: (char, usize),
    test: usize,
    pass: usize,
    fail: usize,
    inspects: usize,
}

impl Monkey {

    fn incr(&mut self){
        self.inspects+=1;
    }

    fn addItem(&mut self, item: usize) {
        self.items.push(item);
    }


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
                            .unwrap();
                            
        let mut op_prep: Vec<&str> = togo.split(" ").collect::<Vec<&str>>();
        println!("{:?}", op_prep);

        let test = instrs.next().unwrap().split("Test: divisible by ").last().unwrap().parse::<usize>().unwrap();
        let pass = instrs.next().unwrap().split("If true: throw to monkey ").last().unwrap().parse::<usize>().unwrap();
        let fail = instrs.next().unwrap().split("If false: throw to monkey ").last().unwrap().parse::<usize>().unwrap();

        Self {
            items: items,
            operation: (op_prep[0].chars().next().unwrap(), match op_prep[1].parse() {
                Ok(T) => T,
                Err(E) => 0
            }),

            test: test,
            pass: pass,
            fail: fail,
            inspects: 0,
        }
    }
}


fn main() {
    let input = include_str!("input.txt");
    let mut splitter = input.split("\n\n");

    let mut monkeys: Vec<_> = splitter.map(|a| Monkey::new(a)).collect();

    println!("{:?}", monkeys);

    let mut itemstoadd: Vec<Vec<usize>> = vec![vec![]; monkeys.len()];

    for i in 0..20 {
        monkeys.iter_mut().enumerate().for_each(|(j, m)|
            {
                //add the bagged items
                itemstoadd[j].iter_mut().for_each(|o| {
                    m.addItem(*o)
                });
                itemstoadd[j].clear();

                for item in 0..m.items.len(){
                    let mut newitem = 0;

                    if m.operation.0 == '*' {
                        if m.operation.1 == 0 {
                            newitem = m.items[item] * m.items[item];
                        }
                        else{
                            newitem = m.items[item] * m.operation.1;
                            println!(" mult {} {} ", newitem, m.operation.1);
                        }
                        
                    }
                    else{
                        if m.operation.1 == 0 {
                            newitem = m.items[item] * m.items[item];
                        }
                        else{
                            newitem = m.items[item] + m.operation.1;
                            println!(" adding {} {} ", newitem, m.operation.1);
                        }
                    }

                    newitem = newitem.div_floor(3);

                    let val = newitem.rem_floor(m.test);
                    println!(" val after % {} {} ", val, newitem);

                    if val > 0 {
                        itemstoadd[m.fail].push(newitem);
                    }
                    else {
                        itemstoadd[m.pass].push(newitem);
                    }

                    println!("{:?}" , itemstoadd);
                    m.incr();
                }

                m.items.clear();
        }
            
        ) 
    }

    monkeys.iter().for_each(|m| {
        println!("{}", m.inspects)
    });

    println!("{:?}", itemstoadd);

    println!("{:?}", monkeys);



}
