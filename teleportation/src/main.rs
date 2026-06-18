use std::fs;
use std::collections::HashMap;

fn main() {
    println!("Activating Teleportor");

    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines();
    
    // NOTE: Row 0 build counts from S
    let first = lines.next().unwrap();
    let mut counts: Vec<u64> = first
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    let mut total: u64 = 0;

    for line in lines {
        let snapshot = counts.clone();
        for (c, ch) in line.char_indices() {
            if ch != '^' { continue; }
            let n = snapshot[c];
            if n == 0 { continue; }
            total += n;
            counts[c] -= n;
            if c > 0                { counts[c - 1] += n; }
            if c + 1 < counts.len() { counts[c + 1] += n; }
        }
    }

//    for (idx, line) in contents.lines().into_iter().enumerate() {
//        for (ch_idx, ch) in line.char_indices() {
//            if idx == 0 {
//                if previous.get(0).is_none() { previous.push(Vec::new()); }
//                if ch == 'S' {
//                    previous[0].push(true);
//                } else {
//                    previous[0].push(false);
//                }
//            } else {
//                if ch == '^' {
//                    // NOTE: use a while loop to enable removing elements within
//                    let mut t = 0;
//                    let mut to_add: Vec<Vec<bool>> = Vec::new();
//                    while t < previous.len() {
//                        let prev = &previous[t];
//                        if prev[ch_idx] {
//                            let mut prev = prev.clone();
//                            total += 1;
//                            prev[ch_idx] = false;
//                            let mut l_p = prev.clone();
//                            l_p[ch_idx - 1] = true;
//                            let mut r_p = prev.clone();
//                            r_p[ch_idx + 1] = true;
//                            to_add.push(l_p);
//                            to_add.push(r_p);
//                            previous.swap_remove(t); // remove that timeline 
//                            // WARN: do NOT increment i here; the next element shifted
//                        } else {
//                            t += 1;
//                        }
//                    }
//                    previous.extend(to_add);
//                }
//            }
//        }
//    }
    println!("{total}");
}
