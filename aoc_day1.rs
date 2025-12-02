use std::fs;

fn main() {
    let mut result = 50;
    let mut counter = 0;
    let contents = fs::read_to_string("aoc1_input.txt").unwrap();
    
    for line in contents.lines() {
        let rot = line.as_bytes()[0] as char;
        let num: i32 = line[1..].trim().parse().unwrap();
        result = match rot{
            'L' => {
                let new_val = result - num%100;
                counter = counter + num/100;
                if new_val < 0 {
                    if result !=0 {
                        counter = counter + 1;
                    }
                    100 + new_val
                }
                else {
                    if new_val == 0 {
                        counter = counter + 1;
                    }
                    new_val 
                }
            },
            'R' => {
                counter = counter + (result + num)/100;
                // if (result + num)%100 == 0 && (result + num) != 100{
                //     counter = counter + 1
                // } 
                (result + num)%100
            },
            _ => result,
        };

        println!("{}", line);
        println!("counter{}", counter);
        println!("result{}", result);
    }
    println!("{}",counter);
}