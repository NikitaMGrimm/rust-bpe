use std::{fs::File, io::{BufReader, Read}};

use rust_bpe::BPE;

fn main() {

    // Make new BPE object.
    let mut bpe = BPE::new();

    // Read training file for vocabulary.
    let file_path = "../input.txt";
    let learn_file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(learn_file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data).unwrap();

    // Build vocabulary.
    let vocabulary = bpe.build(&data);
    // Optional: 

    // Read file that needs to be encoded
    let file_path = "../input.txt";
    let input_file = File::open(file_path).unwrap();
    let mut buf_reader = BufReader::new(input_file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data).unwrap();

    // Encode the data
    let tokens = bpe.encode(&data, );

    let decoded = bpe.decode(&tokens);
}
