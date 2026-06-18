use std::fs;
use std::ops::Range;

const PAPER_ROLL: char = '@';
const ADJACENT_POSITIONS: usize = 8;

fn main() {

    let file = "toilet-paper-map.txt";
    let map = fs::read_to_string(file)
        .expect("ERROR MESSAGE");

    let length = map.lines().count();
    let width = map.lines().next().unwrap_or("").chars().count();

    let mut matrix = flatten_matrix(map);

    let mut fc = 0;
    let mut has_removed = true;
    while has_removed {
        has_removed = false;
        for r in 0..length {
            for c in 0..width {
                if !matrix[r * width + c] { continue; }
                let count = count_number_of_adjacent(&matrix, r, c, length, width);
                if count < 4 {
                    fc += 1;
                    has_removed = true;
                    // flattened array index equation idx = (r x W) + c
                    let idx = (r * width) + c;
                    matrix[idx] = false;
                }
            }
        }

    }



    println!("{}", fc);
}


// INFO: &[bool] is better than Vec because it accepts both Vectors and arrays in function
// signatures
fn count_number_of_adjacent(matrix: &[bool], r: usize, c: usize, length: usize, width: usize) -> u32 {

    let mut count = 0;
    for dr in -1..=1i32 {
        for dc in -1..=1i32 {
            if dr == 0 && dc == 0 { continue; } // skip the center
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr < 0 || nc < 0 || nr >= length as i32 || nc >= width as i32 {
                continue;
            }
            let idx = nr as usize * width + nc as usize;
            if matrix[idx] {
                count += 1;
            }
        }
    }
    return count;
}

fn is_toilet_paper(item: char) -> bool {
    if item == PAPER_ROLL {
        return true;
    }
    return false;
}

fn flatten_matrix(map: String) -> Vec<bool> {
    let mut matrix: Vec<bool> = Vec::new();

    for row in map.trim().lines() {
        for zone in row.chars() {
            matrix.push(zone == PAPER_ROLL)
       }
    }
    return matrix
}
