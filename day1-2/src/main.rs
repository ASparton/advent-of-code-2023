use std::collections::HashMap;
use std::fs;

fn main() {
    const FILE_PATH: &str = "resources/input.txt";
    let inputs = read_file_to_str_list(FILE_PATH);
    let mut calibaration_value_sum: u32 = 0;
    for input in inputs {
        let converted_writteng_digits_line = convert_written_digits_to_digits(&input);
        calibaration_value_sum += get_number_from_line(&converted_writteng_digits_line);
    }
    println!("{}", calibaration_value_sum);
}

fn read_file_to_str_list(file_path: &str) -> Vec<String> {
    let input = fs::read_to_string(file_path).expect("Could not read file.");
    let lines: Vec<&str> = input.split('\n').collect();
    return lines.iter().map(|line| String::from(*line)).collect();
}

fn convert_written_digits_to_digits(line: &String) -> String {
    let written_digits: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut converted_string = line.clone();

    let mut char_index: usize = 0;
    loop {
        if char_index == converted_string.len() {
            break;
        }

        for (written_digit, digit) in written_digits.iter() {
            let end_word_index = char_index + written_digit.len() - 1;
            if end_word_index < converted_string.len() {
                let word_range = char_index..=end_word_index;
                if converted_string.get(word_range.clone()).expect("") == *written_digit {
                    converted_string.replace_range(word_range, *digit);
                    if char_index + 1 < converted_string.len() {
                        converted_string
                            .insert(char_index + 1, written_digit.chars().last().expect(""));
                    }
                    break;
                }
            }
        }

        char_index += 1;
    }

    return converted_string;
}

fn get_number_from_line(line: &String) -> u32 {
    let only_numbers_line = extract_numbers_from_line(line);
    let mut number_as_str = String::new();
    number_as_str.push(
        only_numbers_line
            .chars()
            .nth(0)
            .expect("First index should be included."),
    );
    number_as_str.push(
        only_numbers_line
            .chars()
            .nth(only_numbers_line.len() - 1)
            .expect("Last index should be included."),
    );
    number_as_str.parse().expect("Should be a number")
}

fn extract_numbers_from_line(line: &String) -> String {
    let mut only_digits_line = line.clone();
    let mut remove_index: usize = 0;
    for character in line.chars() {
        if !char::is_ascii_digit(&character) {
            only_digits_line.remove(remove_index);
        } else {
            remove_index += 1;
        }
    }
    only_digits_line
}
