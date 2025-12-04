use std::fs;
use std::io;

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc4_inputs.txt")?;
    let mut summer = 0;
    let mut locations: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let movements = vec![-1isize,0,1];

    while true {
        let summer_at_start = summer;
    for i in 0..locations.len(){
        for j in 0..locations[i].len(){
            if locations[i][j] == '@'{
                // println!("{} {}", i, j);
                let mut sume = 0;
                for &x in &movements{
                    for &y in &movements{
                        let new_i = i as isize + x;
                        let new_j = j as isize + y;
                        if new_i >=0 && new_j >= 0 {
                            let new_i = new_i as usize;
                            let new_j = new_j as usize;
                            if new_i < locations.len() && new_j < locations[0].len(){
                                // println!("22-- {} {} x-{} y-{}", new_i, new_j, x, y);
                                if locations[new_i][new_j] == '@'{
                                    if !(x ==0 && y==0){
                                        sume = sume + 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if sume < 4{
                    locations[i][j] = '.';
                    println!("i{} j{} sume{}", i, j, sume);
                    summer = summer + 1;
                }
            }
        }
    }
    if summer_at_start == summer{
        break;
    }
    }
    println!("{}", summer);
    Ok(())
}