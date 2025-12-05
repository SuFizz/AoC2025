use std::fs;
use std::io;

#[derive(Debug)]
struct Ingrds{
    start_id: u64,
    end_id: u64,
}

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc5_inputs.txt")?;
    let mut ingredient_ids: Vec<&str> = contents.split("\r\n\r\n")
                    .next()
                    .unwrap()
                    .split("\r\n")
                    .filter(|p| !p.is_empty())
                    .collect();

    let mut check_ids: Vec<u64> = contents.split("\r\n\r\n")
                    .last()
                    .unwrap()
                    .split("\r\n")
                    .map(|line| line.parse::<u64>().unwrap())
                    .collect();
    // println!("{:?}", check_ids);
    
    ingredient_ids.sort_by_key(|s| {
        s.split('-')
            .next()
            .unwrap()
            .parse::<u64>()
            .unwrap()
    });
    println!("{:?}", ingredient_ids);

    let mut cons_ingr_ids: Vec<Ingrds> = Vec::<Ingrds>::new();
    let split_ingrds: Vec<_> = ingredient_ids[0].split("-").collect();
    cons_ingr_ids.push(Ingrds{start_id: split_ingrds[0].parse().unwrap(), end_id: split_ingrds[1].parse().unwrap(),});
    for i in 1..ingredient_ids.len(){
        // println!("{} {}", curr_ingr_end, next_ingr_start);
        // println!("{:?}", cons_ingr_ids);
        let split_ingrds: Vec<_> = ingredient_ids[i].split("-").collect();
        let curr_ingr_start: u64 = split_ingrds[0].parse().unwrap();
        let curr_ingr_end: u64 = split_ingrds[1].parse().unwrap();
        let prev_cons_ingr_end: u64 = cons_ingr_ids[cons_ingr_ids.len()-1].end_id;
        let prev_cons_ingr_start: u64 = cons_ingr_ids[cons_ingr_ids.len()-1].start_id;
        cons_ingr_ids.push(Ingrds{start_id: curr_ingr_start, end_id: curr_ingr_end,});
        // println!("{} {}", curr_ingr_start, prev_cons_ingr_end);
        if curr_ingr_start - 1 <= prev_cons_ingr_end{
            cons_ingr_ids.pop();
            cons_ingr_ids.pop();
            let mut end_point = prev_cons_ingr_end;
            if end_point < curr_ingr_end{
                end_point = curr_ingr_end;
            }
            let cons_new_ingr = Ingrds{start_id: prev_cons_ingr_start, end_id: end_point,}; 
            cons_ingr_ids.push(cons_new_ingr);
        }
    }

    println!("{:?}", cons_ingr_ids);

    let mut summer = 0;
    // for i in 0..check_ids.len(){
    //     let chk_id = check_ids[i];

    //     for ingr in &ingredient_ids{
    //         let split_ingrds: Vec<_> = ingr.split("-").collect();
    //         let curr_ingr_start: u64 = split_ingrds[0].parse().unwrap();
    //         let curr_ingr_end: u64 = split_ingrds[1].parse().unwrap();

    //         if chk_id >= curr_ingr_start && chk_id <= curr_ingr_end{
    //             println!("{}, {}, {}", chk_id, curr_ingr_start, curr_ingr_end);
    //             summer = summer + 1;
    //             break;
    //         }
    //     }
    // }

    let mut summi = 0;
    for ingr in &cons_ingr_ids{
        summi = summi + ingr.end_id - ingr.start_id;
        if ingr.start_id != ingr.end_id{
            summi  = summi + 1;
        }
    }
    println!("{}", summi);

    for i in 0..check_ids.len(){
        let chk_id = check_ids[i];
        // println!("{}", chk_id);

        // for ingr in &cons_ingr_ids{
        //     if chk_id >= ingr.start_id && chk_id <= ingr.end_id{
        //         summer = summer + 1;
        //     }
        // }

        let mut start = 0;
        let mut end = cons_ingr_ids.len();
        let mut mid = (start + end)/2;
        while true{
            // println!("s-{} e-{} m-{} mid-{:?}", start, end, mid, cons_ingr_ids[mid]);
            if chk_id >= cons_ingr_ids[mid].start_id && chk_id <= cons_ingr_ids[mid].end_id{
               summer = summer + 1; 
               println!("Found this {}", chk_id);
               break;
            } 
            // else if end == mid || start == mid{
            //     break;
            // }
            else if chk_id > cons_ingr_ids[mid].end_id {
                if start == mid {
                    break;
                }
                start = mid;
                mid = (start + end)/2;
            }
            else if chk_id < cons_ingr_ids[mid].start_id {
                if end == mid {
                    break;
                }
                end = mid;
                mid = (start + end)/2;
            }
        }
    }
    println!("{}", summer);

    Ok(())
}