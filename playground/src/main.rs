use std::fs;

fn main() {
    println!("Calculating shortest distance to junction boxes");

    let contents = fs::read_to_string("test.txt").unwrap();
    
    let junctions: Vec<Junction> = contents.trim().lines()
        .map ( |s| Junction::from_input(s))
        .collect();

    let mut pairs: Vec<ConnectionPairs> = Vec::new();
    for (i, j1) in junctions.iter().enumerate() {
        let (_, remaining) = junctions.split_at(i + 1);
        for j2 in remaining {
            let euc = Junction::euclidean_distance(&j1, &j2);
            pairs.push(ConnectionPairs { j1: *j1, j2: *j2, euc: euc})
        }
    }

    //NOTE: SORTING shortest to longest
    pairs.sort_by(|a,b| a.euc.partial_cmp(&b.euc).unwrap() );

    let mut connections: Vec<Vec<Junction>> = Vec::new();     
    let max_connections = 10;
    let mut con = 1;

    let mut parent: Vec<usize> = (0..junctions.len()).collect();

    //let total = connections[0].len() * connections[1].len() * connections[2].len();

    //println!("{total}");
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x { parent[x] = find(parent, parent[x]); } // path compression
    parent[x]
}

#[derive(Debug)]
struct ConnectionPairs {
    j1: Junction,
    j2: Junction,
    euc: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Junction {
    x: f32,
    y: f32,
    z: f32,
}

impl Junction {

    // Constructor
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn from_input(line: &str) -> Self {
        let split: Vec<f32> = line.split(',').map(|j| j.parse().unwrap() ).collect();
        Self::new(split[0], split[1], split[2])
    }

    fn euclidean_distance(j1: &Junction, j2: &Junction) -> f32 {
        ((j1.x - j2.x).powi(2) + (j1.y - j2.y).powi(2) + (j1.z - j2.z).powi(2)).sqrt()
    }
}
