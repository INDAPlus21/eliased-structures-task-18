// use std::hash::Hash;
use std::hash::Hash;
use std::io;
use std::io::prelude::*;

/* fn getHash(key: &String) -> usize {
    let mut hash = 0;
    for char in key.chars() {
        hash += char as usize;
        println!("char {}", char as u32);
    }

    println!("before modulo {:?}", hash);

    return hash % 100;
} */

/* fn getHash(key: &[u8]) -> u16 {
    let mut hash: u16 = 0;
    hash += key[0] as u16;
    if key.len() > 3 {
        hash += key[2] as u16
    }
    /* for number in key {
        hash += *number as u16;
        // println!("hash {} {:?}", hash, key);
    } */

    // overflow because u8 is 255 max value

    return hash % 100;
} */

/* fn hash_check_patterns(patterns: Vec<String>, text: String) {
    for pattern in patterns.iter() {
        let pattern_hash = // getHash(pattern.as_bytes());

        let pattern_length = pattern.as_bytes().len();
        let upper_index = text.len() - pattern_length + 1; //+ 1;
        for i in 0..upper_index {
            // println!("i {}", i);
            let text_hash = std::hash::Hash(&text.as_bytes()[i..i + pattern_length]); // getHash(&text.as_bytes()[i..i + pattern_length]); // .as_bytes()[i..i + pattern_length]
                                                                              // let text_bytes = &text.as_bytes()[i..i + pattern_length];

            // println!("{:?} {:?}", pattern.as_bytes(), &text.as_bytes()[i..i + pattern_length]);
            if pattern_hash == text_hash {
                if *pattern.as_bytes() == text.as_bytes()[i..i+pattern_length] {
                    print!("{} ", i);
                }
            }

            /* if *pattern_bytes == text[i..i + pattern_length] {
                print!("{} ", i);
            } */
        }
        println!()
    }
} */

fn kmp(pattern: String, text: String) {
    let lps_array = create_lps(&pattern); 
    let pattern_length = pattern.len(); 

    // let mut matching = []; 
    let mut j = 0;  

    for (i, c) in text.chars().enumerate() {
        while j > 0 && pattern.as_bytes()[j] as char != c {
            j = lps_array[j-1]
        }

        if pattern.as_bytes()[j] as char == c {
            if j == pattern_length - 1 {
                print!("{} ", i-j); 
                j = lps_array[j]; 
            } else {
                j += 1; 
            }
        }
    }

    println!(); 
} 

fn create_lps(pattern: &String) -> Vec<usize> {
    let mut lps_array = vec![0; pattern.len()];

    let mut j = 0;

    // "phase 1" IS the for loop
    for i in 1..pattern.len() {
        while j > 0 && pattern.as_bytes()[i] != pattern.as_bytes()[j] {
            j = lps_array[j-1]; 
            // j += 1;
        }

        if pattern.as_bytes()[i] == pattern.as_bytes()[j] {
            // lps_array[j] = i + 1; 
            // i += 1; 
            j += 1;  
        }

        lps_array[i] = j; 
    }

    // println!("{:?}", lps_array); 

    return lps_array;
}

fn main() {
    // create_lps("ABCDABD".to_string()); 
    // kmp("p".to_string(), "Popup".to_string());
    // kmp("pup".to_string(), "Popup".to_string());

    let input = io::stdin();

    let mut num_patterns = 0;
    let mut input_iter = 0;
    let mut patterns: Vec<String> = vec![];

    for line in input.lock().lines().map(|line| line.unwrap()) {
        if input_iter == 0 {
            num_patterns = line.trim().parse().expect("int");
        } else if input_iter == num_patterns + 1 {
            let text = line; // std::str::from_utf8(line).unwrap().to_string(); 
            //.as_bytes();

            // check_pattern(patterns, text);
            for pattern in patterns {
                kmp(pattern, text.clone());
            }

            // println!("i'm here");

            input_iter = -1;
            patterns = vec![];
        } else {
            patterns.push(line)
        }

        input_iter += 1;
    }
}
