use std::io;
use std::io::prelude::*;
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

    let text = "bananananaspaj"; // "abaaa".to_string(); 
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
                // match_string = [match_string, (text.as_bytes()[i+j] as char).to_string()].join("");
                // println!("found match!"); 
            }
            /* if match_string.len() == pattern.len() {
                occurence_position.push(i)
                // break;
            } */
        }

        if true_all {
            occurence_position.push(i)
        }
        // if text[i]+text[i+1]
        // println!("{:?}", c);
    }

    // println!("match: {:?}", match_string);
    println!("occurence position: {:?}", occurence_position);

    /*for c in text.chars() {
        println!("{:?}", c);
    }*/

    // println!("{:?}", "abaaa".find("ba"), Some(1)); 

    /*let input = io::stdin();

    let mut seq: Vec<Vec<usize>> = vec![]; // = Vec::with_capacity(commands[i] as usize);

    let mut line_iter = 0;
    let mut length_of_commands = 0; 

    let mut num_patterns = 0; 

    for _line in input.lock().lines().map(|_line| _line.unwrap()) {
        let commands: Vec<usize> = _line
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

        println!("{:?}", commands)

        "abaaa".find("ba"), Some(1)
    }*/
}
