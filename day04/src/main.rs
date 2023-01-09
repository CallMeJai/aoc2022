use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut ctr = 0;
    if let Ok(assignments) = read_lines("input") {
        for assignment in assignments {
            if let Ok(line) = assignment {
                let endpts: Vec<i32> = line.split(&['-', ',']).map(|num| num.parse().unwrap()).collect();
                ctr += match fully_contains(endpts[0], endpts[1], endpts[2], endpts[3]) {
                    true => 1,
                    false => 0,
                }
            }
        }
    }
    println!("{}", ctr);
}

fn fully_contains(first_lower_bound: i32, first_upper_bound: i32, second_lower_bound: i32, second_upper_bound: i32) -> bool
{
    (first_lower_bound <= second_lower_bound && first_upper_bound >= second_upper_bound) || (first_lower_bound >= second_lower_bound && first_upper_bound <= second_upper_bound)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}