use std::fs;
use std::fs::File;
// use std::io::{self, BufRead};
use std::cmp;
use std::error::Error;
use std::io::prelude::*;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let mut magic_file = File::open("../magic-file.txt");

    let mut index_file = File::open("../index-file.txt")?;

    let magic_contents: String = fs::read_to_string("../magic-file.txt")?.parse()?;

    let string_index_file: String = fs::read_to_string("../index-file.txt")?.parse()?;

    let byte_index_file = fs::read("../index-file.txt")?;

    // let string_korpus_file: String = fs::read_to_string("../korpus.txt")?.parse()?;

    let byte_korpus = fs::read("../korpus")?;

    let input = io::stdin();

    for mut input_word in input.lock().lines().map(|line| line.unwrap()) {
        input_word = input_word.to_lowercase();
        println!("{}", input_word);

        // could convert the magic file to a hash map, alternatively sort it 
        let hash = hash(&input_word);
        println!("{}", hash);

        // a last is 9962292, second to last 9959364

        'outer: for line in magic_contents.split("\n") {
            if line != "" {
                let mut split = line.split(" ").collect::<Vec<_>>();
    
                // println!("{:?}", split);
    
                let magic_hash: u16 = split[0].parse().unwrap();
                let byte_index: usize = split[1].parse().unwrap();
    
                // println!("{}", magic_hash);
        
                if hash == magic_hash {
                    let indexes = &string_index_file.as_bytes()[byte_index..byte_index+100000];

                    // println!("indexes: {:?}", indexes);

                    let indexes_string =  String::from_utf8_lossy(indexes); 

                    println!("indexes string: {:?}", indexes_string);

                    let mut indexes_array: Vec<&str> = indexes_string.split(" ").collect(); //::<Vec&str>>(); 
                    
                    let matched_word = indexes_array[0]; 
                    println!("matched: {:?}", matched_word); 

                    if (matched_word == input_word) {
                        let mut real_indexes_array = vec![]; 
        
                        let mut index_array_iter = 0; 
                        // println!("all {:?}", &indexes_array); 
        
                        for index in &indexes_array {
                            if index.contains("\n") {
                                println!("contains n {:?}", indexes_array[index_array_iter]);
        
                                /* use regex::Regex;
                                let re = Regex::new(r"\n.+").unwrap();
                                let result = re.replace_all(indexes_array[index_array_iter], "");
                                println!("{}", result); // => "xxxxx xxxxx!" */
        
                                for (i, c) in indexes_array[index_array_iter].chars().enumerate() {
                                    if c == '\n' {
                                        indexes_array[index_array_iter] = &indexes_array[index_array_iter][..i as usize]
                                    }
                                }
                            
                                
                                real_indexes_array = indexes_array[1..index_array_iter+1].to_vec(); 
        
                                // r.str.to_owned() is a temporary value. You can take a reference to a temporary, but because the temporary value will usually be dropped (destroyed) at the end of the innermost enclosing statement, the reference becomes dangling at that point. 
                                // let long_lived = result.unwrap(); 
                                // real_indexes_array[index_array_iter] = &long_lived.to_owned(); // to_string(); // indexes_array[index_array_iter]
                                break; 
                            }
                            index_array_iter += 1; 
                        }
                        // println!("{:?}", indexes_array); 
        
        
        
                        println!("real {:?}", &real_indexes_array); 
                        
                        println!("Ordet förekommer {:?} gånger i korpus", real_indexes_array.len()); 
                        let mut korpus_iter = 0; 
                        for index in real_indexes_array {
                            let number_index = index.parse::<usize>().unwrap();
                            // println!("number index {:?}", number_index); 
        
                            let korpus_line = &byte_korpus[number_index-50..number_index+50];
                            let korpus_string = String::from_utf8_lossy(korpus_line); 
                            print!("..."); 
                            for c in korpus_string.chars() {
                                if c != '\n' {
                                    print!("{}", c); 
                                }
                            }
                            // println!("rad: {}", korpus_string); 
                            korpus_iter += 1; 
        
                            println!("..."); 
                            if korpus_iter > 25 {
                                break 'outer;
                            }
        
                        }
                    }
                    
                }
            }
        } 
    }

    return Ok(())
}

fn hash(to_hash: &str) -> u16 {
    let mut sum = 0;
    let upper_index = if to_hash.len() <= 3 { to_hash.len() } else { 3 };
    for i in 0..upper_index {
        sum += to_hash.as_bytes()[i] as u16;
    }
    return sum;
}
