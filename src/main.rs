use std::process;
use std::env;
use adventofcode2025::{puzzles};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config);
}

fn run(cfg: Config) {
    match cfg.day {
        1 => puzzles::day1::solution(&cfg.input_path),
        2 => puzzles::day2::solution(&cfg.input_path, None),
        3 => puzzles::day3::solution(&cfg.input_path),
        4 => puzzles::day4::solution(&cfg.input_path),
        5 => puzzles::day5::solution(&cfg.input_path),
        6 => puzzles::day6::solution(&cfg.input_path),
        _ => println!("Oops not implemented yet")
    }
}

#[derive(Debug)]
struct Config {
    day: u8,
    input_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args, please specify the day you want to solve")
        }
        
        let day: u8;
        match args[1].as_str() {
            "1" | "day1" => day = 1,
            "2" | "day2" => day = 2,
            "3" | "day3" => day = 3,
            "4" | "day4" => day = 4,
            "5" | "day5" => day = 5,
            "6" | "day6" => day = 6,
            "7" | "day7" => day = 7,
            "8" | "day8" => day = 8,
            "9" | "day9" => day = 9,
            "10" | "day10" => day = 10,
            "11" | "day11" => day = 11,
            "12" | "day12" => day = 12,
            _ => return Err("Oop not a good day")
        }

        let mut input_path = String::from(format!("./inputs/day{day}/"));
        if let Some(command) = args.get(2) {
            if command == "sample" {
                input_path.push_str("sample_input.txt");
            }
        } else {
            input_path.push_str("input.txt");
        };

        Ok(Config { day, input_path })
    }
}