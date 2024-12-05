use std::env;
use std::process;

mod solutions;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: cargo run <day>");
        process::exit(1);
    }
    
    // Parse the day from command-line argument
    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid day number");
            process::exit(1);
        }
    };
    
    // Dynamically call the solution for the specified day
    match day {
        1 => solutions::day1::solve(),
        2 => solutions::day2::solve(),
        // Add more days as you implement them
        _ => {
            eprintln!("Solution for day {} not implemented", day);
            process::exit(1);
        }
    }
}
