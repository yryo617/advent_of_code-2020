// advent of code day one
//the Elves in accounting just need you to fix your expense report (your puzzle input);
//apparently, something isn't quite adding up.
//Specifically, they need you to find the two entries that sum to 2020 and then multiply
//those two numbers together.
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    println!("Advent of code 2020 - Day 1 - Part 2!");
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();
    let mut entries: Vec<i32> = Vec::new();
    let index: i32 = 0;

    let file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}",display, why),
        Ok(file) => file,
    };

    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                entries.push(ip.parse::<i32>().unwrap());
            }
        }
    }
    println!("lines:{}",entries.len());

    'outer: for index in 0..entries.len()-3 {
        'innter: for inner_index in index+1..entries.len()-2 {
            for inner_inner_index in index+2..entries.len()-1{
                if entries[index] + entries[inner_index]
                   + entries[inner_inner_index] == 2020 {
                    let multiples: i32 = entries[index]*entries[inner_index]
                                         *entries[inner_inner_index];
                    println!("Found a combination: {},{},{}",
                             entries[index],entries[inner_index],
                             entries[inner_inner_index]);
                    println!("{}",multiples);
                    return();
                }
            }

        }

    }
    println!("Nothing found!");

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
