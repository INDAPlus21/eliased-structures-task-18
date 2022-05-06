use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::cmp;

fn main() {

    let mut magic_file = File::open("magic-file.txt");

    let mut magic_file = File::open("token.txt");


    let contents : String = fs::read_to_string("token.txt")?.parse()?;
    
    let input = io::stdin();

    let common_index_words = vec![]; 

    for mut line in input.lock().lines().map(|line| line.unwrap()) {
        line = line.to_lowercase(); 
        println!("{}", line);

        for line in contents.split("\n") {
            // index (ES) !?!?!? 
            common_index_words = line.split(" ").collect::<Vec<_>>(); 
    
            if common_index_words[0] == line {
                break; 
            }
        }
    }

    for i in 0..common_index_words.len() {
        index_word = common_index_words[i]

        let mut buffer = common_index_words[i]; 
        let mut korpus = File::open("korpus.txt");

        korpus.read_at(buffer, cmp::max(0, index_word-30)); 
        println!("{}", byte_array);
    }
}

fn hash(to_hash : &str) -> u16 {
    let mut sum = 0; 
    let upper_index = if to_hash.len() <= 3 { to_hash.len() } else { 3 };
    for i in 0..upper_index {
        sum += to_hash.as_bytes()[i] as u16; 
    }
    return sum
}