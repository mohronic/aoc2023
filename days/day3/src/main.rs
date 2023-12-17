use std::fs;

fn main() {
    let mut num_grid_current = Vec::<(Vec<usize>, String)>::new();
    let mut num_grid_previous = Vec::<(Vec<usize>, String)>::new();

    let mut sign_grid_current = Vec::<usize>::new();
    let mut sign_grid_previous = Vec::<usize>::new();

    let mut sum = 0;

    for line in fs::read_to_string("input3.txt").unwrap().lines() {
        let mut num_positions = Vec::<usize>::new();
        let mut num = String::new();

        for (x, c) in line.chars().enumerate() {
            if c == '.' && num != "" {
                let mut added = false;
                if sign_grid_current
                    .last()
                    .is_some_and(|sx| num_positions[0] - sx == 1)
                {
                    sum += num.parse::<i32>().unwrap();
                    added = true;
                } else {
                    for &nx in num_positions.as_slice() {
                        if sign_grid_previous
                            .as_slice()
                            .into_iter()
                            .any(|&sx| usize::abs_diff(sx, nx) <= 1)
                        {
    
                            sum += num.parse::<i32>().unwrap();
                            added = true;
                            break;
                        }
                    }
                }

                if !added {
                    num_grid_current.push((num_positions, num));
                }
                num_positions = Vec::<usize>::new();
                num = String::new();
            } else if c.is_digit(10) {
                num.push(c);
                num_positions.push(x);
            } else if c != '.' {
                sign_grid_current.push(x);
                if num != "" {
                    sum += num.parse::<i32>().unwrap();
                    num_positions = Vec::<usize>::new();
                    num = String::new();
                }

                let mut remove = Vec::<usize>::new();
                for (i,(pos, n)) in num_grid_previous.as_slice().into_iter().enumerate() {
                    if pos.iter().any(|&p| usize::abs_diff(x, p) <= 1) {
                        sum += n.parse::<i32>().unwrap();
                        remove.push(i);
                    }
                }
                remove.reverse();
                for r in remove {
                    num_grid_previous.remove(r);
                }
            }
        }

        if num != "" {
            let mut added = false;
            if sign_grid_current
                .last()
                .is_some_and(|sx| num_positions[0] - sx == 1)
            {
                sum += num.parse::<i32>().unwrap();
                added = true;
            } else {
                for &nx in num_positions.as_slice() {
                    if sign_grid_previous
                        .as_slice()
                        .into_iter()
                        .any(|&sx| usize::abs_diff(sx, nx) <= 1)
                    {

                        sum += num.parse::<i32>().unwrap();
                        added = true;
                        break;
                    }
                }
            }

            if !added {
                num_grid_current.push((num_positions, num));
            }
        }

        num_grid_previous = num_grid_current;
        num_grid_current = Vec::<(Vec<usize>, String)>::new();

        sign_grid_previous = sign_grid_current;
        sign_grid_current = Vec::<usize>::new();
    }

    println!("answer is: {sum}");
}
