use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn main() {
    let mut total_priority = 0;
    if let Ok(knapsacks) = read_lines("input") {
        // there must be a better way to read 3 lines at a time
        // but i don't want to look for it
        let mut ctr = 0;
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        for line in knapsacks {
            if ctr == 0 {
                line1 = line.unwrap();
            } else if ctr == 1 {
                line2 = line.unwrap();
            } else {
                line3 = line.unwrap();
                total_priority += priority(find_common_char((&line1, &line2, &line3)).unwrap());
                ctr = -1;
            }
            ctr += 1;
        }
    }
    println!("{}", total_priority);
}

fn find_common_char((first, second, third): (&str, &str, &str)) -> Option<char> {
    let mut map = HashMap::new();
    for character in first.chars() {
        map.entry(character).or_insert(1);
    }
    for character in second.chars() {
        if map.contains_key(&character) {
            map.insert(character, 2);
        }
    }
    for character in third.chars() {
        if map.get(&character) == Some(&2) {
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