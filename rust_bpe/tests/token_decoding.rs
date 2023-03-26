use std::{io::{BufReader, Read}, fs::File};

use rust_bpe::Vocabulary;

// Try to encode the sample txt files in the test/sample folder
#[test]
fn test_token_decoding() {
    for i in 1..=4 {
        let path = format!("./tests/sample/sample_{}.txt", i);
        let file = File::open(&path).expect("This should open the file.");
        let mut reader = BufReader::new(file);

        let mut input_data = String::new();
        reader.read_to_string(&mut input_data).expect("This should read the file.");

        let mut vocab = Vocabulary::new();
        let encoded_file = vocab.learn(&input_data, 1000);
        println!("{:?}", encoded_file);
    }
}

#[test]
fn test_token_decoding2() {
    let i = 5;
    let path = format!("./tests/sample/sample_{}.txt", i);
    let file = File::open(&path).expect("This should open the file.");
    let mut reader = BufReader::new(file);

    let mut input_data = String::new();
    reader.read_to_string(&mut input_data).expect("This should read the file.");

    let mut vocab = Vocabulary::new();
    let final_encoding = vocab.learn(&input_data, 10);
    assert_eq!(final_encoding.len(), 5);
    
    let mut s = String::new();
    vocab.decode(&final_encoding, &mut s);
    assert_eq!("aaabdaaabac" , s);
}
