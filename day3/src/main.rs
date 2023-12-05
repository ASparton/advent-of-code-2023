use std::{
    fs, io,
    ops::{Bound, RangeInclusive},
};

const INPUT_FILE_PATH: &str = "resources/input.txt";

fn main() {
    let lines = read_file_to_str_list(INPUT_FILE_PATH).unwrap();
    let right_boundary: usize = lines[0].len();

    let mut number_ranges: Vec<Vec<RangeInclusive<usize>>> = Vec::new();
    let mut symbol_indexes: Vec<Vec<usize>> = Vec::new();
    for line in lines.iter() {
        number_ranges.push(extract_number_ranges_from_line(line));
        symbol_indexes.push(extract_symbols_index_from_line(line));
    }

    for (line_index, line_number_ranges) in number_ranges.iter().enumerate() {
        println!("{}", lines[line_index]);
        println!("{:?}", line_number_ranges);
        for number_range in line_number_ranges {
            let is_adj = is_number_adjascent_to_symbol(
                &line_index,
                number_range,
                &symbol_indexes,
                &right_boundary,
            );
            println!("Range: {:?}, Adj: {}", number_range, is_adj);
        }
    }
}

fn read_file_to_str_list(file_path: &str) -> Result<Vec<String>, io::Error> {
    Ok(fs::read_to_string(file_path)?
        .split('\n')
        .map(|line| String::from(line))
        .collect())
}

fn extract_number_ranges_from_line(line: &String) -> Vec<RangeInclusive<usize>> {
    let mut number_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    let mut index: usize = 0;
    while index < line.len() {
        if char::is_ascii_digit(&line.chars().nth(index).unwrap()) {
            let mut last_digit_index: usize = index + 1;
            while last_digit_index < line.len()
                && char::is_ascii_digit(&line.chars().nth(last_digit_index).unwrap())
            {
                last_digit_index += 1;
            }
            number_ranges.push(index..=last_digit_index - 1);
            index = last_digit_index;
        } else {
            index += 1;
        }
    }

    return number_ranges;
}

fn extract_symbols_index_from_line(line: &String) -> Vec<usize> {
    let mut symbol_indexes: Vec<usize> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if !char::is_ascii_digit(&character) && character != '.' {
            symbol_indexes.push(index);
        }
    }

    return symbol_indexes;
}

fn is_number_adjascent_to_symbol(
    line_index: &usize,
    number_range: &mut RangeInclusive<usize>,
    symbol_indexes: &Vec<Vec<usize>>,
    right_boundary: &usize,
) -> bool {
    number_range.any(|digit_index| {
        is_digit_adjascent_to_symbol(line_index, &digit_index, symbol_indexes, right_boundary)
    })
}

fn is_digit_adjascent_to_symbol(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
    right_boundary: &usize,
) -> bool {
    symbol_at_top_left(line_index, digit_index, symbol_indexes)
        || symbol_at_left(line_index, digit_index, symbol_indexes)
        || symbol_at_bottom_left(line_index, digit_index, symbol_indexes)
        || symbol_at_bottom(line_index, digit_index, symbol_indexes)
        || symbol_at_bottom_right(line_index, digit_index, symbol_indexes, right_boundary)
        || symbol_at_right(line_index, digit_index, symbol_indexes, right_boundary)
        || symbol_at_top_right(line_index, digit_index, symbol_indexes, right_boundary)
        || symbol_at_top(line_index, digit_index, symbol_indexes)
}

fn symbol_at_left(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
) -> bool {
    *digit_index > 0
        && symbol_indexes[*line_index]
            .binary_search(&(*digit_index - 1))
            .is_ok()
}

fn symbol_at_top_left(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
) -> bool {
    *digit_index > 0
        && *line_index > 0
        && symbol_indexes[*line_index - 1]
            .binary_search(&(*digit_index - 1))
            .is_ok()
}

fn symbol_at_bottom_left(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
) -> bool {
    *digit_index > 0
        && *line_index < symbol_indexes.len() - 1
        && symbol_indexes[*line_index + 1]
            .binary_search(&(*digit_index - 1))
            .is_ok()
}

fn symbol_at_right(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
    right_boundary: &usize,
) -> bool {
    *digit_index < *right_boundary - 1
        && symbol_indexes[*line_index]
            .binary_search(&(*digit_index + 1))
            .is_ok()
}

fn symbol_at_top_right(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
    right_boundary: &usize,
) -> bool {
    *digit_index < *right_boundary - 1
        && *line_index > 0
        && symbol_indexes[*line_index]
            .binary_search(&(digit_index + 1))
            .is_ok()
}

fn symbol_at_bottom_right(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
    right_boundary: &usize,
) -> bool {
    *digit_index < *right_boundary - 1
        && *line_index < symbol_indexes.len() - 1
        && symbol_indexes[*line_index]
            .binary_search(&(*digit_index + 1))
            .is_ok()
}

fn symbol_at_top(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
) -> bool {
    *line_index > 0
        && symbol_indexes[*line_index - 1]
            .binary_search(digit_index)
            .is_ok()
}

fn symbol_at_bottom(
    line_index: &usize,
    digit_index: &usize,
    symbol_indexes: &Vec<Vec<usize>>,
) -> bool {
    *line_index < symbol_indexes.len() - 1
        && symbol_indexes[*line_index + 1]
            .binary_search(digit_index)
            .is_ok()
}
