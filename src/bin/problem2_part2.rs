use clap::Parser;
use log::error;
use std::process::exit;
use std::str::FromStr;

use itertools::{Itertools, MinMaxResult};
use mimalloc::MiMalloc;
use simple_logger::SimpleLogger;

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

fn diff_range_valid(config: &[i64]) -> bool {
    let invalid_diffs = config
        .iter()
        .tuple_windows()
        .map(|(i, j)| i.abs_diff(*j))
        .filter(|i| !(1..=3).contains(i))
        .count();

    invalid_diffs == 0
}

fn generate_configs(base_config: &[i64]) -> Vec<Vec<i64>> {
    let mut output = vec![base_config.to_vec()];

    for combination in base_config.iter().combinations(base_config.len() - 1) {
        output.push(combination.iter().map(|i| **i).collect());
    }

    output
}

fn check_if_ascending_descending(config: &[i64]) -> bool {
    let diffs = config
        .iter()
        .tuple_windows()
        .map(|(f, s)| f - s)
        .collect::<Vec<_>>();
    match diffs.iter().minmax() {
        MinMaxResult::MinMax(min, max) => (min * max) > 0,
        MinMaxResult::OneElement(_) | MinMaxResult::NoElements => false,
    }
}

fn calculate_safe_configs(configs: Vec<Vec<i64>>) -> u64 {
    let mut safe_configs: u64 = 0;
    for config in configs {
        let variation_configs = generate_configs(config.as_ref());
        let valid_configs = variation_configs
            .iter()
            .filter(|a| diff_range_valid(a.as_ref()))
            .filter(|a| check_if_ascending_descending(a.as_ref()))
            .collect::<Vec<_>>();

        if !valid_configs.is_empty() {
            println!("✓ {:?} - Found {} configs!", config, valid_configs.len());
            safe_configs += 1;
        } else {
            println!("❌ {:?}", config);
        }
    }
    safe_configs
}

fn parse_input(input_str: &str) -> Vec<Vec<i64>> {
    input_str
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|e| i64::from_str(e).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
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
        assert_eq!(safe_configs, 4);
    }
}
