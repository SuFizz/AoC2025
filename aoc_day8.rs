use std::fs;
use std::io;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Coords{
    x: u64,
    y: u64,
    z: u64,
}

impl Coords{
    fn dist(&self, other: &Coords) -> u64{
//        println!("{} {} {} {} {} {}", self.x, other.x, self.y, other.y, self.z, other.z);
        let x_dist: u64 = if self.x>other.x {self.x - other.x} else {other.x-self.x};
        let y_dist: u64 = if self.y>other.y {self.y - other.y} else {other.y-self.y};
        let z_dist: u64 = if self.z>other.z {self.z - other.z} else {other.z-self.z};
        return (x_dist*x_dist + y_dist*y_dist + z_dist*z_dist).isqrt();
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
    let contents: String = fs::read_to_string("aoc8_inputs.txt")?;
    let locations: Vec<Vec<u64>> = contents.split("\r\n").filter(|line| !line.is_empty()).map(|line| line.split(',').map(|b|  b.parse::<u64>().unwrap()).collect()).collect();

    let mut all_coords: Vec<Coords> = Vec::new();
    let mut distances: Vec<Distance> = Vec::new();

    for i in 0..locations.len(){
        all_coords.push(Coords{x:locations[i][0], y:locations[i][1], z:locations[i][2]});
    }

    for i in 0..locations.len(){
        for j in i+1..locations.len(){
            distances.push(Distance::new(&all_coords[i], &all_coords[j]));
        }
    }

    distances.sort_by_key(|s| {s.distance});
    // println!("len of dist - {}", distances.len());
    //for i in 0..1001{
    //    // println!("{:?}",distances[i]);
    //}

    let mut circuits: Vec<HashSet<Coords>> = Vec::new();
    let mut circuits0: HashSet<Coords> = HashSet::new();

    circuits0.insert(distances[0].coord1.clone());
    circuits0.insert(distances[0].coord2.clone());
    circuits.push(circuits0);

    // println!("first circuits - {:?}", circuits);
    for i in 1..distances.len(){
        //println!("i-{} {:?} {:?}", i, distances[i].coord1, distances[i].coord2);
        let mut mergers = Vec::new();
        for j in 0..circuits.len(){
            let cont_coord1 = circuits[j].contains(distances[i].coord1);
            let cont_coord2 = circuits[j].contains(distances[i].coord2);
            if cont_coord1 && !cont_coord2{
                circuits[j].insert(distances[i].coord2.clone());
                mergers.push(j);
                //println!("j-{} contains coord1 {:?} adding coord2 {:?}", j, distances[i].coord1, distances[i].coord2);
            }
            else if !cont_coord1 && cont_coord2{
                circuits[j].insert(distances[i].coord1.clone());
                mergers.push(j);
                //println!("j-{} contains coord2 {:?} adding coord1 {:?}", j, distances[i].coord2, distances[i].coord1);
            }
            else if cont_coord1 && cont_coord2{
                mergers.push(j);
                //println!("j-{} contains both coord2 {:?} and coord1 {:?}", j, distances[i].coord2, distances[i].coord1);
            }
        }
        let mut next_circuits = circuits.clone();
        //println!("{:?}", mergers);
        for mrg in (1..mergers.len()).rev(){
            next_circuits[mergers[0]].extend(circuits[mergers[mrg]].clone());
            next_circuits.remove(mergers[mrg]);
            //println!("extending i-{} {} {}", i, mergers[0], mergers[mrg]);
        }
        circuits = next_circuits;
        if mergers.is_empty(){
            println!("adding coord1 {:?} adding coord2 {:?}", distances[i].coord1, distances[i].coord2); // last added one answers part 2
            let mut circuits0: HashSet<Coords> = HashSet::new();
            circuits0.insert(distances[i].coord1.clone());
            circuits0.insert(distances[i].coord2.clone());
            circuits.push(circuits0);
        }
    }

    // println!("full circuits - {} {:?}", circuits.len(), circuits);

    circuits.sort_by_key(|mayp| {mayp.len()});
    circuits.reverse();
    println!("final circuits {:?}", circuits[0].len()*circuits[1].len()*circuits[2].len());

    // println! ("{} {:?}", distances.len(), distances);
    Ok(())
}
