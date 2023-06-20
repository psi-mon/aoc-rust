use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn solve(file_path:String) -> Result<String, String> {
    let mut total = 0;
    let mut line_num = 1;
    let mut caloriens = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(num_str) = line {
                match num_str.trim().parse::<i32>() {
                    Ok(num) => total += num,
                    Err(_) => {
                        //eprintln!("Invalid number on line {}", line_num);
                        caloriens.push(total);
                        total = 0;
                 }
                }
                line_num += 1;
            }
        }

        if total != 0 {
            caloriens.push(total);
        }

        caloriens.sort();

        if let Some(calorie) = caloriens.last() {
            Ok(calorie.to_string())
        } else {
            Err("No calories found".to_string())
        }

        //Ok(caloriens.to_string())
    } else {
        Err("Error reading lines from file".to_string())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let path = filename.as_ref();
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {:?}", path),
        ));
    }

    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
