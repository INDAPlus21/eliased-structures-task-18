use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
// use positioned_io::WriteAt;
use std::os::unix::fs::FileExt;

/* Notes (i princip alla anteckningar är kopierade från andra källor)
it's this concordance that is referred to: https://www.wikiwand.com/en/Concordance_(publishing)

"A concordance is an alphabetical list of the principal words used in a book or body of work, listing every instance of each word with its immediate context."

"A concordance is more than an index, with additional material such as commentary, definitions and topical cross-indexing which makes producing one a labor-intensive process even when assisted by computers."

But we're not going to use additional material "such as commentary, definitions", nor topical concordance which is based on themes

https://help.olivetree.com/hc/en-us/articles/213709006-What-s-the-difference-between-a-Concordance-and-a-Cross-Reference-
"A cross reference is a verse that has a similar theme or topic as the verse that you are reading."

"A concordance offers more precise lookups on specific words than a cross reference and shows you where those words appear throughout Scripture. "

Teorifråga: "Bör indexinformationen lagras tillsammans med själva ordet eller på ett separat ställe?"

Latmanshashing = "Hasha bara på de tre första bokstäverna i söknyckeln. Använd sedan
binärsökning". Lämpligt för sökning med få diskaccesser i stor text nar indexet inte
kan ligga i primärminne. Exempel indexera lexikon
användbart: file:///home/elias/Downloads/F03-1.pdf slide 16 och 17

"However, if we expand the array by a constant proportion, e.g. by doubling its size, the total time to insert n elements will be O(n), and we say that each insertion takes constant amortized time." Is this the same thing as preallocating?

http://burtleburtle.net/bob/hash/doobs.html
"A good hash function distributes hash values uniformly. If you don't know the keys before choosing the function, the best you can do is map an equal number of possible keys to each hash value. If keys were distributed uniformly, an excellent hash would be to choose the first few bytes of the key and use that as the hash value. Unfortunately, real keys aren't uniformly distributed. Choosing the first few bytes works quite poorly in practice.

The real requirement then is that a good hash function should distribute hash values uniformly for the keys that users actually use."

"En konkordans är en databas där man kan slå upp ord och då få se alla förekomster av ordet tillsammans med orden närmast före och närmast efter i texten. Detta är ett stort hjälpmedel för lingvister som vill undersöka hur olika ord används i språket."




*/

fn hash(to_hash: &str) -> u16 {
    // println!("{:?}", to_hash.as_bytes()[0]);
    let mut sum = 0;
    let upper_index = if to_hash.len() <= 3 { to_hash.len() } else { 3 };
    // println!("{:?} {:?}", upper_index, to_hash.len());
    for i in 0..upper_index {
        sum += to_hash.as_bytes()[i] as u16;
    }
    // let mut sum = to_hash.as_bytes()[0] as u16+to_hash.as_bytes()[1] as u16 +to_hash.as_bytes()[2] as u16;
    return sum;
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let mut token = File::open("token.txt");

    // index file is just token.txt without duplicate words
    // the large text file isn't needed, it's only token that is actually needed
    // what??? good instructions have been here all along???
    // This file is simple so it can be skipped as long as you account for the change in your MagicFile (Having this file will be easier though!) All you want for this file is to not have duplicate words to make searching easier! The above words will instead look like this
    // a 10000368 10017347 10047993 10058693 10067117 [...] \n
    let mut index_file = File::create("index-file.txt")?;
    let mut magic_file = File::create("magic-file.txt")?;

    /*if let Ok(lines) = read_lines("./hosts") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }

    file.write_all(b"Hello, world!")?;
    Ok(())*/

    // let mut contents = String::new();
    // token.read_to_string(&mut contents);

    let contents: String = fs::read_to_string("token.txt")?.parse()?;
    let content_vec = contents.split("\n"); //.collect();
    let mut previous_word = ""; // &str;
    // index_file.write("a".as_bytes());
    let mut previous_hash = 0; // &str;

    // println!("{:?}", hash("hello"));

    for line in contents.split("\n") {
        let temp_vec = line.split(" ").collect::<Vec<_>>();
        let word = temp_vec[0]; //
        let byte_index = temp_vec[1];

        // println!("{:?}", word); // första ordet är a, andra är byte indexen
        // println!("{:?}", byte_index); // första ordet är a, andra är byte indexen

        if previous_word != word {
            index_file.write("\n".as_bytes());
            index_file.write(word.as_bytes());
            previous_word = word;
            // println!("{:?}", word); // första ordet är a, andra är byte indexen

            let hash = hash(word);

            if hash != previous_hash {
                for i in previous_hash + 1..hash {
                    // bytearray := []byte(strconv.FormatInt(stat.Size(), 10))
                    let metadata = fs::metadata("index-file.txt")?;
                    let size = metadata.len();

                    // let buffer = vec![0; 10]; // size.as_bytes(); //common_index_words[i];
                                              // write at the byte index calculated with hash
                    // magic_file.write(size as &[u8]);
                }
            }

            previous_hash = hash;

            /* index_file.write_at(
                i * 10,
                byte_index.parse::<u64>().unwrap(), /* .to_owned().as_bytes()[0] as u64  *//*&[u8]*/
            ); */
        }

        index_file.write(" ".as_bytes())?;
        index_file.write(byte_index.as_bytes())?;
    }

    // let metadata = fs::metadata("index-file.txt")?;
    // let size = metadata.size_of();

    // println!("{}", contents);

    Ok(())

    // assert_eq!(contents, "Hello, world!");
    // Ok(())
}
