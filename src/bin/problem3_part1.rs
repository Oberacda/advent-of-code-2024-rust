use std::ops::Mul;
use std::process::exit;
use clap::Parser;
use log::error;

use regex;

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

fn get_mult_pairs(input_string: &str) -> Vec<(i64, i64)> {
    let mut output = Vec::new();
    let regex = regex::Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    for capture in regex.captures_iter(input_string) {
        let first = capture.name("first").map(|m| m.as_str().parse::<i64>().unwrap()).unwrap();
        let second = capture.name("second").map(|m| m.as_str().parse::<i64>().unwrap()).unwrap();
        output.push((first, second));
    }
    output
}

fn multiply_pairs_and_add(pairs: Vec<(i64, i64)>) -> i64 {
    pairs.iter().map(|(f, s)| f.mul(s)).sum()
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
    let mult_pairs = get_mult_pairs(input_string.as_ref());
    let mult_result = multiply_pairs_and_add(mult_pairs);

    println!("{} -> {}", input_string, mult_result);
}

#[cfg(test)]
mod tests {
    use crate::{get_mult_pairs, multiply_pairs_and_add};

    #[test]
    fn test_example() {
        let input_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let mult_pairs = get_mult_pairs(input_string);
        let mult_result = multiply_pairs_and_add(mult_pairs);

        assert_eq!(mult_result, 161);
    }
}
