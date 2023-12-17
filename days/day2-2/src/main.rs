use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum: i32 = 0;

    let seperator = Regex::new(r"(Game )|(: )|(; )").expect("Invalid regex");

    let file_name = &args[1];
    for line in fs::read_to_string(file_name.as_str()).unwrap().lines() {
        let games: Vec<&str> = seperator.split(line).skip(1).collect();
        
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for game in games.iter().skip(1) {
            for cubes in game.split(", ") {
                let values: Vec<&str> = cubes.split(' ').collect();
                match values[1] {
                    "red" => {
                        let v = values[0].parse::<i32>().unwrap();
                        if v > min_red {
                            min_red = v;
                        }
                    }
                    "green" => {
                        let v = values[0].parse::<i32>().unwrap();
                        if v > min_green {
                            min_green = v;
                        }
                    }
                    "blue" => {
                        let v = values[0].parse::<i32>().unwrap();
                        if v > min_blue {
                            min_blue = v;
                        }
                    }
                    _ => (),
                }
            }
        }
        
        sum += min_red * min_blue * min_green;
    }

    println!("answer is: {sum}");
}
