use std::thread::current;

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

    let mut topscore = 0;

    for i in 0..rows {
        'inner: for j in 0..cols {
            let height = forestMatrix[i][j];

            //downscore
            let mut downscore = 0;
            
            'downscore: for h in i+1..rows{
                if i == rows {
                    break 'downscore;
                }
                else if forestMatrix[h][j]<height{
                    downscore += 1;
                }
                else if forestMatrix[h][j]>=height{
                    downscore += 1;
                    break 'downscore;
                } 
            }

            //upscore
            let mut upscore = 0;
            
            'upscore: for h in (0..i).rev(){
                if i == 0 {
                    break 'upscore;
                }
                else if forestMatrix[h][j]<height{
                    upscore += 1;
                }
                else if forestMatrix[h][j]>=height{
                    upscore += 1;
                    break 'upscore;
                } 
            }

            //leftscore
            let mut leftscore = 0;
            
            'leftscore: for h in (0..j).rev(){
                if j == 0 {
                    break 'leftscore;
                }
                else if forestMatrix[i][h]<height{
                    leftscore += 1;
                }
                else if forestMatrix[i][h]>=height{
                    leftscore += 1;
                    break 'leftscore;
                } 
            }

            //rightscore
            let mut rightscore = 0;
            
            'rightscore: for h in j+1..cols{
                if j == cols {
                    break 'rightscore;
                }
                else if forestMatrix[i][h]<height{
                    rightscore += 1;
                }
                else if forestMatrix[i][h]>=height{
                    rightscore += 1;
                    break 'rightscore;
                } 
            }



            let currentScore = downscore * upscore * leftscore * rightscore;
            println!("{} {} {} {} {}", currentScore, downscore, upscore, leftscore, rightscore);


            topscore = std::cmp::max(topscore, currentScore);
    
        }
    }

    println!("top: {}", topscore);



    // println!("{} {} ", cols, rows);
    // println!("{:?}", forestMatrix);




}

