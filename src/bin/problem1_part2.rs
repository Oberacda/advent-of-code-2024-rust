use std::collections::HashMap;
use std::process::exit;
use std::str::FromStr;
use clap::Parser;
use log::error;

use regex::Regex;
use simple_logger::SimpleLogger;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
use advent_of_code_2024::parse_input_file;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = Args::parse();
    
    let input_string = match parse_input_file(args.input_file) {
        Ok(input_string) => input_string,
        Err(err) => {
            error!("Failed to parse file: {}", err);
            exit(-1);
        }
    };

    let re = match Regex::new(r"(?<first>[0-9]+)\s+(?<second>[0-9]+)") {
        Ok(regex) => regex,
        Err(err) => {
            error!("Could not compile regex: {}", err);
            exit(-4);
        }
    };


    let mut list_left = Vec::<u64>::new();
    let mut right = HashMap::<u64, u64>::new();

    let pairs: Vec<(u64, u64)> = re.captures_iter(input_string.as_str()).map(|caps| {
        let (_, [first, second]) = caps.extract();
        let first_number = match u64::from_str(first) {
            Ok(number) => number,
            Err(err) => {
                error!("Could not parse first number: {}", err);
                exit(-5);
            }
        };
        let second_number = match u64::from_str(second) {
            Ok(number) => number,
            Err(err) => {
                error!("Could not parse second number: {}", err);
                exit(-6);
            }
        };
        (first_number, second_number)
    }).collect();

    for (first_number, second_number) in pairs {
        list_left.push(first_number);

        if right.contains_key(&second_number) {
            right.insert(second_number, right[&second_number] + 1);
        } else {
            right.insert(second_number, 1);
        }
    }

    let mut result: u64 = 0;

    for left in list_left {
        result += left * right.get(&left).unwrap_or(&0u64);
    }

    println!("Difference: {}", result);
    exit(0);
}
