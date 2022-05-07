use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
// use positioned_io::WriteAt;
use std::os::unix::fs::FileExt;
use std::time::Instant;

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

// index file is just token.txt without duplicate words
// the large text file isn't needed, it's only token that is actually needed
// what??? good instructions have been here all along???

*/

// det finns inga ord där de tre första bokstäverna är annorlunda (construed example)
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
    let mut start = Instant::now();

    let mut index_file = File::create("index-file.txt")?;
    let mut magic_file = File::create("magic-file.txt")?;

    let contents: String = fs::read_to_string("token.txt")?.parse()?;
    let mut previous_word = "";

    // println!("{:?}", hash("hello"));

    // To convert the byte slice back into a string slice, use the from_utf8 function.
    // let mut total_bytes = 0;

    let split = contents.split("\n"); 
    let split_length = split.to_owned().count();

    for (i, line) in split.enumerate() {
        let temp_vec = line.split(" ").collect::<Vec<_>>();
        let word = temp_vec[0]; //
        let byte_index = temp_vec[1];

        // println!("{:?}", word); // första ordet är a, andra är byte indexen
        // println!("{:?}", byte_index); // första ordet är a, andra är byte indexen

        if previous_word != word {
            index_file.write("\n".as_bytes());

            let metadata: u32 = index_file.metadata().unwrap().len() as u32;

            index_file.write(word.as_bytes());
            previous_word = word;
            // println!("{:?}", word); // första ordet är a, andra är byte indexen

            let hash = hash(word);

            // println!("{:?}", metadata); // byte index in magic file of the first word

            // and if the hash doesn't match exactly go to the next line in search...
            // &transform_u32_to_array_of_u8(metadata)

            // magic_file.write(&hash.to_ne_bytes())?; // &transform_u32_to_array_of_u8// (metadata));
            magic_file.write(&hash.to_string().as_bytes())?; // &transform_u32_to_array_of_u8// (metadata));
            magic_file.write(" ".as_bytes())?;
            // magic_file.write(&metadata.to_ne_bytes())?; // &transform_u32_to_array_of_u8(metadata));
            magic_file.write(&metadata.to_string().as_bytes())?; // &transform_u32_to_array_of_u8(metadata));
            magic_file.write("\n".as_bytes());
            // magic_file.write();

            // ett alternativ till att kontinuerligt hålla koll på byte length
            /* if hash != previous_hash {
                for i in previous_hash + 1..hash {
                    // bytearray := []byte(strconv.FormatInt(stat.Size(), 10))
                    // let metadata = fs::metadata("index-file.txt")?;

                    let metadata = index_file.metadata().unwrap().len();


                    println!("{:?}", metadata);
                    // let size = metadata.len();

                    // let buffer = vec![0; 10]; // size.as_bytes(); //common_index_words[i];
                                              // write at the byte index calculated with hash
                    // magic_file.write(size as &[u8]);
                }
            } */

            /* index_file.write_at(
                i * 10,
                byte_index.parse::<u64>().unwrap(), /* .to_owned().as_bytes()[0] as u64  *//*&[u8]*/
            ); */
        }

        index_file.write(" ".as_bytes())?;
        index_file.write(byte_index.as_bytes())?;

        if i % 10000 == 0 {
            // tar runt ~200 ms för % 10000
            println!("iteration: {:?} av {}", i, split_length);
            println!("Tog: {:.2?}", start.elapsed());
            start = Instant::now();
        }
    }

    println!("done");

    Ok(())
}
