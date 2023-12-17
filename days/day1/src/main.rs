use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sum: i32 = 0;
    let numbers = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let file_name = &args[1];
    for line in fs::read_to_string(file_name.as_str()).unwrap().lines() {
        let mut fst = 'a';
        let mut snd = 'a';
        let mut num = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                if fst == 'a' {
                    fst = c;
                }
                snd = c;
                num = String::new();
            } else {
                num.push(c);
                for (number, value) in numbers.iter() {
                    if num.ends_with(number) {
                        if fst == 'a' {
                            fst = *value;
                        }
                        snd = *value;
                        break;
                    }
                }
            }
        }

        if fst != 'a' {
            let mut number = String::new();
            number.push(fst);
            number.push(snd);

            sum += number.parse::<i32>().unwrap();
        }
    }

    println!("answer is: {sum}");
}
