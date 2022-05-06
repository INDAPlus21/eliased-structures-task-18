use std::io::{self, BufRead};
// use std::io::stdin;

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

fn radio(transmitted_length: usize, transmitted: String) {
    // println!("{}", transmitted);

    // let mut start_index = 0;
    let mut tried = vec![];
    for mut i in 1..transmitted_length + 1 {
        let first_try = &transmitted.as_bytes()[0..i];
        // println!("{:?}", first_try);
        let clone = first_try.clone();
        // println!("{:?}", first_try);
        // let vec_try = transmitted.as_bytes()[0..i].to_vec();
        // let vec_try = &first_try.clone().to_vec();
        if !tried.contains(&clone) {
            let mut worked = true;
            let mut k = 0;
            for j in i..transmitted_length {
                if k == first_try.len() {
                    k = 0;
                }
                // println!("transmitted: {:?} {:?}", transmitted, first_try);
                if transmitted.as_bytes()[j as usize] != first_try[k] {
                    worked = false;
                    // tried.push(clone);
                    break;
                }
                k += 1
            }

            // println!("{:?}", first_try);
            if worked {
                println!("{}", i);
                return;
            }
            /* if i > 100 {
                start_index += 1;
                i = 1
            } */
        }
    }
}

/* fn radio(transmitted_length: usize, transmitted: String) {
    // println!("{}", transmitted);

    // let mut start_index = 0;
    let mut tried = vec![];
    for mut i in 1..transmitted_length + 1 {
        let first_try = &transmitted.as_bytes()[0..i];
        // println!("{:?}", first_try);
        let clone = first_try.clone();
        // println!("{:?}", first_try);
        // let vec_try = transmitted.as_bytes()[0..i].to_vec();
        // let vec_try = &first_try.clone().to_vec();
        if !tried.contains(&clone) {
            let mut worked = true;
            let mut k = 0;
            for j in i..transmitted_length {
                if k == first_try.len() {
                    k = 0;
                }
                // println!("transmitted: {:?} {:?}", transmitted, first_try);
                if transmitted.as_bytes()[j as usize] != first_try[k] {
                    worked = false;
                    // tried.push(clone);
                    break;
                }
                k += 1
            }

            // println!("{:?}", first_try);
            if worked {
                println!("{}", i);
                return;
            }
            /* if i > 100 {
                start_index += 1;
                i = 1
            } */
        }
    }
} */

fn main() {
    // radio(8, "cabcabca".to_string());
    // radio(10, "beanibeani".to_string());
    // radio("hellorustacian".len(), "hellorustacian".to_string());
    // okay... if we've already tried "c", "cab"... let's start with at least 3?
    // "c", "ca", "cab", check if it's the same thing

    let stdin = io::stdin();

    let mut iterator = stdin.lock().lines();
    let transmitted_length = iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .expect("int"); // as i8;
    let transmitted = iterator.next().unwrap().unwrap();

    radio(transmitted_length, transmitted);
}
