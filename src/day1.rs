use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

    pub fn solve() -> Result<String, String>{

 if let Ok(lines) = read_lines(".data/d1") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);

            }
        }
    }

        Ok("Day 1 solution".to_string())
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
      let file = File::open(filename)?;
      Ok(io::BufReader::new(file).lines())
    }
