use std::process::exit;
use std::str::FromStr;
use clap::Parser;
use log::error;

use simple_logger::SimpleLogger;
use itertools::{Itertools, MinMaxResult};
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

fn calculate_safe_configs(configs: Vec<Vec<i64>>) -> u64 {
    let mut safe_configs: u64 = 0;
    for config in configs {

        let diffs = config.iter().tuple_windows().map(|(f, s)| f - s).collect::<Vec<_>>();
        let invalid_nos = diffs.iter().map(|i| i.abs()).filter(|i| !(1..=3).contains(i)).count();

        if invalid_nos > 0 {
            println!("❌ {:?} - Diffs {:?}; {} are not within bounds!", config, diffs, invalid_nos);
            continue;
        }

        match diffs.iter().minmax() {
            MinMaxResult::MinMax(min, max) => {
                if (min * max) > 0 {
                    println!("✓ {:?} - Diffs {:?};", config, diffs);
                    safe_configs += 1;
                } else {
                    println!("❌ {:?} - Diffs {:?}; Not continuous: Min: {}; Max: {}", config, diffs, min, max);
                }
                continue;
            },
            MinMaxResult::OneElement(_) | MinMaxResult::NoElements => {
                println!("❌ {:?} - Diffs {:?}; List does not contain correct number of elements!", config, diffs);
                continue;
            }
        }
    }
    safe_configs
}

fn parse_input(input_str: &str) -> Vec<Vec<i64>> {
    input_str
        .split("\n")
        .map(|line| line.split_whitespace().map(|e| i64::from_str(e).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>()
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
    let configs = parse_input(input_string.as_str());

    let safe_configs = calculate_safe_configs(configs);

    println!("Safe configs: {}", safe_configs);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_safe_configs, parse_input};

    #[test]
    fn test_example() {
        let input_string = include_str!("../../problems/problem2_test.txt");
        let configs = parse_input(input_string);

        let safe_configs = calculate_safe_configs(configs);
        assert_eq!(safe_configs, 2);
    }
}
