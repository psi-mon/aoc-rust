use clap::Parser;
use crate::day1::solution::solve;
pub mod day1;

const MAX_DAYS:u8 = 1;


#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short,long, value_parser = validate_day)]
    day: u8
}

fn validate_day(value: &str) -> Result<u8, String>{
    let day = value.parse::<u8>().map_err(|_| "Invalid day")?;
    if day >= 1 && day <= MAX_DAYS{
        Ok(day)
    } else {
        let error_msg = format!("Day must be between 1 and {} but was {}", MAX_DAYS, day);
        Err(error_msg)
    }
}
fn main() {
    let args = Args::parse();

    let day = args.day;

    let solution = match day {
        1 => solve("../../data/d1.txt".to_string()),
        _ => Err(format!("Invalid day: {}", day)),
    };

    match solution {
        Ok(result) => println!("Solution: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    } 


    println!("Hello, world!{:?}", args);
}

