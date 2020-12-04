/* Day 2 Password Philosophy
 * Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

*/

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};
use std::path::Path;
use std::env;

const COLON: char = ':';
const SPACE: char = ' ';
const DASH: char = '-';

fn main() {
    println!("Advent of Code 2020 - Day 2");

    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut num_entries:u32 = 0;

    let mut valid_count:u32 = 0;
    let mut invalid_count:u32 = 0;
    let mut valid_count_pt2:u32 = 0;
    let mut invalid_count_pt2:u32 = 0;

    let file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}",display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let entry:PasswordEntry=parse_line(line.unwrap());
        let entry_clone=entry.clone();
        num_entries+=1;
        if validate_entry_part_one(entry){
            valid_count+=1;
        }else{
            invalid_count+=1;
        }

        if validate_entry_part_two(entry_clone){
            valid_count_pt2+=1;
        }else{
            invalid_count_pt2+=1;
        }
    }

    println!("Part 1: Out of {} entries:\n {} was valid \n {} was invalid.",
             num_entries, valid_count, invalid_count);

    println!("Part 2: Out of {} entries:\n {} was valid \n {} was invalid.",
             num_entries, valid_count_pt2, invalid_count_pt2);


}

fn parse_line(s:String) -> PasswordEntry{
    let split: Vec<&str> = s.split(COLON).collect();
    let policy: &str = split[0];
    let passwd: &str = split[1];

    let policy_split: Vec<&str> = policy.split(SPACE).collect();
    let policy_range: &str = policy_split[0];
    let policy_char: char = policy_split[1].chars().next().unwrap();

    let policy_range: Vec<&str> = policy_range.split(DASH).collect();
    let policy_min:u32 = policy_range[0].parse().unwrap();
    let policy_max:u32 = policy_range[1].parse().unwrap();

    //println!("{}-{} {}:{}",policy_min,policy_max,policy_char,passwd.to_string());

    return PasswordEntry{
        min : policy_min,
        max : policy_max,
        char : policy_char,
        password : passwd.to_string()
    }

}

fn validate_entry_part_one(e:PasswordEntry) -> bool{
    let match_count = e.password.matches(e.char).count() as u32;

    if match_count < e.min || match_count > e.max {
        return false;
    }
    return true;
}


fn validate_entry_part_two(e:PasswordEntry) -> bool{
    let mut count = 0;
    let all_match_index: Vec<_> = e.password.match_indices(e.char).collect();
    if all_match_index.len() < 1 {
        return false;
    }
    for tup in all_match_index {
        if tup.0 as u32  == e.min {
            //print!("{}m,",tup.0 as u32 );
            count +=1;
        }else if tup.0 as u32  == e.max {
            //print!("{}M,", tup.0 as u32);
            count +=1;
        }else{
            //print!("{},",tup.0 as u32)
        }
    }
    //print!(" - count {}\n\n",count);

    return count == 1
}

#[derive(Clone)]
struct PasswordEntry {
    min: u32,
    max: u32,
    char: char,
    password: String
}
