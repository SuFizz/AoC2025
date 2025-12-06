use std::fs;
use std::io;

// #[derive(Debug)]
// struct MathProb{
//     first_num: u64,
//     second_num: u64,
//     is_mul: bool,
//     is_add: bool,
// }

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc6_inputs.txt")?;
    let split_contents: Vec<_> = contents.split("\r\n").map(|line| line.chars().collect::<Vec<_>>()).collect();
    // let mut first_nums: Vec<_> = split_contents[0].split_whitespace().map(|line| line.parse::<u64>().unwrap()).collect();
    // let mut second_nums: Vec<_> = split_contents[1].split_whitespace().map(|line| line.parse::<u64>().unwrap()).collect();
    // let mut third_nums: Vec<_> = split_contents[2].split_whitespace().map(|line| line.parse::<u64>().unwrap()).collect();
    // let mut fourth_nums: Vec<_> = split_contents[3].split_whitespace().map(|line| line.parse::<u64>().unwrap()).collect();
    let operations: Vec<char> = contents.split("\r\n").last().expect("REASON").split_whitespace().filter_map(|s| s.chars().next()).collect();

    let mut k = -1isize;
    let mut summer_fin: u64 = 0;
    let mut summer_in: u64 = 0;

    for i in 0..split_contents[0].len(){
        if split_contents[4][i as usize] == '*'{
            k = k+1;
            summer_fin = summer_fin + summer_in;
            summer_in = 1;
            println!("sum fin - {}", summer_fin);
        }
        else if split_contents[4][i as usize] == '+'{
            k = k+1;
            summer_fin = summer_fin + summer_in;
            summer_in = 0;
            println!("sum fin - {}", summer_fin);
        }
        let mut number: u64 = 0;
        let mut atleast_one_digit = false;
        for j in 0..4{
            if split_contents[j][i as usize] != ' '{
                atleast_one_digit = true;
                number = number * 10 + split_contents[j][i as usize].to_digit(10).unwrap() as u64;
            }
        }
        if operations[k as usize] == '*' && atleast_one_digit{
            summer_in = summer_in * number;
        }
        else if operations[k as usize] == '+' && atleast_one_digit{
            summer_in = summer_in + number;
        }
        println!("num-{} k-{} summer_in-{}", number, k, summer_in);
    }

    // println!("{:?}", first_nums);
    // println!("{:?}", second_nums);
    // println!("{:?}", operations);

    // let mut summer: u64 = 0;

    // for i in 0..operations.len(){
    //     if operations[i] == '*'{
    //         summer = summer + (first_nums[i]*second_nums[i] * third_nums[i] * fourth_nums[i]);
    //     }
    //     else if operations[i] == '+'{
    //         summer = summer + first_nums[i] + second_nums[i] + third_nums[i] + fourth_nums[i];
    //     }
    // }

    summer_fin = summer_fin + summer_in;
    println!("{}", summer_fin);
    Ok(())
}