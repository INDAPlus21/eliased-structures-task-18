use std::io;
use std::io::prelude::*;

fn main() {
    // let text = "abcd";

    // for c in text.chars() {
    //     if
    //     println!("{}", c);
    // }

    // println!("hello world");

    let input = io::stdin();

    for line in input.lock().lines().map(|_line| _line.unwrap()) {
        if line != "." {
            let text = line.to_string(); // "abcd".to_string();

            for first_n_chars_text in 1..(text.len() + 1) {
                // println!("hello world");
                let mut final_text = "".to_string();
                // text.as_bytes[i].len();
                // if text.as_bytes[i].len()

                //
                let times_to_multiply =
                    (text.len() as f64 / (first_n_chars_text) as f64).floor() as i32;
                for _j in 0..times_to_multiply {
                    // println!("hello world");
                    for k in 0..first_n_chars_text {
                        // println!("hello world");
                        final_text = format!("{}{}", final_text, text.as_bytes()[k] as char);
                        if final_text != text[..final_text.len()] {
                            break;
                        }
                    }
                    if final_text != text[..final_text.len()] {
                        break;
                    }
                    // final_text = format!("{}{}", final_text, text.as_bytes()[i] as char);
                    // concat!(final_text, text.as_bytes()[i].to_string());
                }

                eprintln!("{}", final_text);

                if final_text == text {
                    println!("{}", text.len() / first_n_chars_text);
                    // power = first_n_chars_text
                    break;
                }
            }
        }
    }
    // println!("{}", power);
}
