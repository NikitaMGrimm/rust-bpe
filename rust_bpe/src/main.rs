use rust_bpe::{Vocabulary, print_top_n_tokens};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./bigdata.txt";
    let data = file_opener(path);

    let mut vocab = Vocabulary::new();

    let serialize = true;
    let deserialize = !&serialize;

    let learn = serialize;

    let merges = 100000;
    let replacements = 1;
    let cutoff = 1;

    if learn {
        let encoded = vocab.learn(&data, merges, replacements, cutoff);
    }
    // println!("Encoded: {:?}, len: {}", encoded, encoded.len());

    // let mut s = String::new();
    // vocab.decode(&encoded, &mut s);
    // println!("Decoded: {:?}, len: {}", s, s.len());

    if serialize {
        let encoded: Vec<u8> = bincode::serialize(&vocab).unwrap();
        // save to file called "vocabulary.bincode"
        std::fs::write("vocabulary.bincode", encoded)?;
    }
    // now read it back in
    if deserialize {
        let encoded = std::fs::read("./vocabulary/vocabulary.bincode")?;
        let mut vocab: Vocabulary = bincode::deserialize(&encoded).unwrap();
        // now use it
        println!("Vocabulary size: {}", vocab.len());
        print_top_n_tokens(&mut vocab, 100);
    }

    Ok(())
}

fn file_opener(path: &str) -> String {
    let path = Path::new(path);
    let file = File::open(&path).expect("This should open the file.");
    let mut reader = BufReader::new(file);

    let mut input_data = String::new();
    reader
        .read_to_string(&mut input_data)
        .expect("This should read the file.");
    input_data
}
