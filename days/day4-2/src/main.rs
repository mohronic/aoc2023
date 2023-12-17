use std::fs;

fn main() {
    let mut sum = 0;

    let input = fs::read_to_string("input_test.txt").unwrap();

    let games: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1)
        .map(|l| l.split_once(" | ").unwrap())
        .map(|ns| (ns.0.split_whitespace().collect::<Vec<&str>>(), ns.1.split_whitespace().collect::<Vec<&str>>()))
        .collect();


    let mut copies: Vec<usize> = vec![0; games.len()];
    for (i, (winning_numbers, numbers)) in games.iter().enumerate() {
        let mut wins = 0;
        for n in numbers {
            if winning_numbers.contains(&n) {
                wins += 1;
            }
        }

        sum += 1 + copies[i];

        println!("game {i} with {wins} wins and {} copies result in new sum: {sum}", copies[i]);

        while wins > 0 {
            copies[i+wins] += 1 + copies[i];
            wins -= 1;
        }
    }

    println!("answer is: {sum}");
}
