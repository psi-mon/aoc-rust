use clap::Parser;


#[derive(Parser,Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[arg(short,long, value_parser = validate_day)]
    day: u8
}

fn validate_day(value: &str) -> Result<u8, String>{
    let day = value.parse::<u8>().map_err(|_| "Invalid day")?;
    if day >= 1 && day <= 24{
        Ok(day)
    } else {
        Err("Day myusaeeff".to_string())
    }
}
fn main() {
    let args = Args::parse();

    println!("Hello, world!{:?}", args);
}
