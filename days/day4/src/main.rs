use std::fs;

fn main() {
    let score: i32 = fs::read_to_string("input_test.txt")
        .unwrap()
        .lines()
        .map(|l| l.split_once(": ").unwrap().1)
        .map(|l| l.split_once(" | ").unwrap())
        .map(|ns| (ns.0.split_whitespace().collect::<Vec<&str>>(), ns.1.split_whitespace().collect::<Vec<&str>>()))
        .map(|(n1, n2)| {
            let mut points = 0;
            for n in n2 {
                if n1.contains(&n) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }
            points
        })
        .sum();

    println!("answer is: {score}");
}
