use std::fs;
use std::convert::TryInto;
use std::io;

fn number_beams(loc_v: usize, loc_h: usize, locations: &Vec<Vec<char>>, available_number: &mut Vec<Vec<u64>>) -> u64{
    if available_number[loc_v][loc_h] != 0 {
        return available_number[loc_v][loc_h];
    }
    if locations[loc_v + 1][loc_h] == '^' {
        if loc_h == 0 {
            // println!("beam - ({},{}) splitter - ({},{}) sum of ({},{})", loc_v, loc_h, loc_v+1, loc_h, loc_v+1, loc_h+1);
            let num_beams = number_beams(loc_v + 1, loc_h + 1, locations, available_number);
            available_number[loc_v][loc_h] = num_beams;
            return num_beams;
        }
        if loc_h == locations[loc_v].len() {
            // println!("beam - ({},{}) splitter - ({},{}) sum of ({},{})", loc_v, loc_h, loc_v+1, loc_h, loc_v+1, loc_h-1);
            let num_beams = number_beams(loc_v + 1, loc_h - 1, locations, available_number);
            available_number[loc_v][loc_h] = num_beams;
            return num_beams;
        }
        else {
            // println!("beam - ({},{}) splitter - ({},{}) sum of ({},{}), ({},{})", loc_v, loc_h, loc_v+1, loc_h, loc_v+1, loc_h-1, loc_v+1, loc_h+1);
            let num_beams: u64 = number_beams(loc_v + 1, loc_h - 1, locations, available_number) + number_beams (loc_v + 1, loc_h + 1, locations, available_number);
            available_number[loc_v][loc_h] = num_beams;
            return num_beams;
        }
    }
    let num_beams = number_beams(loc_v+1, loc_h, locations, available_number);
    available_number[loc_v][loc_h] = num_beams;
    return num_beams;
}


fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc7_inputs.txt")?;
    let locations: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut start_location = 0;
    
    for j in 0..locations[0].len(){
        if locations[0][j] == 'S'{
            start_location = j;
            break;
        }
    }
    let available_number: Vec<Vec<u32>> = Vec::new();
    let mut avl_nums = vec![vec![0u64; locations[0].len()]; locations.len()];
    avl_nums[locations.len()-1].fill(1);
    let num_beams = number_beams(0, start_location, &locations, &mut avl_nums);
    println! ("{}", num_beams);
    Ok(())
}