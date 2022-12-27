fn main() {
    let input: &str = include_str!("input.txt");
    let lines: std::str::Lines  = input.lines();

    let mut visible = 0;

    let cols: usize = input.lines().into_iter().next().unwrap().chars().count();
    let rows: usize = input.lines().count();

    println!("row: {}, cols: {}", rows, cols);

    let mut forestMatrix: Vec<Vec<u32>> = vec![vec![0;cols];rows];

    let mut temp = 0;

    for line in lines {
        for i in 0..line.chars().count() {
            forestMatrix[temp][i] = line.chars().nth(i).unwrap().to_digit(10).unwrap();
        }
        temp += 1;
    }

    for i in 0..rows {
        'inner: for j in 0..cols {
            let mut curvis = true;
            let height = forestMatrix[i][j];

            
            for h in i+1..rows{
                curvis = forestMatrix[h][j]<height && curvis;
            }

            if curvis{
                visible += 1;
                // println!("status added from downwards at {} {}", i ,j);
                continue 'inner;
            }
            curvis = true;

            for h in 0..i{
                curvis = forestMatrix[h][j]<height && curvis;
            }
            if curvis{
                visible += 1;
                // println!("status added at {} {}", i ,j);
                continue 'inner;
            }
            curvis = true;

            for h in j+1..cols{
                curvis = forestMatrix[i][h]<height && curvis;
            }
            if curvis{
                visible += 1;
                // println!("status added at {} {}", i ,j);
                continue 'inner;
            }
            curvis = true;

            for h in 0..j{
                curvis = forestMatrix[i][h]<height && curvis;
            }
            if curvis{
                visible += 1;
                // println!("status added at {} {}", i ,j);
                continue 'inner;
            }


            if i == 0 || i == (rows) || j == 0 || j == (cols) {
                visible += 1;
                println!("got an outer edge {} {}", i, j);
                continue 'inner;
            }

    
        }
    }

    println!("visible: {}", visible);



    // println!("{} {} ", cols, rows);
    // println!("{:?}", forestMatrix);




}

