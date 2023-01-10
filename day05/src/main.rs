use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // hard code a nice representation of drawing
    // first vector empty so indexing isn't off by one
    let mut stacks = vec![
        vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'], 
        vec!['J', 'R', 'V', 'L'], 
        vec!['S', 'Q', 'F'], 
        vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'], 
        vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'], 
        vec!['S', 'W', 'T', 'C', 'H', 'F'], 
        vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'], 
        vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'], 
        vec!['J', 'B', 'W', 'V', 'P']];
    if let Ok(moves) = read_lines("input") {
        let mut skip_lines = 10;
        for mv in moves {
            if skip_lines == 0 {
                if let Ok(line) = mv {
                    let spls: Vec<_> = line.split("move ").collect();
                    let spls: Vec<_> = spls[1].split(" from ").collect();
                    let num_crates = spls[0].parse().unwrap();
                    let stax: Vec<usize> = spls[1].split(" to ").map(|num| num.parse().unwrap()).collect();
                    crane(num_crates, stax[0], stax[1], &mut stacks);
                }
            }
            else {
                skip_lines -= 1;
            }
        }
    }
    println!("{}", stacks.into_iter().map(|mut stack| stack.pop().unwrap()).collect::<String>());
}

fn crane (num_crates: i32, start_stack: usize, end_stack: usize, stacks: &mut Vec<Vec<char>>) {
        let mut temp_vec = Vec::new();
        for _ in 0..num_crates {
            temp_vec.push(stacks[start_stack - 1].pop().unwrap());
        }
        for _ in 0..num_crates {
            stacks[end_stack - 1].push(temp_vec.pop().unwrap());
        }
        
    }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}
