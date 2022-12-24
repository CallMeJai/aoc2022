use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut elves = vec![0];
    let mut elf_index = 0;
    let file = File::open("input").unwrap(); // fail if the file fails to open
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(calories) = line {
            if calories != ""
            {
                elves[elf_index] += calories.parse::<i32>().unwrap();
            } else {
                elves.push(0);
                elf_index += 1;
            }
        }
    }
    elves.sort_unstable();
    elves.reverse();
    println!("{}", elves[0..3].iter().sum::<i32>());
}
