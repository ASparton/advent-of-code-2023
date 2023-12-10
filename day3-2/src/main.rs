use std::{fs, io, ops::RangeInclusive};

const INPUT_FILE_PATH: &str = "resources/input.txt";

fn main() {
    let lines = read_file_to_str_list(INPUT_FILE_PATH).unwrap();

    let mut adj_number_sum: u32 = 0;
    let mut number_ranges: Vec<Vec<RangeInclusive<usize>>> = Vec::new();
    let mut gear_indexes: Vec<Vec<usize>> = Vec::new();
    for line in lines.iter() {
        number_ranges.push(extract_number_ranges_from_line(line));
        gear_indexes.push(extract_gears_index_from_line(line));
    }

    for (line_index, gear_indexes) in gear_indexes.iter().enumerate() {
        println!("Line {}: {:?}", line_index, gear_indexes);
        for gear_index in gear_indexes.iter() {
            let adj_number_ranges =
                get_adjascent_number_ranges(&line_index, gear_index, &mut number_ranges);
            println!(
                "Adj ranges for gear at index {}: {:?}",
                gear_index, adj_number_ranges
            );
        }
    }

    println!("Sum: {}", adj_number_sum);
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

fn extract_gears_index_from_line(line: &String) -> Vec<usize> {
    let mut gear_indexes: Vec<usize> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if character == '*' {
            gear_indexes.push(index);
        }
    }

    return gear_indexes;
}

fn get_number_in_line_from_range(line: &String, number_range: &mut RangeInclusive<usize>) -> u32 {
    let mut adj_number_str = String::new();
    for index in number_range {
        adj_number_str.push(line.chars().nth(index).unwrap());
    }
    adj_number_str.parse::<u32>().unwrap()
}

fn get_adjascent_number_ranges(
    line_index: &usize,
    gear_index: &usize,
    digit_indexes: &mut Vec<Vec<RangeInclusive<usize>>>,
) -> Vec<(usize, RangeInclusive<usize>)> {
    let mut adj_number_ranges: Vec<(usize, RangeInclusive<usize>)> = Vec::new();
    let possible_adj_number_ranges = vec![
        (
            *line_index,
            get_number_range_at_left(line_index, gear_index, digit_indexes),
        ),
        (
            *line_index,
            get_number_range_at_right(line_index, gear_index, digit_indexes),
        ),
        (
            *line_index - 1,
            get_number_range_at_top(line_index, gear_index, digit_indexes),
        ),
        (
            *line_index + 1,
            get_number_range_at_bottom(line_index, gear_index, digit_indexes),
        ),
    ];
    for possible_adj_number_range in possible_adj_number_ranges.into_iter() {
        match possible_adj_number_range.1 {
            None => (),
            Some(adj_number_range) => {
                adj_number_ranges.push((possible_adj_number_range.0, adj_number_range))
            }
        }
    }
    adj_number_ranges
}

fn get_number_range_at_left(
    line_index: &usize,
    gear_index: &usize,
    digit_indexes: &mut Vec<Vec<RangeInclusive<usize>>>,
) -> Option<RangeInclusive<usize>> {
    for range in digit_indexes[*line_index].iter_mut() {
        if *(range.end()) == gear_index - 1 {
            return Some(range.clone());
        }
    }
    None
}

fn get_number_range_at_right(
    line_index: &usize,
    gear_index: &usize,
    digit_indexes: &mut Vec<Vec<RangeInclusive<usize>>>,
) -> Option<RangeInclusive<usize>> {
    for range in digit_indexes[*line_index].iter_mut() {
        if *(range.start()) == gear_index + 1 {
            return Some(range.clone());
        }
    }
    None
}

fn get_number_range_at_top(
    line_index: &usize,
    gear_index: &usize,
    digit_indexes: &mut Vec<Vec<RangeInclusive<usize>>>,
) -> Option<RangeInclusive<usize>> {
    if *line_index == 0 {
        return None;
    }
    for range in digit_indexes[*line_index - 1].iter_mut() {
        match range.find(|index| {
            index == gear_index || *index == gear_index - 1 || *index == gear_index + 1
        }) {
            None => (),
            Some(_) => return Some(range.clone()),
        }
    }
    None
}

fn get_number_range_at_bottom(
    line_index: &usize,
    gear_index: &usize,
    digit_indexes: &mut Vec<Vec<RangeInclusive<usize>>>,
) -> Option<RangeInclusive<usize>> {
    if *line_index >= digit_indexes.len() - 1 {
        return None;
    }
    for range in digit_indexes[*line_index + 1].iter_mut() {
        match range.find(|index| {
            index == gear_index || *index == gear_index - 1 || *index == gear_index + 1
        }) {
            None => (),
            Some(_) => return Some(range.clone()),
        }
    }
    None
}
