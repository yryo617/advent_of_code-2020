/* Advent of Code 2020 - Day 3 */

/*The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter? */


use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;
use std::env;

const TREE:char = '#';
const OPEN:char = '.';


fn main() {
    println!("Advent of Code 2020 - Day 3!");

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let x:i8 = args[2].trim().parse().expect("not valid uint");
    let y:i8 = args[3].trim().parse().expect("not valid uint");

    let mut width:i32 = 0;

    let map:Vec<Vec<char>> = load_map(path)?;
    warn(map);

}

fn load_map(p:&Path) -> Result<Vec<Vec<char>>,String> {

}

fn check_tree(c:char) -> Result<bool, String> {

    match c {
        TREE => Ok(true),
        OPEN => Ok(false),
        _ => Err("Not tree nor open space!".to_string())
    }

}
