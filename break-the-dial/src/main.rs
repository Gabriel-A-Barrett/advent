use std::fs;

fn main() {
    let mut dial = Dial::new();
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to read file");

    for line in contents.lines() {
        dial.read_instruction(line);
    }

    println!("Password: {}", dial.password);
}

pub struct Dial {
    position: u32,
    password: u32,
}

impl Dial {
    pub const DIAL_UP: char = 'R';
    pub const DIAL_DOWN: char = 'L';

    pub fn new() -> Dial {
        Dial {
            position: 50,
            password: 0,
        }
    }

    pub fn read_instruction(&mut self, instruct: &str) {
        let instruct: (char, u32) = Self::process_instruct(instruct);

        if instruct.0 == Self::DIAL_UP {
            self.dial_up(instruct.1)
        } else if instruct.0 == Self::DIAL_DOWN {
            self.dial_down(instruct.1);
        } else {
            panic!("Unrecognized dial command: {}", instruct.0);
        }
        println!("Position: {}", self.position);
    }

    // Works for ASCII but not general Unicode
    fn process_instruct(instruct: &str) -> (char, u32) {
        let (instruct, spaces) = instruct.split_at(1);
        let instruct = instruct.chars().next().unwrap();
        let spaces: u32 = spaces.parse().unwrap();
        return (instruct, spaces);
    }

    fn dial_up(&mut self, inc: u32) {
        for _ in 0..inc {
            self.position += 1;
            if self.position == 100 {
                let _ = self.position = 0;
            }
            self.is_at_zero();
        }
    }

    fn dial_down(&mut self, dec: u32) {
        for _ in 0..dec {
            if self.position == 0 {
               self.position = 99;
            } else {
                self.position -= 1;
            }
            self.is_at_zero();
        }
    }

    fn is_at_zero(&mut self) {
        if self.position == 0 {
            self.password += 1;
        }
    }

    pub fn password(&self) -> u32 {
        self.password
    }
}


