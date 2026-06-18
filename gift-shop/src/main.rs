use std::fs;

fn main() {
    let mut sum_of_invalid_ids: u64 = 0;

    let id_ranges = fs::read_to_string("id-ranges.txt")
        .expect("Unable to read input.txt");

    // WARN: remove \n characters from text
    let id_ranges = id_ranges.trim();
    let ids = process_input_file(id_ranges);
    println!("Number of ids found: {}", ids.len());
    for id in &ids {
        iterate_across_id(id, &mut sum_of_invalid_ids);
    }
    println!("Sum of invalid ids: {}", sum_of_invalid_ids);
    // look for matches of same sequence twice
    // numbers with leading zeros are not considered
}

/*
*   TODO: sliding window (n) char = (n) char
*   string slicing &s[0..2]
*/
fn iterate_across_id(id: &str, sum: &mut u64) {
    let length = id.len();
    let half_length = length / 2;
    let mut found_match = false;
    'a: for window in 1..=half_length {
        if length % window != 0 { continue; }
        let mut prev: Option<&str> = None;
        let mut start = 0;
        while start + window <= length {
            let slice = &id[start..start + window];
            if let Some(p) = prev {
                if slice != p { break }
            }
            prev = Some(slice);
            start += window;
        }
        if start == length {
            println!("Made it this far: {id}");
            
            found_match = true;
        }
    }
    if found_match == true {
        let id: u64 = id.parse().unwrap();
        *sum += id;
    }
    found_match = false;
}

fn mid_point_check(id: &str, mid_point: usize, mut sum: u64) {
    let (before, after) = id.split_at(mid_point);
    if before == after {
        let id: u64 = id.parse().unwrap();
        sum += id;
    }
}

fn process_input_file(line: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for range in iterate_across_commas(&line) {
        ids.append(&mut unravel_ids(range));
    }
    return ids
}

fn iterate_across_commas(line: &str) -> impl Iterator<Item = &str> {
    line.split(',')
}

// NOTE: str cannot be known at compile time must use String
fn unravel_ids(range: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let (start, stop) = range.split_once('-').unwrap();
    let start: u64 = start.parse().unwrap();
    let stop: u64 = stop.parse().unwrap();
    for id in start..=stop {
        ids.push(id.to_string());
    }
    return ids;
}

// fn decompose_id_into_vector(num: &u64) {
//     let id_as_vector: Vec<u8> = Vec::new();
//     while num > 0 {
//         let digit: u8 = num % 10;
//         num /= 10;
//         id_as_vector.push(digit);
//     }
// }

fn scan_fake_id(id: &u64) {
    // NOTE: take the length
    //
    // NOTE: look for repeats (1/) lenght of 6 look for matches in the middle 3 = 3
}
