use std::fs;
use std::io;

fn is_fake_id(number: u64) -> bool{
    let mut other_num = number;
    let mut num_ints = 0;
    let mut ints:Vec<u64> = Vec::<u64>::new();
    while 0 != other_num{
        ints.push(other_num % 10_u64);
        other_num = other_num/10_u64;
        num_ints = num_ints + 1;
    }
    //println!("num_ints {} ints {:?}",num_ints, ints);

    if num_ints % 2 != 0{
        //println!("odd number of ints");
        return false;
    }
    else {
        for i in 0..num_ints/2{
            if ints[i as usize] != ints[(num_ints/2 + i) as usize]{
                //println!("mismatch at {} and {} ints {} {}", i, num_ints/2+i, ints[i as usize], ints[(num_ints/2 + i) as usize]);
                return false;
            }
        }
    }
    //println!("likely fake id");
    return true;
}

// fn find_next_fake_id_nums(start: u64, end: u64){
//     // first do the repeat of singles
//     let mut ones_start = start;
//     let mut ones_end   = end;
//     let mut fin_start;
//     let mut fin_end;
//     let mut num_ints = 0;
//     let mut num_end_ints = 0;
//     let mut fake_ids: Vec<u64> = Vec::<u64>::new();
    
//     while ones_start != 0{
//         fin_start = ones_start;
//         ones_start = ones_start/10;
//         num_ints = num_ints + 1;
//     }

//     while ones_end != 0{
//         fin_end = ones_end;
//         ones_end = ones_end/10;
//         num_end_ints = num_end_ints + 1;
//     }

//     if num_end_ints != num_ints{
//         for i in fin_start..10{
//             dump_num = 0;
//             let mut powerer = 1;
//             for j in 0..num_ints{
//                 dump_num = dump_num + powerer*i;
//                 powerer = powerer * 10;
//             }
//             if dump_num >= start && dump_num <= end{
//                 fake_ids.push(dump_num);
//             }
//             else if (dump_num > end)
//             {
//                 break;
//             }
//         }
//         for i in 0..ones_end{
//             dump_num = 0;
//             let mut powerer = 1;
//             for j in 0..num_ints{
//                 dump_num = dump_num + powerer*i;
//                 powerer = powerer * 10;
//             }
//             if dump_num >= start && dump_num <= end{
//                 fake_ids.push(dump_num);
//             }
//             else if (dump_num > end)
//             {
//                 break;
//             }
//         }
//     }

// }

fn find_next_fake_id(number: u64) -> u64{
    let mut other_num = number;
    let mut num_ints = 0;
    let mut ints:Vec<u64> = Vec::<u64>::new();
    while other_num != 0{
        ints.push(other_num % 10_u64);
        other_num = other_num/10_u64;
        num_ints = num_ints + 1;
    }
    //println!("num ints {}", num_ints);

    if 0!=num_ints % 2 {
        let first_thing = 10_u64.pow(num_ints/2);
        let second_thing = 10_u64.pow(num_ints); 
        //println!("{}", second_thing + first_thing);
        return second_thing + first_thing;
    }
    else {
        let mut flag = false;
        //println!("ints first {:?}", ints);
        for i in 0..num_ints/2{
            if ints[i as usize] != ints[(num_ints/2 + i) as usize]{
                ints[i as usize] = ints[(num_ints/2 + i) as usize];
                flag = true;
            }
        }
        if flag{
            //println!("ints next {:?}", ints);
            let mut summer = 0;
            for i in 0..num_ints{
                summer = summer + (10_u64.pow(i))*(ints[i as usize]);
            }
            //println!("summer - {}", summer);
            return summer;
        }
    }
    let mut non_9s: u16 = 0;
    for i in 0..num_ints/2{
        //println!("i-{} ints-{}, ints-{}", i, ints[i as usize], ints[(num_ints/2 + i) as usize]);
        if ints[i as usize] != 9{
            ints[i as usize] = ints[i as usize] + 1;
            ints[(num_ints/2 + i) as usize] = ints[i as usize];
            non_9s = 1;
            break;
        }
        else {
            ints[i as usize] = 0;
            ints[(num_ints/2 + i) as usize] = 0;
            non_9s = 2;
        }
    }
    //println!("ints next2 {:?}", ints);

    if 2 == non_9s{
        let first_thing = 10_u64.pow(num_ints/2);
        let second_thing = 10_u64.pow(num_ints+1);
        //println!("returns - {}", first_thing + second_thing);
        return second_thing + first_thing;
    } 
    else {
        let mut summer = 0;
        for i in 0..num_ints{
            summer = summer + (10_u64.pow(i))*(ints[i as usize]);
        }
        //println!("summer {}", summer);
        return summer;
   }
}


fn main() -> io::Result<()>{
    let all_ids: String = fs::read_to_string("aoc2_inputs.txt")?;
    let parts: Vec<&str> = all_ids.split(',').collect();
    let mut fake_ids: Vec<u64> = Vec::<u64>::new();
    //println!("{:?}", parts);

    for ids in &parts{
        let start_fin: Vec<&str> = ids.split("-").collect();
        let start = start_fin[0 as usize].parse::<u64>().unwrap();
        let end = start_fin[1 as usize].parse::<u64>().unwrap();
        let mut next_fake_id;
        println!("start {} end {}", start, end);
        if is_fake_id(start){
            next_fake_id = start;
        }
        else {
            next_fake_id = find_next_fake_id(start);
        }
        
        while next_fake_id <= end{
            if next_fake_id >= start{
                fake_ids.push(next_fake_id);
            }
            next_fake_id = find_next_fake_id(next_fake_id);
        }
    }
    let mut sum: u64 = 0;
    for i in &fake_ids{
        println!("{}", i);
        sum = sum + i;
    }
    println!("{}", sum);
    Ok(())
}