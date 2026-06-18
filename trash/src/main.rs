use std::fs;
use std::mem::take; // NOTE: moves the Vec and leaves and empty one in its place

fn main() -> Result<(), std::io::Error> {
    println!("Cephalopad Homework assignment");
    let contents = fs::read_to_string("hw.txt")?;
    let mut nested_eggs: Vec<Egg> = Vec::new();
    // WARN: into_iter() consumes the data and takes ownership
    // NOTE: .rev().enumerate() order starts the enumerator at 0 instead of the max value
    let iter = contents.lines().collect::<Vec<&str>>().into_iter().rev().enumerate();
    
    for (row, line) in iter {
        let mut start: bool = true;
        //let numbers = line.trim().split(" ");
        let chars: Vec<(usize, char)> = line.char_indices().collect();
        let mut column_idx: usize = 0;
        let mut inner_idx: usize = 0;
        let mut row_num: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            let (idx, ch) = chars[i];
            if row == 0 {
                if ['*','+'].contains(&ch) {
                    let arithmetic: Arithmetic = Arithmetic::get(&ch).unwrap();
                    let egg = Egg { eggs: Vec::new(), arithmetic: arithmetic , start: idx};
                    nested_eggs.push(egg);    
                }
            } else {
                // NOTE: if there is another arithmetic we might skip a space coming before nxt
                if let Some(nxt_egg) = nested_eggs.get(column_idx + 1) {
                    let nxt_col_pos = nxt_egg.start - 1;
                    if ch == ' ' && idx == nxt_col_pos { inner_idx=0; column_idx+=1; continue }
                }

                if ch != ' ' {

                    // NOTE: BORROW MAGIC!!! REVIEW CLOSELY
                    let eggs_vec = &mut nested_eggs[column_idx];

                    if inner_idx >= eggs_vec.eggs.len() {
                        // NOTE: ex: .resize(5, 0) = [0, 0, 0, 0, 0] or if elements exist will
                        // append 0
                        eggs_vec.eggs.resize(inner_idx + 1, Vec::new());
                    }
                    let col = &mut eggs_vec.eggs[inner_idx];
                    col.push(ch);
                }

                inner_idx+=1;

                // NOTE: 2 logics for ' ' chars

                // if space but comes before the next start continue
            }
        }
    }

    let mut total = 0;
    for egg in nested_eggs {
        println!("{:?}", egg);
        let mut left = 0;
        for col in egg.eggs {

            let mut parse = String::from("");
            for ch in col.into_iter().rev() {
                parse.push(ch);
                
            }
            let parse: u64 = parse.parse().unwrap();
            if left == 0 {
                left = parse
            } else {
                left = Arithmetic::perform(&egg.arithmetic, &left, &parse);
            }
        }
        total += left;
    }
    
    println!("{total}");

        //for (col, number) in numbers.into_iter().enumerate() {
        //    if row != 0 {
        //        let char_vec: Vec<char> = number.chars().collect(); 
        //        let length = char_vec.len() as usize;
        //        if length > nested_eggs[col].max_length {
        //            nested_eggs[col].max_length = length;
        //        }

        //        nested_eggs[col].eggs.push(char_vec); // WARN: moves char_vec so no longer can
        //   } else {
        //        if number.is_empty() { continue }
        //        let arithmetic: Arithmetic = Arithmetic::get(number).unwrap();
        //        let egg = Egg { eggs: Vec::new(), max_length: 0, arithmetic: arithmetic };
        //        nested_eggs.push(egg);
        //    }
        //}
    

    // NOTE: first pass to create paddings
    // NOTE: iter_mut allows you to modify the collection
    // let padding = '@';
    // let mut toggle = true;
    // for egg_struct in nested_eggs.iter_mut() {
    //     for egg_vec in egg_struct.eggs.iter_mut() {
    //         let num_length = egg_vec.len();
    //         // WARN: using a method get_max_length() would grant ownership
    //         if num_length == egg_struct.max_length { continue }

    //         let padding_length = egg_struct.max_length - num_length;
    //         // NOTE: starts padding to the left
    //         if toggle {
    //             for _ in 0..padding_length {
    //                 egg_vec.insert(0, padding);
    //             }
    //         } else { // NOTE: right padding
    //             for _ in (0..padding_length).rev() {
    //                 egg_vec.push(padding);
    //             }
    //         }
    //     }
    //     toggle = !toggle; // NOTE: flops between true and false
    // }

    //for egg in nested_eggs.clone() {
    //    println!("{:?}", egg);
    //}

    //let mut total: u64 = 0;
    //for egg in nested_eggs.clone().into_iter().rev() {
    //    let mut col_total: Option<u64> = None;
    //    for char in (0..egg.max_length).rev() {
    //        let mut complete_num = String::from("");
    //        for num_vec in egg.eggs.clone().into_iter().rev() {
    //            if char < num_vec.len() {
    //                let num = num_vec[char];
    //                if num != '@' {
    //                    complete_num.push(num);   
    //                }
    //            }
    //        }
    //        let n: u64 = complete_num.parse().unwrap();

    //        println!("{n}");
    //        col_total = Some(match col_total {
    //            None => n,
    //            Some(acc) => Arithmetic::perform(&egg.arithmetic, &acc, &n),
    //        });
    //    }
    //    println!("{:?}",col_total);
    //    total += col_total.unwrap();
    //}

    //println!("{:?}", total);
   // let total: u64 = nested_eggs.iter().sum();
   // 
   // println!("{total}");

    Ok(())
}

#[derive(Debug, Clone)]
struct Egg {
    // NOTE: String takes ownership of the str
    eggs: Vec<Vec<char>>, 
    // NOTE: &str is a fat pointer: borrows string slice. 2 usize therefore size is known at
    // compile time
    arithmetic: Arithmetic,
    start: usize,
}

#[derive(Debug, Clone)]
enum Arithmetic {
    Multiplication,
    Addition,
}

impl Arithmetic {

    fn get(s: &char) -> Result<Arithmetic, std::io::Error> {
        match s {
            '*' => Ok(Arithmetic::Multiplication),
            '+' => Ok(Arithmetic::Addition),
            _ => panic!("Arithmetic type not supported"),
        }
    }

    fn perform(op: &Arithmetic, left: &u64, right: &u64) -> u64 {
        let result = match op {
            Arithmetic::Multiplication => left * right,
            Arithmetic::Addition => left + right,
        };
        return result
    }
}


