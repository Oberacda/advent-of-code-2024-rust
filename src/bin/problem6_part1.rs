use advent_of_code_2024::parse_input_file;
use clap::Parser;
use log::error;
use mimalloc::MiMalloc;
use ndarray::Array2;
use simple_logger::SimpleLogger;
use std::process::exit;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input_file: String,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Orientation {
    TOP,
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Guard {
    pub pos: (usize, usize),
    pub orientation: Orientation,
}

impl Default for Guard {
    fn default() -> Self {
        Guard {
            pos: (0, 0),
            orientation: Orientation::TOP,
        }
    }
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
    let (map, start_position) = create_map(input_string.as_ref());
    let traversed_positions: Vec<(usize, usize)> = find_traveled_path(&map, &start_position);

    visualize_path(traversed_positions.as_ref(), map.nrows(), map.ncols());
    println!("Found {} XMAS!", traversed_positions.len());
}

fn visualize_path(path: &[(usize, usize)], rows: usize, cols: usize) {
    todo!()
}

fn get_next_postion_checked(map: &Array2<bool>, guard: &Guard) -> Option<(usize, usize)> {
    todo!()
}

fn find_traveled_path(map: &Array2<bool>, guard: &Guard) -> Vec<(usize, usize)> {
    let current_pos = guard.clone();
    loop {
        match current_pos.orientation {
            _ => todo!(),
        }
    }
    todo!()
}

fn create_map(input_string: &str) -> (Array2<bool>, Guard) {
    let rows = input_string.lines().collect::<Vec<_>>();

    let no_rows = rows.len();
    let no_columns = rows[0].len();

    let mut map = Array2::<bool>::default((no_rows, no_columns));
    let mut start_pos = Guard::default();

    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    map[[row_index, column_index]] = true;
                }
                '.' => {
                    map[[row_index, column_index]] = false;
                }
                '^' => {
                    map[[row_index, column_index]] = false;
                    start_pos.pos = (row_index, column_index);
                    start_pos.orientation = Orientation::TOP;
                }
                '>' => {
                    map[[row_index, column_index]] = false;
                    start_pos.pos = (row_index, column_index);
                    start_pos.orientation = Orientation::RIGHT;
                }
                '<' => {
                    map[[row_index, column_index]] = false;
                    start_pos.pos = (row_index, column_index);
                    start_pos.orientation = Orientation::LEFT;
                }
                'V' => {
                    map[[row_index, column_index]] = false;
                    start_pos.pos = (row_index, column_index);
                    start_pos.orientation = Orientation::DOWN;
                }
                _ => panic!("Invalid char!"),
            }
        }
    }

    (map, start_pos)
}

#[cfg(test)]
mod tests {
    use crate::{create_map, find_traveled_path, visualize_path};

    #[test]
    fn test_example() {
        let input_string = include_str!("../../problems/problem6_test.txt");
        let (map, start_position) = create_map(input_string);
        let traversed_positions: Vec<(usize, usize)> = find_traveled_path(&map, &start_position);

        visualize_path(traversed_positions.as_ref(), map.nrows(), map.ncols());

        assert_eq!(traversed_positions.len(), 41);
    }
}
