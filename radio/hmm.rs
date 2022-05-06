use std::io::{self, BufRead};
// use std::io::stdin;

fn radio(transmitted_length: usize, transmitted: String) {
    // println!("{}", transmitted);

    // let mut start_index = 0;
    let mut tried = vec![];
    for mut i in 0..transmitted_length + 1 {
        let first_try = transmitted.as_bytes()[0..i];
        // let vec_try = &first_try.to_vec()
        // if !tried.contains(first_try) {
            // tried.push(first_try);
            let mut worked = true;
            let mut k = 0;
            for j in i..transmitted_length {
                if k == first_try.len() {
                    k = 0;
                }
                // println!("transmitted: {:?} {:?}", transmitted, first_try); 
                if transmitted.as_bytes()[j as usize] != first_try[k] {
                    worked = false;
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
        // }
    }
}

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
