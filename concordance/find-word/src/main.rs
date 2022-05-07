use std::fs;
// use std::fs::File;
use std::error::Error;
// use std::io::prelude::*;
use std::io::{self, prelude::*};
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    // eg all these have the same hash
    // println!("{:?}", hash("ale"));
    // println!("{:?}", hash("adm"));
    // println!("{:?}", hash("apa"));

    // let mut magic_file = File::open("../magic-file.txt");

    // let mut index_file = File::open("../index-file.txt")?;

    let magic_contents: String = fs::read_to_string("../magic-file.txt")?.parse()?;

    let string_index_file: String = fs::read_to_string("../index-file.txt")?.parse()?;

    // let byte_index_file = fs::read("../index-file.txt")?;

    // let string_korpus_file: String = fs::read_to_string("../korpus.txt")?.parse()?;

    let byte_korpus = fs::read("../korpus")?;

    let input = io::stdin();

    for mut input_word in input.lock().lines().map(|line| line.unwrap()) {
        let start = Instant::now();

        input_word = input_word.to_lowercase();

        // could convert the magic file to a hash map, alternatively sort it
        let hash = hash(&input_word);
        // println!("{}", hash);

        let input_byte_length = input_word.as_bytes().len();

        // a's last byte index is 9962292, second to last is 9959364

        'outer: for line in magic_contents.split("\n") {
            if line != "" {
                let split = line.split(" ").collect::<Vec<_>>();

                // println!("{:?}", split);

                let magic_hash: u16 = split[0].parse().unwrap();
                let byte_index: usize = split[1].parse().unwrap();

                // println!("{}", magic_hash);

                if hash == magic_hash {
                    let quick_word =
                        &string_index_file.as_bytes()[byte_index..byte_index + input_byte_length];

                    if quick_word == input_word.as_bytes() {
                        let indexes = &string_index_file.as_bytes()
                            [byte_index + input_byte_length..byte_index + 100000];

                        // println!("indexes: {:?}", indexes);

                        let indexes_string = String::from_utf8_lossy(indexes);

                        let mut indexes_array: Vec<&str> = indexes_string.split(" ").collect(); //::<Vec&str>>();

                        let matched_word = indexes_array[0];
                        // println!("matched: {:?}", matched_word);

                        // println!("indexes string: {:?}", indexes_string);
                        let mut real_indexes_array = vec![];

                        let mut index_array_iter = 0;
                        // println!("all {:?}", &indexes_array);

                        for index in &indexes_array {
                            if index.contains("\n") {
                                // println!("contains n {:?}", indexes_array[index_array_iter]);

                                for (i, c) in indexes_array[index_array_iter].chars().enumerate() {
                                    if c == '\n' {
                                        indexes_array[index_array_iter] =
                                            &indexes_array[index_array_iter][..i as usize]
                                    }
                                }

                                real_indexes_array =
                                    indexes_array[1..index_array_iter + 1].to_vec();
                                break;
                            }
                            index_array_iter += 1;
                        }
                        // println!("{:?}", indexes_array);

                        // println!("real {:?}", &real_indexes_array);

                        let times = real_indexes_array.len();
                        let gång_er = if times > 1 { "gånger" } else { "gång" };

                        println!(
                            "\"{}\" förekommer {} {} i korpus",
                            input_word, times, gång_er
                        );

                        for (korpus_iter, index) in real_indexes_array.iter().enumerate() {
                            let number_index = index.parse::<usize>().unwrap();
                            // println!("number index {:?}", number_index);

                            let korpus_line = &byte_korpus[number_index - 50..number_index + 50];
                            let korpus_string = String::from_utf8_lossy(korpus_line);
                            print!("...");
                            for c in korpus_string.chars() {
                                if c != '\n' {
                                    print!("{}", c);
                                } else {
                                    print!(" ");
                                }
                            }

                            println!("...");
                            if korpus_iter > 25 {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        println!("Tog: {:.2?}", start.elapsed());
    }

    return Ok(());
}

fn hash(to_hash: &str) -> u16 {
    let mut sum = 0;
    let upper_index = if to_hash.len() <= 3 { to_hash.len() } else { 3 };
    for i in 0..upper_index {
        sum += to_hash.as_bytes()[i] as u16;
    }
    return sum;
}
