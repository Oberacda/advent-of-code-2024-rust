use advent_of_code_2024::parse_input_file;
use clap::Parser;
use log::error;
use mimalloc::MiMalloc;
use regex::Regex;
use simple_logger::SimpleLogger;
use std::collections::{HashMap, HashSet};
use std::process::exit;
use std::str::FromStr;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
}

fn parse_input(input_string: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let input_lines = input_string.lines();
    let rules_regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
    let order_regex = Regex::new(r"^((\d)+(,)?)+$").unwrap();

    let mut rules = HashMap::<u64, HashSet<u64>>::new();
    let mut orders = Vec::new();

    for input_line in input_lines {
        if rules_regex.is_match(input_line) {
            let rules_split = input_line.split("|").collect::<Vec<&str>>();
            let left = u64::from_str(rules_split[0]).unwrap();
            let right = u64::from_str(rules_split[1]).unwrap();

            let elem = rules.get_mut(&left);
            if let Some(item) = elem {
                item.insert(right);
            } else {
                rules.insert(left, HashSet::from([right]));
            }

            continue;
        }
        if order_regex.is_match(input_line) {
            let orders_split = input_line.split(",").collect::<Vec<&str>>();
            let mut order = Vec::with_capacity(orders_split.len());
            for order_str in orders_split {
                order.push(order_str.parse::<u64>().unwrap());
            }
            orders.push(order);
        }
    }

    (rules, orders)
}

fn validate_order(rules: &HashMap<u64, HashSet<u64>>, order: &[u64]) -> bool {
    let mut prefix: Vec<&u64> = Vec::new();
    for elem in order {
        if let Some(rhs_set) = rules.get(elem) {
            if prefix.iter().filter(|e| rhs_set.contains(e)).count() > 0 {
                return false;
            }
        }
        prefix.push(elem);
    }
    true
}

fn get_middle_numer(order: &[u64]) -> u64 {
    let middle_index = match order.len().checked_div(2) {
        Some(index) => index,
        None => {
            error!("Could not compute half of {}", order.len());
            exit(-1);
        }
    };
    order[middle_index]
}

fn calculate_result(rules: &HashMap<u64, HashSet<u64>>, orders: &[Vec<u64>]) -> u64 {
    orders
        .iter()
        .filter(|order| validate_order(rules, order))
        .map(|order| get_middle_numer(order))
        .sum::<u64>()
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
    let (rules, orders) = parse_input(&input_string);
    let result = calculate_result(&rules, &orders);

    println!("Result: {}", result);
}
#[cfg(test)]
mod tests {
    use crate::{calculate_result, parse_input};

    #[test]
    fn test_example() {
        let input_string = include_str!("../../problems/problem5_test.txt");
        let (rules, orders) = parse_input(input_string);
        let result = calculate_result(&rules, &orders);

        assert_eq!(result, 143);
    }
}
