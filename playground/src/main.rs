use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Calculating shortest distance to junction boxes");

    let contents = fs::read_to_string("input.txt").unwrap();
    
    let junctions: Vec<Junction> = contents.trim().lines()
        .map ( |s| Junction::from_input(s))
        .collect();

    let mut parent: Vec<usize> = (0..junctions.len()).collect();
    let mut edges: Vec<Edge> = Vec::new();
    for (i, j1) in junctions.iter().enumerate() {
        for (j, j2) in junctions.iter().enumerate().skip(i + 1) {
            edges.push(Edge {
                j1_idx: i,
                j2_idx: j,
                euc: Junction::euclidean_distance(j1, j2),
            });
        }
    }

    //NOTE: SORTING shortest to longest based on euclidean
    edges.sort_by(|a,b| a.euc.partial_cmp(&b.euc).unwrap() );
    let n = junctions.len();
    let mut accepted = 0;
    let mut bridging: Option<&Edge> = None;
    for edge in &edges { // NOTE: add a .iter().take(1000) to limit the number of connections
        let r1 = find(&mut parent, edge.j1_idx);
        let r2 = find (&mut parent, edge.j2_idx);
        if r1 == r2 { continue; }
        parent[r1] = r2;
        accepted += 1;
        
        if accepted == n - 1 {
            bridging = Some(edge);
            break;
        }
    }

    let e = bridging.expect("graph never fully connected");
    let x1 = junctions[e.j1_idx].x as i64;
    let x2 = junctions[e.j2_idx].x as i64;

    println!("{}", x1 * x2);


    //let mut sizes: HashMap<usize, usize> = HashMap::new();
    //for i in 0..junctions.len() {
    //    let root = find(&mut parent, i);
    //    *sizes.entry(root).or_insert(0) += 1;
    //}

    //let mut counts: Vec<usize> = sizes.into_values().collect();
    //counts.sort_unstable_by(|a, b| b.cmp(a)); // descending
    //
    //let mut total: usize = counts.iter().take(3).product();
    //println!("{total}");


    //let total = connections[0].len() * connections[1].len() * connections[2].len();

    //println!("{total}");
}

fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x { parent[x] = find(parent, parent[x]); } // path compression
    parent[x]
}

#[derive(Debug)]
struct Edge { // NOTE: Based on Kruskal-path MST
    j1_idx: usize,
    j2_idx: usize,
    euc: f32,
}

#[derive(Debug)]
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
