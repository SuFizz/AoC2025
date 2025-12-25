use std::fs;
use std::io;
use std::collections::VecDeque;
use std::collections::HashSet;
use good_lp::{default_solver, variable, ProblemVariables, SolverModel, Expression};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Button{
    buttons: Vec<usize>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct ButtonList{
    actions: Vec<Button>,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Machine{
    indicator_lights: Vec<char>,
    joltages: Vec<u32>,
    num_jumps: u32,
}

fn find_least_steps(goal_state: &Machine, button_list: &ButtonList) -> u32 {
    let mut seen_states: HashSet<Vec<char>> = HashSet::new();
    let start_state: Machine = Machine{indicator_lights: vec!['.'; goal_state.indicator_lights.len()], joltages: vec![0,1], num_jumps : 0};
    let mut iterating_states: VecDeque<Machine> = VecDeque::new();
//    println!("goal - {:?}", goal_state);
//    println!("add - {:?}", start_state);
    iterating_states.push_back(start_state.clone());
    loop{
        if iterating_states.is_empty(){
            break;
        }
        let current_state = iterating_states.pop_front();
        for b in &button_list.actions{
            let mut intermediate_state: Machine = current_state.clone().expect("REASON");
            for i in &b.buttons{
                if intermediate_state.indicator_lights[*i] == '.'{
                    intermediate_state.indicator_lights[*i] = '#';
                }
                else{
                    intermediate_state.indicator_lights[*i] = '.';
                }
            }
            intermediate_state.num_jumps = intermediate_state.num_jumps + 1;
            if seen_states.contains(&intermediate_state.indicator_lights){
                continue;
            }
            seen_states.insert(intermediate_state.indicator_lights.clone());
            iterating_states.push_back(intermediate_state.clone());
//            println!("add - {:?}", intermediate_state);
            if intermediate_state.indicator_lights == *goal_state.indicator_lights {
                return intermediate_state.num_jumps;
            }
        }
    }
    return 0;
}

fn solve_integer_system(a: &Vec<Vec<f64>>, b: &Vec<f64>) -> Vec<i32> {
    let num_vars = a[0].len();
    let num_eqs = a.len();

    let mut vars = ProblemVariables::new();
    let x: Vec<_> = (0..num_vars)
        .map(|_| vars.add(variable().min(0).integer()))  // Non-negative integers
        .collect();

    // We don't have an objective, so minimize sum (or set to 0)
    let objective = Expression::from(0);

    // Add all equations as constraints
    let mut problem = vars.minimise(objective).using(default_solver);

    for i in 0..num_eqs {
        let mut constraint = Expression::from(0);
        for j in 0..num_vars {
            constraint += a[i][j] * x[j];
        }
        problem = problem.with(constraint.eq(b[i]));  // Exact equality
    }

    let solution = problem.solve().expect("No solution found");

    x.iter().map(|&var| solution.value(var) as i32).collect()
}

fn find_least_joltages(joltage_p: &Machine, button_list: &ButtonList) -> u32 {
    let mut seen_states: HashSet<Vec<u32>> = HashSet::new();
    let start_state: Machine = Machine{indicator_lights: vec!['.'; 1], joltages:vec![0; joltage_p.joltages.len()], num_jumps:0};
    let mut iterating_states: VecDeque<Machine> = VecDeque::new();
    //println!("goal - {:?}", joltage_p);
    //println!("add - {:?}", start_state);
    iterating_states.push_back(start_state.clone());
    loop{
        if iterating_states.is_empty(){
            break;
        }
        let current_state = iterating_states.pop_front();
        for b in &button_list.actions{
            let mut intermediate_state: Machine = current_state.clone().expect("REASON");
            for i in &b.buttons{
                intermediate_state.joltages[*i] = intermediate_state.joltages[*i] + 1;
            }
            intermediate_state.num_jumps = intermediate_state.num_jumps + 1;
            if seen_states.contains(&intermediate_state.joltages){
                continue;
            }
            let mut flag: bool = true;
            let mut all_match: bool = true;
            for i in 0..intermediate_state.joltages.len(){
                if joltage_p.joltages[i] < intermediate_state.joltages[i] {
                    flag = false;
                }
                if joltage_p.joltages[i] != intermediate_state.joltages[i] {
                    all_match = false;
                }
            }
            if !flag{
                continue;
            }
            seen_states.insert(intermediate_state.joltages.clone());
            iterating_states.push_back(intermediate_state.clone());
            //println!("add - {:?}", intermediate_state);
            if all_match{
                return intermediate_state.num_jumps;
            }
        }
    }
    return 0;

}

fn find_least_jltg(joltage_p: &Machine, button_list: &ButtonList) -> u32 {
    let mut x: Vec<Vec<f64>> = vec![[0; button_list.buttons.len()];joltage_p.joltages.len()];
    let mut j = 0;
    for b in &button_list.actions{
        for i in &b.buttons{
            x[i][j] = 1.0;
        }
        j = j + 1;
    }
    let y = joltage_p.joltages.iter().parse::<fp64>().collect();
    println!("{} {}", x, y);
    return 0;
}

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc10_inputs.txt")?;
    let full_lines: Vec<String> = contents.split("\r\n").filter(|line| !line.is_empty()).map(|line| line.to_string()).collect();
    let mut summer: u32 = 0;
    let mut i = 0;

    for l in &full_lines{
        let all_lists:Vec<_> = l.split(" ").collect();
        let machine = Machine{indicator_lights : all_lists[0].chars().filter(|&l| l=='.' || l=='#').collect(), joltages: vec![0, 1], num_jumps : 0};
        let mut all_actions: Vec<Button> = Vec::new();
        for j in 1..all_lists.len()-1{

            let action:Vec<_> = all_lists[j].split(",").map(|l| l.trim_matches(|c| c=='(' || c==')').parse::<usize>().unwrap()).collect();
            all_actions.push(Button{buttons:action});
        }
        let joltages: Vec<u32> = all_lists[all_lists.len()-1].split(',').map(|l| l.trim_matches(|c| c=='{' || c == '}').parse::<u32>().unwrap()).collect();
        let button_lists = ButtonList{actions:all_actions};
        let mach: Machine = Machine{indicator_lights:vec![',';1], joltages: joltages, num_jumps:0};
//        summer = summer + find_least_steps(&machine, &button_lists);
        summer = summer + find_least_jltg(&mach, &button_lists);
        i = i + 1;
        println!("iter-{}",i);
    }
    println!("{}", summer);
    Ok(())
}

