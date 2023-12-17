use std::fs;

#[derive(Debug)]
enum Type {
    Number(String, usize, usize),
    Star,
    Empty,
}

fn main() {
    let mut grid = Vec::<Vec<Type>>::new();

    for line in fs::read_to_string("input3.txt").unwrap().lines() {
        let mut row = Vec::<Type>::new();
        let mut num = String::new();
        for c in line.chars() {
            if c.is_digit(10) {
                num.push(c);
            } else if c == '*' {
                if num != "" {
                    let len = num.len();
                    for i in 0..len {
                        row.push(Type::Number(num.clone(), i+1, len));
                    }
                    num = String::new();
                }
                row.push(Type::Star);
            } else {
                if num != "" {
                    let len = num.len();
                    for i in 0..len {
                        row.push(Type::Number(num.clone(), i+1, len));
                    }
                    num = String::new();
                }
                row.push(Type::Empty);
            }
        }
        grid.push(row);
    }

    let mut sum = 0;
    let grid_len = grid.len() - 1;

    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            match val {
                Type::Star => {
                    let mut fst = String::new();
                    let mut snd = String::new();

                    if y > 0 {
                        match check_row(x, grid.get(y-1).unwrap()) {
                            (Some(f), Some(s)) => {
                                fst = f;
                                snd = s;
                            }
                            (Some(f), None) => {
                                if fst == "" {
                                    fst = f;
                                } else {
                                    snd = f;
                                }
                            }
                            _ => (),
                        }
                    }
                    if y < grid_len{
                        match check_row(x, grid.get(y + 1).unwrap()) {
                            (Some(f), Some(s)) => {
                                fst = f;
                                snd = s;
                            }
                            (Some(f), None) => {
                                if fst == "" {
                                    fst = f;
                                } else {
                                    snd = f;
                                }
                            }
                            _ => (),
                        }
                    }
                    match check_row(x, row) {
                        (Some(f), Some(s)) => {
                            fst = f;
                            snd = s;
                        }
                        (Some(f), None) => {
                            if fst == "" {
                                fst = f;
                            } else {
                                snd = f;
                            }
                        }
                        _ => (),
                    }

                    if fst != "" && snd != "" {
                        sum += fst.parse::<usize>().unwrap() * snd.parse::<usize>().unwrap();
                    }
                }
                _ => (),
            }
        }
    }

    println!("answer is: {sum}");
}

fn check_row(x: usize, row: &Vec<Type>) -> (Option<String>, Option<String>) {
    let mut fst = None;
    let mut snd = None;

    let mut skip = 0;

    for val in &row[x-1..x+2] {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match val {
            Type::Number(n, i, len) => {
                if fst.is_none() {
                    fst = Some(n.clone());
                } else if snd.is_none() {
                    snd = Some(n.clone());
                }
                if i < len {
                    skip = len - i;
                }
            }
            _ => (),
        }
    }

    return (fst, snd);
}
