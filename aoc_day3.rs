use std::fs;
use std::io;

fn find_largest(numbers: Vec<u32>) -> u32
{
    let num_len = numbers.len();
    let mut dig_r = numbers[num_len - 1];
    let mut dig_l = numbers[num_len - 2];
    for i in (0..num_len-2).rev(){
        let tmp_dig_l = dig_l;
        if numbers[i] >= dig_l{
            dig_l = numbers[i];
            if tmp_dig_l > dig_r{
                dig_r = tmp_dig_l;
            }
        }
    }
    return 10*dig_l +dig_r;
}

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc3_inputs.txt")?;
    let mut summer = 0;
    for line in contents.lines(){
        let numbers: Vec<u32> = line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        // println!("{:?}", numbers);
        let largest = find_largest(numbers);
        summer = summer + largest;
        println!("{}", largest);
    }
    println!("{}", summer);
    Ok(())
}