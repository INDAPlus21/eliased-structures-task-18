use std::fs;
use std::fs::File;
// use std::io::{self, BufRead};
use std::cmp;
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let mut magic_file = File::open("../magic-file.txt");

    let mut index_file = File::open("../index-file.txt")?;

    let magic_contents: String = fs::read_to_string("../magic-file.txt")?.parse()?;

    let string_index_file: String = fs::read_to_string("../index-file.txt")?.parse()?;

    let byte_index_file = fs::read("../index-file.txt")?;

    // let string_korpus_file: String = fs::read_to_string("../korpus.txt")?.parse()?;

    let byte_korpus = fs::read("../korpus.txt")?;

    let input = io::stdin();

    'outer: for mut line in input.lock().lines().map(|line| line.unwrap()) {
        line = line.to_lowercase();
        println!("{}", line);

        // could convert the magic file to a hash map, alternatively sort it 
        let hash = hash(&line);
        println!("{}", hash);

        for line in magic_contents.split("\n") {
            let mut split = line.split(" ").collect::<Vec<_>>();
            let magic_hash: u16 = split[0].parse().unwrap();
            let byte_index: usize = split[1].parse().unwrap();

            // println!("{}", magic_hash);
    
            if hash == magic_hash {
                let indexes = &string_index_file.as_bytes()[byte_index+1..byte_index+100];
                let indexes_string =  String::from_utf8_lossy(indexes); 
                let indexes_array: Vec<&str> = indexes_string.split(" ").collect(); //::<Vec&str>>(); 
                println!("{:?}", String::from_utf8_lossy(indexes_array));
                
                println!("Ordet förekommer {:?} gånger i korpus", indexes_array.len()); 
                let mut korpus_iter = 0; 
                for index in indexes_array {
                    let number_index = index.parse::<usize>().unwrap();
                    println!("number index {:?}", number_index); 

                    let korpus_line = &byte_korpus[number_index-50..number_index+50];
                    let korpus_string = String::from_utf8_lossy(korpus_line); 
                    println!("{}", korpus_string); 
                    korpus_iter += 1; 

                    if korpus_iter > 25 {
                        break 'outer;
                    }
                }
        
            }
        } 
    }

    /* let mut iter = 0;
    for line in contents.split("\n") {
        let mut split = line.split(" ").collect::<Vec<_>>();
        let hash = split[0];
        let byte_index: usize = split[1].parse().unwrap();
        println!("{:?}", hash);
        println!("{:?}", byte_index);
        /* for line in string_index_file.as_bytes()[byte_index+1..].split("\n") {

        } */
        let indexes = &string_index_file.as_bytes()[byte_index+1..byte_index+100];
        println!("{:?}", String::from_utf8_lossy(indexes));
        // 32 == space in ascii
        iter += 1;
        if iter > 10 {
            break;
        }
    } */

    // let input = io::stdin();

    /* let contents : String = fs::read_to_string("token.txt")?.parse()?;
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
    } */

    /* let index_reader = BufReader::new(index_file);

    for line in index_reader.lines() {
        // let byte_indexes: Vec<&str> = line?.split(" ") //.collect::<Vec<&str>>();
        let byte_indexes = line?.split(" "); //.collect::<Vec<&str>>();
        // println!("{:?}", line?);
        println!("{:?}", byte_indexes);
    } */

    /* let contents: String = fs::read_to_string("../index-file.txt")?.parse()?;

    let mut iter = 0;
    for line in contents.split("\n") {
        let mut byte_indexes = line.split(" ").collect::<Vec<_>>();
        let word = byte_indexes[0];
        byte_indexes.remove(0);
        println!("{:?}", word);
        println!("{:?}", byte_indexes);
        iter += 1;
        if iter > 3 {
            break;
        }
    } */

    /* for i in 0..common_index_words.len() {
        index_word = common_index_words[i]

        let mut buffer = common_index_words[i];
        let mut korpus = File::open("korpus.txt");

        korpus.read_at(buffer, cmp::max(0, index_word-30));
        println!("{}", byte_array);
    } */

    Ok(())
}

fn hash(to_hash: &str) -> u16 {
    let mut sum = 0;
    let upper_index = if to_hash.len() <= 3 { to_hash.len() } else { 3 };
    for i in 0..upper_index {
        sum += to_hash.as_bytes()[i] as u16;
    }
    return sum;
}
