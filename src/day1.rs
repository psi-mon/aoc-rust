use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve() -> Result<String, String> {
    let mut total = 0;
    let mut line_num = 1;
    if let Ok(lines) = read_lines("../data/d1") {
        for line in lines {
            if let Ok(num_str) = line {
                match num_str.trim().parse::<i32>() {
                    Ok(num) => total += num,
                    Err(_) => eprintln!("Invalid number on line {}", line_num),
                }
                line_num += 1;
            }
        }
    }
    Ok(total.to_string())
}
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
      let file = File::open(filename)?;
      Ok(io::BufReader::new(file).lines())
    }
