use std::fs;
use std::io;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Coords{
    x: u64,
    y: u64,
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

fn main() -> io::Result<()>{
    let contents: String = fs::read_to_string("aoc9_inputs.txt")?;
    let locations: Vec<Vec<u64>> = contents.split("\r\n").filter(|line| !line.is_empty()).map(|line| line.split(',').map(|b|  b.parse::<u64>().unwrap()).collect()).collect();

    let mut all_coords: Vec<Coords> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();

    for i in 0..locations.len(){
        all_coords.push(Coords{x:locations[i][0], y:locations[i][1]});
    }

    for i in 0..locations.len(){
        for j in i+1..locations.len(){
            distances.push(Distance::new(&all_coords[i], &all_coords[j]));
        }
    }

    distances.sort_by_key(|s| {s.distance});
    println!("len of dist - {}", distances.len());
    for i in 0..distances.len(){
        println!("{:?}",distances[i]);
    }

    println!("{}", distances[distances.len()-1].distance);

    // println! ("{} {:?}", distances.len(), distances);
    Ok(())
}

