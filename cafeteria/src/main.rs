use std::io;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;
use std::fs::File;
use std::path::Path;
use intervaltree::IntervalTree;

fn main() {
    println!("Hi I am your cafeteria helper.");

    // NOTE: range values do overlap = _Interval Tree_
    // It sort of extends a binary tree search
    // contains a value, left, and right (ranges)
    // .query_point() to return ranges falling inside the given point. 

    let path = Path::new("foods.txt");
    let mut stream = FoodStream::open(path).unwrap();

    let mut sum: u64 = 0;
    //while let Some(food) = stream.next() {
    //    let food = food.unwrap().item;

    //    if stream.header.query_point(food).next().is_some() {
    //        sum += 1;
    //    }
    //}
    let mut ranges: Vec<Range<u64>> = stream.header.into_iter().map(|n| n.range).collect(); // NOTE:
    // need randowm access

    ranges.sort_by_key(|r| r.start);

    let mut total: u64 = 0;
    let mut cur: Option<Range<u64>> = None;
    for r in ranges {
        match cur.as_mut() {
            Some(c) if r.start <= c.end => c.end = c.end.max(r.end), // NOTE: entirely inside cur
            // range
            _ => {
                if let Some(c) = cur.take() { total += c.end - c.start; }
                cur = Some(r);
            }
        }
    }
    if let Some(c) = cur { total += c.end - c.start; } 

    println!("{total}");
//    for node in stream.header.iter() {
//        println!("{:?}", node);
//    }
}

#[derive(Debug)]
pub struct Food {
    item: u64
}

#[derive(Debug)]
pub struct FoodStream {
    reader: BufReader<File>,
    header: IntervalTree<u64, String>,
}

impl FoodStream {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut reader = BufReader::new(File::open(path)?);
        let header = Self::read_header(&mut reader).unwrap();
        Ok(Self { reader, header  }) // NOTE: move the reader into the struct, now at record position
    }

    fn read_header(reader: &mut BufReader<File>) -> io::Result<IntervalTree<u64, String>> {
        let mut line = String::new();
        let mut intervals: Vec<(Range<u64>, String)> = Vec::new();

        loop {
            line.clear();
            let n = reader.read_line(&mut line)?;
            if n == 0 { break; } // EOF before blank line
            let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
            if trimmed.is_empty() { break; } // header done
            
            let tuple = Self::parse_header_line(trimmed)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            intervals.push(tuple);
        }
        let tree:IntervalTree<u64, String> = intervals.into_iter().collect();
        return Ok(tree)
    }

    fn parse_header_line(range: &str) -> io::Result<(Range<u64>, String)> {
        let (start, stop) = range.split_once('-').unwrap();
        let start: u64 = start.parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData,
                format!("bad start {start:?}: {e}")))?;
        let stop: u64 = stop.parse().unwrap();
        return Ok((start..stop + 1, "fresh".to_string()))
    }

    fn parse_food(line: &str) -> io::Result<Food> {
        Ok(Food { item: line.trim().parse().unwrap() })
    }
}

// WARN: impl Trait for Type blocks can only contain items defined by the trai.
impl Iterator for FoodStream {
    type Item = io::Result<Food>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None, // EOF
            Ok(_) => Some(Self::parse_food(&line)),
            Err(e) => Some(Err(e)),
        }
    }
}
// // NOTE: A BTreeMap or segment-tree can be useful for fast look ups to a range of values
// pub struct FoodCollection {
//     ranges: [std::ops::Range<u64>]
//     foods: [u64],
// }
// 
// impl FoodCollection {
//     // WARN: Not setup to move through the reader
//     pub fn new() -> Self {
//         Self {
// 
//         }
//     }
// 
//     pub fn iter(&self) -> UserIterator {
//         FoodIterator {
//             
//         }
//     }
// }
// 
// pub struct FoodIterator<'a> {
//     food_collection: &'a FoodCollection,
// }
// 
// // NOTE: only need to override next for access to fold, map, for_each
// impl Iterator for FoodIterator<'_> {
//     type Item = &'static u64;
// 
//     fn next(&mut self) -> Option<Self::Item>;
// 
// 
// }
