use std::io;
use std::io::prelude::*;
// use std::rand::{self, Rng};
use std::time::{Duration, SystemTime};

/*fn old_check(patterns: Vec<String>, text: &[u8]) {
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
}*/

/*fn native_impl(patterns: Vec<String>, text: &str) {
    for pattern in patterns.iter() {
        /*let pattern_bytes = pattern.as_bytes();
        let mut occurence_position = "".to_string(); // vec![];
        let upper_index = text.len() - pattern.len() + 1;
        let pattern_length = pattern.len();

        for i in 0..upper_index {
            // eprintln!("{:?} {:?}", text.as_bytes()[i] as char, i);
            // let mut true_all = true;

            if *pattern_bytes == text[i..i+pattern_length] {
                occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            }

            /*for j in 0..pattern_length {
                // eprintln!("{:?} {:?}", text.as_bytes()[i+j] as char, pattern.as_bytes()[j] as char);
                if text[i + j] != pattern_bytes[j] {
                    true_all = false;
                    // eprintln!("false")
                }
            }*/

            // if true_all {
            //     occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            // }
        }*/

        let mut occurence_position = "".to_string(); // vec![];

        let position = text.to_string().find(pattern); // .unwrap(); 

        if position != None {
            occurence_position += &(position.unwrap().to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            // println!("{}", position.unwrap());
        }

        println!("{}", occurence_position);

        // let vec_to_string : String = occurence_position.iter().map( |&id| id.to_string() + " ").collect();
        // println!("pattern: {}", pattern);
    }
}*/

/* fn knueth(patterns: Vec<String>, text: &str) {
    for pattern in patterns.iter() {
        /*let pattern_bytes = pattern.as_bytes();
        let mut occurence_position = "".to_string(); // vec![];
        let upper_index = text.len() - pattern.len() + 1;
        let pattern_length = pattern.len();

        for i in 0..upper_index {
            // eprintln!("{:?} {:?}", text.as_bytes()[i] as char, i);
            // let mut true_all = true;

            if *pattern_bytes == text[i..i+pattern_length] {
                occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            }

            /*for j in 0..pattern_length {
                // eprintln!("{:?} {:?}", text.as_bytes()[i+j] as char, pattern.as_bytes()[j] as char);
                if text[i + j] != pattern_bytes[j] {
                    true_all = false;
                    // eprintln!("false")
                }
            }*/

            // if true_all {
            //     occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            // }
        }*/

        let mut previous_longest_suffix = 0; 
        let mut longest_previous_suffix_values = []; 

        let i = 1; 
        
        /* while (i < pattern.length) {
            if (pattern[i] == pattern[pattern.length])
        } */

        // let vec_to_string : String = occurence_position.iter().map( |&id| id.to_string() + " ").collect();
        // println!("pattern: {}", pattern);
    }
} */


fn benchmark() {
    let mut text: &[u8];
    let mut patterns: Vec<String> = vec![];

    let now = SystemTime::now();

    println!("{:?}", now.elapsed().unwrap().as_nanos()); 

    patterns = vec!["aa".to_string(), "ba".to_string()];

    let alphabet = "abcdefghijklmnopqrstuvwxzåäö"; 

    /* for _i in 0..100 {
        let random_time: u8 = now.elapsed().unwrap().subsec_nanos() as u8; 
        println!("{:?}", random_time); 
        let random_character = random_time as char; // num of ascii chars 
        println!("rand char: {}", random_character); 
        patterns.push(random_character.to_string())
    } */

    /* for _ in 0..10 {
        for letter in alphabet.chars() {
            patterns.push(letter.to_string()); 
        }
    } */

    patterns.push("abcde".to_string());

    // text = ""; 

    let mut text_to_convert = "".to_string(); 

    for _ in 0..1000 {
        for letter in alphabet.chars() {
            text_to_convert = format!("{}{}", text_to_convert, letter); // [text_to_convert, letter].join("");
            // text_to_convert += letter;  
        }
    }

    text = text_to_convert.as_bytes(); 

    println!("{:?}", patterns); 
    println!("{:?}", text); 

    // text = "abaaaa".as_bytes();

    let check_execution = SystemTime::now();
    // old_check(patterns, text); 
    check_pattern(patterns, text);
    println!("ms: {:?}", check_execution.elapsed().unwrap().as_millis())
}

fn check_pattern(patterns: Vec<String>, text: &[u8]) {
    for pattern in patterns.iter() {
        let pattern_bytes = pattern.as_bytes();
        // let mut occurence_position = "".to_string(); // vec![];
        let upper_index = text.len() - pattern.len() + 1;
        let pattern_length = pattern.len();
        // let mut occurence_position = vec![]; 

        for i in 0..upper_index {
            // eprintln!("{:?} {:?}", text.as_bytes()[i] as char, i);
            // let mut true_all = true;

            if *pattern_bytes == text[i..i+pattern_length] {
                print!("{} ", i); 
                // occurence_position.push(i); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            } 

            // abc
            // aabcaaabca

            /* for j in 0..pattern_length {
                // eprintln!("{:?} {:?}", text.as_bytes()[i+j] as char, pattern.as_bytes()[j] as char);
                if text[i + j] != pattern_bytes[j] {
                    // i += j; 
                    true_all = false;
                    // print!("{} ", i); 
                    // eprintln!("false")
                }
            }

            if true_all {
                print!("{} ", i); 
                // occurence_position += &(i.to_string() + " "); // concat!(occurence_position, i.to_string(), " ");  //i+" "; // .push(i)
            } */
        }

        // let vec_to_string : String = occurence_position.iter().map( |&id| id.to_string() + " ").collect();
        // println!("pattern: {}", pattern);
        println!(); 
        // println!("{:?}", occurence_position);
    }
}

fn main() {

    // benchmark()


    let input = io::stdin();

    let mut num_patterns = 0;
    let mut input_iter = 0;
    let mut text: &[u8];
    let mut patterns: Vec<String> = vec![];


    for line in input.lock().lines().map(|line| line.unwrap()) {
        // eprintln!("{:?} iter {:?}", _line, input_iter);
        if input_iter == 0 {
            num_patterns = line.trim().parse().expect("int"); // _line.parse::<i32>().unwrap();
            // eprintln!("number of patterns: {:?}", num_patterns);
        } else if input_iter == num_patterns+1 {
            // eprintln!("patterns: {:?}", patterns);
            // eprintln!("text: {:?}", line);
            text = &line.as_bytes(); // &line; 

            check_pattern(patterns, text);
            // native_impl(patterns, text); 

            input_iter = -1;
            patterns = vec![];
        } else {
            patterns.push(line)
        }

        input_iter += 1;
    }
}
