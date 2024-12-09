use clap::Parser;
use log::error;
use std::collections::{HashSet, VecDeque};
use std::process::exit;

use advent_of_code_2024::parse_input_file;
use ndarray::Array2;
use simple_logger::SimpleLogger;

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
    center: (usize, usize),
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct WordCandiate {
    center: (usize, usize),
}

fn get_candidate_value(search_grid: &Array2<char>, start: (usize, usize), row_offset: isize, col_offset: isize) -> Option<char> {
    let col_signed = match col_offset.checked_add(start.1 as isize) {
        Some(x) => x,
        None => return None,
    };
    
    let col = if col_signed < 0 || search_grid.ncols() <= col_signed as usize {
        return None;
    } else {
        col_signed as usize
    };

    let row_signed = match row_offset.checked_add(start.0 as isize) {
        Some(y) => y,
        None => return None,
    };

    let row = if row_signed < 0 || search_grid.nrows() <= row_signed as usize {
        return None;
    } else {
        row_signed as usize
    };
    
    search_grid.get((row, col)).copied()
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
        .filter(|(_, e)| **e == 'A')
        .for_each(|((r, c), _)| {
            initial_word_candidates.insert(WordCandiate { center: (r, c)});
        });
    
    let mut word_candiates: Vec<WordCandiate> = initial_word_candidates
        .iter()
        .map(|a| a.to_owned())
        .collect::<Vec<WordCandiate>>();
    while let Some(next_candidate) = word_candiates.pop() {
       
        let top_left_opt = get_candidate_value(search_matrix, next_candidate.center, 1, -1);
        let top_right_opt = get_candidate_value(search_matrix, next_candidate.center, 1, 1);
        let bottom_left_opt = get_candidate_value(search_matrix, next_candidate.center, -1, -1);
        let bottom_right_opt = get_candidate_value(search_matrix, next_candidate.center, -1, 1);
        
        if ! (top_left_opt.is_some() && top_right_opt.is_some() && bottom_left_opt.is_some() && bottom_right_opt.is_some()) {
            continue;
        }
        let top_left = top_left_opt.unwrap();
        let top_right = top_right_opt.unwrap();
        let bottom_left = bottom_left_opt.unwrap();
        let bottom_right = bottom_right_opt.unwrap();
        
        match top_left {
            'M' => {
                match top_right {
                    'M' => {
                        if bottom_left == 'S' && bottom_right == 'S' {
                           found_words.push(FoundWord { center: next_candidate.center }); 
                        } else {
                            continue;
                        }
                    },
                    'S' => {

                        if bottom_left == 'M' && bottom_right == 'S' {
                            found_words.push(FoundWord { center: next_candidate.center });
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            'S' => {

                match top_right {
                    'M' => {
                        if bottom_left == 'S' && bottom_right == 'M' {
                            found_words.push(FoundWord { center: next_candidate.center });
                        } else {
                            continue;
                        }
                    },
                    'S' => {
                        if bottom_left == 'M' && bottom_right == 'M' {
                            found_words.push(FoundWord { center: next_candidate.center });
                        } else {
                            continue;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
            _ => {
                continue;
            }
        }
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
        &search_matrix,
    );
    println!("Found {} X-MAS!", found_words.len());
}

fn visualize_matches(found_words: &[FoundWord], search_grid: &Array2<char>) {
    let mut array = Array2::<char>::from_elem((search_grid.nrows(), search_grid.ncols()), '.');
    for found_word in found_words {
        
        array[found_word.center] = 'A';
        let top_left = get_candidate_value(search_grid, found_word.center, 1, -1).unwrap();
        let top_right = get_candidate_value(search_grid, found_word.center, 1, 1).unwrap();
        let bottom_left = get_candidate_value(search_grid, found_word.center, -1, -1).unwrap();
        let bottom_right = get_candidate_value(search_grid, found_word.center, -1, 1).unwrap();
        array[[found_word.center.0 + 1, found_word.center.1 - 1]] = top_left;
        array[[found_word.center.0 + 1, found_word.center.1 + 1]] = top_right;
        array[[found_word.center.0 - 1, found_word.center.1 - 1]] = bottom_left;
        array[[found_word.center.0 - 1, found_word.center.1 + 1]] = bottom_right;
    }

    for r in 0..array.nrows() {
        for c in 0..array.ncols() {
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
            &search_matrix,
        );

        assert_eq!(found_words.len(), 9);
    }
}
