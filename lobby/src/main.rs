use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("battery_bank.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut max_joltage: u64= 0;

    for line_result in reader.lines() {
        let line = line_result?; // unwrap the result or return an Error
        // INFO: Initialize a vector containing number of batteries we can use for power n = 12
        let max_batteries = 12;

        let mut batteries: Vec<Battery> = Vec::new();
        let mut previous_battery_location: isize = -1;
        for count in (0..max_batteries).rev() {
            batteries.push(
                iterate_across_bank(count, &line, previous_battery_location)
            );
            previous_battery_location = batteries.last().unwrap().index;
        }
        resultant_joltage(batteries, &mut max_joltage);
        println!("{}", "=".repeat(30));
   }
    println!("Max Joltage: {}", max_joltage);
    Ok(())
}

fn iterate_across_bank(count: u8, bank: &String, previous: isize) -> Battery {
    let mut battery: Battery = Battery { index: 0, value: 0 };
    let mut iter = bank.chars().enumerate().peekable();
    while let Some((idx, potential_battery)) = iter.next() {
        let mut peek_iter = iter.clone();
        if count != 0 {
            let adj_count = count - 1;
            if peek_iter.nth(adj_count as usize) == None {
                return battery
            }
        }
        if let Some(power) = potential_battery.to_digit(10) {
            if power > battery.value && idx as isize > previous {
                battery = Battery { index: idx as isize, value: power};
            }
        }
    }
    return battery
}

fn resultant_joltage(batteries: Vec<Battery>, max_joltage: &mut u64) {
    let mut place: u64 = 1;
    for battery in batteries.into_iter().rev() {
        println!("{:?}", battery);
        let joltage = battery.value as u64 * place;
        println!("joltage: {joltage}");
        *max_joltage += joltage;
        place = place * 10;
    }
}

#[derive(Debug)]
struct Battery {
    value: u32,
    index: isize,
}


