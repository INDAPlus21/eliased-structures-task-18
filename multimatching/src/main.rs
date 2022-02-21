use std::io;
// use std::slice::Join;
use std::io::prelude::*;
// use str::parse::<T>();
// use std::str::pattern;
// faster if only ascii: https://www.reddit.com/r/rust/comments/apan2c/what_is_the_best_way_to_index_into_strings/ 

// fn index_str(s: str) -> str {
//     return s.chars().nth(x).unwrap();
// }

// fn index()

fn main() {

    // println!("{:?}", "abaaa".find("ba"), Some(1)); 

    // Let's first implement the naive algorithm, then: 
    // https://www.geeksforgeeks.org/boyer-moore-algorithm-for-pattern-searching/

    // if error in kattis: https://stackoverflow.com/questions/24542115/how-to-index-a-string-in-rust

    /*let text = "bananananaspaj"; // "abaaa".to_string(); 
    let pattern = "a"; // "ba".to_string(); 
    println!("{:?}", pattern.len()); 
    println!("{:?}", text.len()); 
    // println!("{:?}", index_str(text, i))
    // println!("{:?}", text.as_bytes()[1] as char);

    let mut match_string = "".to_string();  
    let mut occurence_position = vec![]; 

    for i in 0..text.len()-pattern.len()+1 {
        println!("{:?} {:?}", text.as_bytes()[i] as char, i); 
        let mut true_all = true; 

        for j in 0..pattern.len() {
            println!("{:?} {:?}", text.as_bytes()[i+j] as char, pattern.as_bytes()[j] as char); 
            if text.as_bytes()[i+j] != pattern.as_bytes()[j] {
                true_all = false;
                println!("false")
            }
        }

        if true_all {
            occurence_position.push(i)
        }
    }*/

    // println!("occurence position: {:?}", occurence_position);

    let input = io::stdin();

    let mut num_patterns = 0; 
    let mut input_iter = 0; 
    let mut text : &[u8]; 
    let mut patterns : Vec<String> = vec![]; 

    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        // eprintln!("{:?} iter {:?}", _line, input_iter);
        if input_iter == 0 {
            num_patterns = _line.trim().parse().expect("int"); // _line.parse::<i32>().unwrap();
            // eprintln!("num {:?}", num_patterns);
        } else if input_iter == num_patterns+1 {
            // eprintln!("patterns {:?}", patterns);
            // eprintln!("line {:?}", _line);
            text = &_line.as_bytes(); 

            for pattern in patterns.iter() {
                let pattern_bytes = pattern.as_bytes();
                let mut occurence_position = "".to_string(); // vec![]; 
                let upper_index = text.len()-pattern.len()+1;
                let pattern_length = pattern.len(); 

                for i in 0..upper_index {
                    // eprintln!("{:?} {:?}", text.as_bytes()[i] as char, i); 
                    let mut true_all = true; 
            
                    for j in 0..pattern_length {
                        // eprintln!("{:?} {:?}", text.as_bytes()[i+j] as char, pattern.as_bytes()[j] as char); 
                        if text[i+j] != pattern_bytes[j] {
                            true_all = false;
                            // eprintln!("false")
                        }
                    }
            
                    if true_all {
                        occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
                    }
                }
    
                // let vec_to_string : String = occurence_position.iter().map( |&id| id.to_string() + " ").collect(); 
                println!("{}", occurence_position); 
            }

            input_iter = -1; 
            patterns = vec![];         
        } else {
            patterns.push(_line)
        } 

        input_iter += 1; 
    }
}
