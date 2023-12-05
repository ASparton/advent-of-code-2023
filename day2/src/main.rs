use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;
const INPUT_FILE_PATH: &str = "resources/input.txt";

fn main() {
    let mut game_ids_sum: u32 = 0;
    let games = read_file_to_str_list(INPUT_FILE_PATH);

    for game in games.iter() {
        let mut current_game_valid = true;
        for draw in get_draws_from_game(game) {
            let cubes_count = get_cubes_count_from_draw(&draw);
            if !is_draw_valid(cubes_count.0, cubes_count.1, cubes_count.2) {
                current_game_valid = false;
                break;
            }
        }

        println!("{}", game);
        println!("{}", current_game_valid);

        if current_game_valid {
            game_ids_sum += extract_game_id(game);
        }
    }

    println!("{}", game_ids_sum);
}

fn read_file_to_str_list(file_path: &str) -> Vec<String> {
    let input = fs::read_to_string(file_path).expect("Could not read file.");
    let lines: Vec<&str> = input.split('\n').collect();
    return lines.iter().map(|line| String::from(*line)).collect();
}

fn get_draws_from_game(game: &String) -> Vec<String> {
    let mut draws: Vec<String>;
    let game_parts: Vec<&str> = game.split(':').collect();
    let draws_str = String::from(String::from(game_parts[1]).trim_start());
    draws = draws_str
        .split(';')
        .map(|draw_str| String::from(draw_str))
        .collect();
    draws = draws
        .iter()
        .map(|draw| String::from(String::from(draw).trim_start()))
        .collect();
    draws.iter().map(|draw| draw.replace(",", "")).collect()
}

fn get_cubes_count_from_draw(draw: &String) -> (u32, u32, u32) {
    let mut red_cubes_count: u32 = 0;
    let mut green_cubes_count: u32 = 0;
    let mut blue_cubes_count: u32 = 0;

    let values: Vec<&str> = draw.split(' ').collect();
    for (index, value) in values.iter().enumerate() {
        if *value == "red" {
            red_cubes_count = String::from(values[index - 1])
                .parse()
                .expect("Should be a number")
        }
    }
    for (index, value) in values.iter().enumerate() {
        if *value == "green" {
            green_cubes_count = String::from(values[index - 1])
                .parse()
                .expect("Should be a number")
        }
    }
    for (index, value) in values.iter().enumerate() {
        if *value == "blue" {
            blue_cubes_count = String::from(values[index - 1])
                .parse()
                .expect("Should be a number")
        }
    }

    return (red_cubes_count, green_cubes_count, blue_cubes_count);
}

fn is_draw_valid(red_cubes_count: u32, green_cubes_count: u32, blue_cubes_count: u32) -> bool {
    return red_cubes_count <= MAX_RED
        && green_cubes_count <= MAX_GREEN
        && blue_cubes_count <= MAX_BLUE;
}

fn extract_game_id(game: &String) -> u32 {
    let game_parts: Vec<&str> = game.split(':').collect();
    let game_title_parts: Vec<&str> = game_parts[0].split(' ').collect();
    let game_id_str: String = String::from(game_title_parts[1]);
    game_id_str.parse().expect("Should be a number")
}
