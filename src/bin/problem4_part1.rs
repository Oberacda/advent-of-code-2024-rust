use clap::Parser;
use log::error;
use std::collections::{HashSet, VecDeque};
use std::process::exit;

use advent_of_code_2024::parse_input_file;
use ndarray::Array2;
use simple_logger::SimpleLogger;
use mimalloc::MiMalloc;

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct FoundWord {
    positions: Vec<(usize, usize)>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct WordCandiate {
    next_position: (usize, usize),
    next_required_char: char,
    previous_positions: VecDeque<(usize, usize)>,
}

fn create_search_matrix(input_string: &str) -> Array2<char> {
    let rows = input_string.lines().collect::<Vec<_>>();

    let no_rows = rows.len();
    let no_columns = rows[0].len();

    let mut search_array = Array2::<char>::default((no_rows, no_columns));

    for (row_index, row) in rows.iter().enumerate() {
        for (column_index, c) in row.chars().enumerate() {
            search_array[[row_index, column_index]] = c;
        }
    }

    search_array
}

fn find_words_in_matrix(search_matrix: &Array2<char>) -> Vec<FoundWord> {
    let mut found_words = Vec::new();

    let mut initial_word_candidates: HashSet<WordCandiate> = HashSet::new();
    search_matrix
        .indexed_iter()
        .filter(|(_, e)| **e == 'X')
        .for_each(|((r, c), _)| {
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_add(1), c),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_sub(1), c),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_add(1), c.saturating_add(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r, c.saturating_add(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_sub(1), c.saturating_add(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_add(1), c.saturating_sub(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r, c.saturating_sub(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
            initial_word_candidates.insert(WordCandiate {
                next_position: (r.saturating_sub(1), c.saturating_sub(1)),
                next_required_char: 'M',
                previous_positions: VecDeque::from([(r, c)]),
            });
        });
    let mut word_candiates: Vec<WordCandiate> = initial_word_candidates
        .iter()
        .map(|a| a.to_owned())
        .collect::<Vec<WordCandiate>>();
    while let Some(next_candidate) = word_candiates.pop() {
        if next_candidate.next_position.0 >= search_matrix.nrows()
            || next_candidate.next_position.1 >= search_matrix.ncols()
        {
            //warn!("Skipping candiate as position is not in search grid!");
            continue;
        }

        let char = search_matrix[[
            next_candidate.next_position.0,
            next_candidate.next_position.1,
        ]];
        if char != next_candidate.next_required_char {
            //warn!("{} != {} : Skipping candiate as char does not match expected value!", char, next_candidate.next_required_char);
            continue;
        }

        // Candiate should be investigated further!
        let previous_position = next_candidate.previous_positions.back().unwrap();
        let col_diff = next_candidate.next_position.1 as isize - previous_position.1 as isize;
        let row_diff = next_candidate.next_position.0 as isize - previous_position.0 as isize;

        let mut previous_positions = next_candidate.previous_positions;
        previous_positions.push_back(next_candidate.next_position);

        let next_char = match char {
            'M' => 'A',
            'A' => 'S',
            'S' => {
                found_words.push(FoundWord {
                    positions: Vec::from(previous_positions),
                });
                continue;
            }
            _ => {
                continue;
            }
        };
        let next_pos_row = next_candidate.next_position.0 as isize + row_diff;
        let next_pos_col = next_candidate.next_position.1 as isize + col_diff;
        if next_pos_row < 0 || next_pos_col < 0 {
            continue;
        }

        word_candiates.push(WordCandiate {
            next_position: (next_pos_row as usize, next_pos_col as usize),
            next_required_char: next_char,
            previous_positions: previous_positions,
        })
    }

    found_words
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
    let search_matrix = create_search_matrix(input_string.as_ref());
    let found_words = find_words_in_matrix(&search_matrix);

    visualize_matches(
        found_words.as_ref(),
        search_matrix.nrows(),
        search_matrix.ncols(),
    );
    println!("Found {} XMAS!", found_words.len());
}

fn visualize_matches(found_words: &[FoundWord], rows: usize, cols: usize) {
    let mut array = Array2::<char>::from_elem((rows, cols), '.');
    for found_word in found_words {
        array[found_word.positions[0]] = 'X';
        array[found_word.positions[1]] = 'M';
        array[found_word.positions[2]] = 'A';
        array[found_word.positions[3]] = 'S';
    }

    for r in 0..rows {
        for c in 0..cols {
            print!("{}", array[[r, c]]);
        }
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_search_matrix, find_words_in_matrix, visualize_matches};

    #[test]
    fn test_example() {
        let input_string = include_str!("../../problems/problem4_test.txt");
        let search_matrix = create_search_matrix(input_string);

        let found_words = find_words_in_matrix(&search_matrix);

        visualize_matches(
            found_words.as_ref(),
            search_matrix.nrows(),
            search_matrix.ncols(),
        );

        assert_eq!(found_words.len(), 18);
    }
}
