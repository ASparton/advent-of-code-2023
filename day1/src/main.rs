use std::fs;

fn main() {
    const FILE_PATH: &str = "resources/input.txt";
    let inputs = read_file_to_str_list(FILE_PATH);
    let mut calibaration_value_sum: u32 = 0;
    for input in inputs {
        calibaration_value_sum += get_number_from_line(&input);
    }
    println!("{}", calibaration_value_sum);
}

fn read_file_to_str_list(file_path: &str) -> Vec<String> {
    let input = fs::read_to_string(file_path).expect("Could not read file.");
    let lines: Vec<&str> = input.split('\n').collect();
    return lines.iter().map(|line| String::from(*line)).collect();
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
