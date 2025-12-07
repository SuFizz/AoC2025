use std::fs;
use std::convert::TryInto;
use std::io;

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc7_inputs.txt")?;
    let locations: Vec<Vec<char>> = contents.lines().map(|line| line.chars().collect()).collect();
    let mut splitters: Vec<Vec<u32>> = Vec::new();
    let mut beam_location: Vec<u32> = Vec::<u32>::new();
    
    for j in 0..locations[0].len(){
        if locations[0][j] == 'S'{
            beam_location.push(j.try_into().unwrap());
            break;
        }
    }
    

    for i in 1..locations.len(){
        let mut splitter_location: Vec<u32> = Vec::<u32>::new();
        for j in 0..locations[i].len(){
            if locations[i][j] == '^'{
                splitter_location.push(j.try_into().unwrap());
            }
        }
        splitters.push(splitter_location);
    }

    let mut counter = 0;
    for i in 1..splitters.len(){
        let mut next_beam_locations: Vec<u32> = Vec::<u32>::new();
        let mut last_added_beam_location: u32 = locations[0].len().try_into().unwrap();
        println!("beam location - {:?}", beam_location);
        println!("splitters[{}] - {:?}", i, splitters[i]);
        for b in 0..beam_location.len(){
            let mut flag = false;
            for spl in &splitters[i]{
                println!("{} {} {:?}", beam_location[b], *spl, next_beam_locations);
                if beam_location[b] == *spl{
                    counter = counter + 1;
                    if *spl-1 != last_added_beam_location && *spl>=1{
                        next_beam_locations.push(*spl - 1);
                    }
                    if *spl+1 < locations[i].len().try_into().unwrap(){
                        next_beam_locations.push(*spl + 1);
                        last_added_beam_location = *spl+1;
                    }
                    flag = true;
                    break;
                }
            }
            if !flag{
                if beam_location[b] != last_added_beam_location{
                    next_beam_locations.push(beam_location[b]);
                    last_added_beam_location = beam_location[b];
                }
            }
        }
        beam_location = next_beam_locations;
    } 
    println!("counter {}", counter);
    Ok(())
}