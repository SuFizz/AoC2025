use std::fs;
use std::io;

fn is_fake_id(number: u64) -> bool{
    let mut num_mut = number;
    let mut num_ints = 0;
    let mut ints:Vec<u64> = Vec::<u64>::new();
    while 0!=num_mut{
        ints.push(num_mut%10);
        num_mut = num_mut/10;
        num_ints = num_ints + 1;
    }
    // println!("num ints - {} array - {:?}", num_ints, ints);
    let mut flag: bool = false;

    for j in 1..num_ints/2+1{
        if num_ints%j != 0 {
            continue;
        }
        // println!("check stride {}", j);
        flag = false;
        for i in 0..num_ints-j{
            if ints[i]!=ints[i+j]{
                // println!("ints - {} {}", ints[i], ints[i+j]);
                flag = true;
                break;
            }
        }
        if !flag{
            return true;
        }
    }
    return false;
}

fn main() -> io::Result<()>{
    let all_ids: String = fs::read_to_string("aoc2_inputs.txt")?;
    let parts: Vec<&str> = all_ids.split(',').collect();
    let mut summer: u64 = 0;
    //println!("{:?}", parts);

    for ids in &parts{
        let start_fin: Vec<&str> = ids.split("-").collect();
        let start = start_fin[0 as usize].parse::<u64>().unwrap();
        let end = start_fin[1 as usize].parse::<u64>().unwrap();
        println!("start {} end {}", start, end);
        for i in start..end+1{
            if is_fake_id(i){
                println!("{}",i);
                summer = summer + i;
            }
        }
    }
    println!("{}", summer);
    Ok(())
}