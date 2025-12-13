use std::fs;
use std::io;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Coords{
    x: u64,
    y: u64,
    x_changed: usize,
    y_changed: usize,
    posn: usize,
}

impl Coords{
    fn dist(&self, other: &Coords) -> u64{
//        println!("{} {} {} {} {} {}", self.x, other.x, self.y, other.y, self.z, other.z);
        let x_dist: u64 = if self.x>other.x {self.x - other.x} else {other.x-self.x};
        let y_dist: u64 = if self.y>other.y {self.y - other.y} else {other.y-self.y};
        return (x_dist+1)*(1+y_dist);
    }
}

#[derive(Debug, Clone)]
struct Distance<'a>{
    coord1: &'a Coords,
    coord2: &'a Coords,
    distance: u64,
}

impl Distance<'_>{
    fn new<'a>(coord_1: &'a Coords, coord_2: &'a Coords) -> Distance<'a>{
        let dista = coord_1.dist(&coord_2);
        return Distance{coord1: coord_1, coord2: coord_2, distance:dista};
    }
}

fn inside_surface(distance: &Distance, tiling: &Vec<Vec<char>>) -> bool{
    let coord1 = distance.coord1;
    let coord2 = distance.coord2;
    let x_min = coord1.x_changed.min(coord2.x_changed);
    let x_max = coord1.x_changed.max(coord2.x_changed);
    let y_min = coord1.y_changed.min(coord2.y_changed);
    let y_max = coord1.y_changed.max(coord2.y_changed);

    for i in y_min..=y_max{
        for j in x_min..=x_max{
            if tiling[i][j] == ','{
                return false;
            }
        }
    }
    return true;
}

fn make_tiling(all_coords: &Vec<Coords>) -> Vec<Vec<char>>{
    // The tiling idea came from a reddit thread which references coordinate compression.
    // The key insight was the comment saying that we could reduce it to less than 500.
    // Although I would think I made this algo on my own. I owe it to that insight.
    let mut tiling: Vec<Vec<char>> = vec![vec![',';300]; 300];
    //println!("{:?}", tiling);

    let mut i = 0;
    loop{
        tiling[all_coords[i].y_changed][all_coords[i].x_changed] = 'R';
        let mut next_i = i+1;
        if next_i == all_coords.len(){
            next_i = 0;
        }
        // println!("tiling Red {} {} with next coords {} {}", all_coords[i].y_changed, all_coords[i].x_changed, all_coords[next_i].y_changed, all_coords[next_i].x_changed);
        if all_coords[next_i].y_changed != all_coords[i].y_changed{
            let min_y = all_coords[i].y_changed.min(all_coords[next_i].y_changed);
            let max_y = all_coords[i].y_changed.max(all_coords[next_i].y_changed);
            // println!("y - {} {}", min_y, max_y);
            for j in min_y+1..max_y{
                tiling[j][all_coords[i].x_changed] = 'G';
                // println!("{} {}", j, all_coords[i].x_changed);
            }
            //for qu in 0..5{
            //    println!("{:?}", tiling[qu]);
            //}
        }
        else if all_coords[next_i].x_changed != all_coords[i].x_changed{
            let min_x = all_coords[i].x_changed.min(all_coords[next_i].x_changed);
            let max_x = all_coords[i].x_changed.max(all_coords[next_i].x_changed);
            //println!("x - {} {}", min_x, max_x);
            for j in min_x+1..max_x{
                tiling[all_coords[i].y_changed][j] = 'G';
                //println!("{} {}", all_coords[i].y_changed, j);
            }
            //for qu in 0..5{
            //    println!("{:?}", tiling[qu]);
            //}
        }
        i = i+1;
        if i == all_coords.len(){
            break;
        }
    }
    for i in 0..tiling.len(){
        let mut set_flag = false;
        for j in 0..tiling[0].len(){
            if tiling[i][j] != ',' && tiling[i][j+1] ==','{
                set_flag = !set_flag;
            }
            else if set_flag{
                tiling[i][j] = 'G';
            }
        }
    }
    //for i in 0..5{
    //    println!("{:?}", tiling[i]);
    //}
    return tiling;
}

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc9_inputs.txt")?;
    let locations: Vec<Vec<u64>> = contents.split("\r\n").filter(|line| !line.is_empty()).map(|line| line.split(',').map(|b|  b.parse::<u64>().unwrap()).collect()).collect();

    let mut all_coords: Vec<Coords> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();
    for i in 0..locations.len(){
        all_coords.push(Coords{x:locations[i][0], y:locations[i][1], x_changed: i, y_changed: i, posn: i});
    }

    all_coords.sort_by_key(|e| e.x);
    all_coords[0].x_changed = 0;
    let mut k: usize = 1;
    for i in 1..locations.len(){
        if all_coords[i].x==all_coords[i-1].x {
            all_coords[i].x_changed = all_coords[i-1].x_changed;
            //println!("i-1 : {:?}; i : {:?}", all_coords[i-1], all_coords[i]);
        }
        else {
            all_coords[i].x_changed = k;
            k = k+1;
            //println!("incr k i-1 : {:?}; i : {:?}", all_coords[i-1], all_coords[i]);
        }
    }
    //println!("{}", k);

    all_coords.sort_by_key(|e| e.y);
    all_coords[0].y_changed = 0;
    let mut k: usize = 1;
    for i in 1..locations.len(){
        if all_coords[i].y==all_coords[i-1].y {
            all_coords[i].y_changed = all_coords[i-1].y_changed;
            //println!("i-1 : {:?}; i : {:?}", all_coords[i-1], all_coords[i]);
        }
        else {
            all_coords[i].y_changed = k;
            k = k+1;
            //println!("incr k i-1 : {:?}; i : {:?}", all_coords[i-1], all_coords[i]);
        }
    }
    //println!("{}", k);

    all_coords.sort_by_key(|e| e.posn);
    for i in 0..locations.len(){
        for j in i+1..locations.len(){
            distances.push(Distance::new(&all_coords[i], &all_coords[j]));
        }
    }

    distances.sort_by_key(|s| {s.distance});
    println!("len of dist - {}", distances.len());
    //for i in 0..distances.len(){
    //    println!("{:?}",distances[i]);
    //}

//    println!("{}", distances[distances.len()-1].distance);

    let tiling = make_tiling(&all_coords);
    for i in (0..distances.len()).rev(){
        if inside_surface(&distances[i], &tiling){
            println!("{:?}", distances[i]);
            break;
        }
    }
    // println! ("{} {:?}", distances.len(), distances);
    Ok(())
}

