use std::fs;
use std::io;

fn find_largest(numbers: Vec<u64>) -> u64
{
    let num_len = numbers.len();
    let mut dig0 = numbers[num_len - 1 -0];
    let mut dig1 = numbers[num_len - 1 -1];
    let mut dig2 = numbers[num_len - 1 -2];
    let mut dig3 = numbers[num_len - 1 -3];
    let mut dig4 = numbers[num_len - 1 -4];
    let mut dig5 = numbers[num_len - 1 -5];
    let mut dig6 = numbers[num_len - 1 -6];
    let mut dig7 = numbers[num_len - 1 -7];
    let mut dig8 = numbers[num_len - 1 -8];
    let mut dig9 = numbers[num_len - 1 -9];
    let mut dig10 = numbers[num_len - 1 -10];
    let mut dig11 = numbers[num_len - 1 -11];

    for i in (0..num_len-12).rev(){
        if numbers[i] >= dig11 {
            let tmp_dig11 = dig11;
            dig11 = numbers[i];
            if tmp_dig11 >= dig10 {
                let tmp_dig10 = dig10;
                dig10 = tmp_dig11;
            if tmp_dig10 >= dig9 {
                let tmp_dig9 = dig9;
                dig9 = tmp_dig10;
            if tmp_dig9 >= dig8 {
                let tmp_dig8 = dig8;
                dig8 = tmp_dig9;
            if tmp_dig8 >= dig7 {
                let tmp_dig7 = dig7;
                dig7 = tmp_dig8;
            if tmp_dig7 >= dig6 {
                let tmp_dig6 = dig6;
                dig6 = tmp_dig7;
            if tmp_dig6 >= dig5 {
                let tmp_dig5 = dig5;
                dig5 = tmp_dig6;
            if tmp_dig5 >= dig4 {
                let tmp_dig4 = dig4;
                dig4 = tmp_dig5;
            if tmp_dig4 >= dig3 {
                let tmp_dig3 = dig3;
                dig3 = tmp_dig4;
            if tmp_dig3 >= dig2 {
                let tmp_dig2 = dig2;
                dig2 = tmp_dig3;
            if tmp_dig2 >= dig1 {
                let tmp_dig1 = dig1;
                dig1 = tmp_dig2;
            if tmp_dig1 >= dig0 {
                let tmp_dig0 = dig0;
                dig0 = tmp_dig1;
            }
            }}}}}}}}}}
        }
    }
    let ret: u64 = 10_u64.pow(11)*dig11 + 
              10_u64.pow(10)*dig10 + 
              10_u64.pow(9)*dig9 + 
              10_u64.pow(8)*dig8 + 
              10_u64.pow(7)*dig7 + 
              10_u64.pow(6)*dig6 + 
              10_u64.pow(5)*dig5 + 
              10_u64.pow(4)*dig4 + 
              10_u64.pow(3)*dig3 + 
              10_u64.pow(2)*dig2 + 
              10_u64.pow(1)*dig1 + 
              dig0 ; 
    return ret;
}

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc3_inputs.txt")?;
    let mut summer: u64 = 0;
    for line in contents.lines(){
        let numbers: Vec<u64> = line.chars()
            .filter_map(|c| c.to_digit(10))
             .map(|d| d as u64)
            .collect();
        // println!("{:?}", numbers);
        let largest = find_largest(numbers);
        summer = summer + largest;
        println!("{}", largest);
    }
    println!("{}", summer);
    Ok(())
}