use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum: i32 = 0;

    let seperator = Regex::new(r"(Game )|(: )|(; )").expect("Invalid regex");

    let file_name = &args[1];
    for line in fs::read_to_string(file_name.as_str()).unwrap().lines() {
        let games: Vec<&str> = seperator.split(line).skip(1).collect();
        let game_id = games[0].parse::<i32>().unwrap();
        let mut valid = true;

        for game in games.iter().skip(1) {
            for cubes in game.split(", ") {
                let values: Vec<&str> = cubes.split(' ').collect();
                match values[1] {
                    "red" => {
                        if values[0].parse::<i32>().unwrap() > 12 {
                            valid = false;
                            break;
                        }
                    }
                    "green" => {
                        if values[0].parse::<i32>().unwrap() > 13 {
                            valid = false;
                            break;
                        }
                    }
                    "blue" => {
                        if values[0].parse::<i32>().unwrap() > 14 {
                            valid = false;
                            break;
                        }
                    }
                    _ => (),
                }
                if !valid {
                    break;
                }
            }

            if !valid {
                break;
            }
        }

        if valid {
            sum += game_id;
        }
    }

    println!("answer is: {sum}");
}
