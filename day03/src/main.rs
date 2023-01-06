use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let mut total_priority = 0;
    if let Ok(knapsacks) = read_lines("input") {
        for line in knapsacks {
            if let Ok(knapsack) = line {
                total_priority += priority(find_common_char(knapsack.split_at(knapsack.len()/2)).unwrap());
            }
        }
    }
    println!("{}", total_priority);
}

fn find_common_char((first, second): (&str, &str)) -> Option<char> {
    let mut map = HashMap::new();
    for character in first.chars() {
        map.entry(character).or_insert(1);
    }
    for character in second.chars() {
        if map.contains_key(&character) {
            return Some(character);
        }
    }
    return None;
}

fn priority (letter: char) -> i32 {
    if letter.is_ascii_lowercase() {
        letter as i32 - 'a' as i32 + 1
    } else {
        letter as i32 - 'A' as i32 + 27
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}