//! # Rust Byte Pair Encoding (BPE)
//!
//! This is a Rust implementation of Byte Pair Encoding (BPE), a simple and effective data compression technique that has also found use in natural language processing (NLP) applications.
//!
//! ## What is Byte Pair Encoding?
//!
//! Byte Pair Encoding (BPE) is a form of data compression in which the most common pair of contiguous bytes of data in a sequence are replaced with a byte that does not occur within the sequence. A lookup table of the replacements is required to rebuild the original data. BPE can also be used for tokenisation of text in a given language to produce a variable sequence of terms from a fixed-size vocabulary of tokens.
//!
//! ## Table of Contents
//!
//! - [Installation](#installation)
//! - [Usage](#usage)
//! - [License](#license)
//!
//! ## Installation
//!
//! To use this Rust BPE library, you can simply add it to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! rust-bpe = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use rust_bpe::BPE;
//! 
//! let mut bpe = BPE::new(); 
//! ```
//!
//! ### Compression
//!
//! To compress something using BPE, first build the vocabulary.
//!
//! ```rust
//! let learn_file = File::open("input.txt").unwrap();
//! let mut buf_reader = BufReader::new(learn_file);
//! let mut data = String::new();
//! buf_reader.read_to_string(&mut data).unwrap();
//! let vocabulary = bpe.build(&file);
//! ```
//!
//! Then, use the encode method to encode the file into tokens:
//!
//! ```rust
//! let input_file = File::open("input.txt").unwrap();
//! let tokens = bpe.encode(&input_file);
//! ```
//!
//! The resulting `tokens` variable will contain the compressed representation of the input file.
//!
//! ### Decompression
//!
//! To decompress the compressed file back into the original text, use the decode method:
//!
//! ```rust
//! let decoded = bpe.decode(&tokens);
//! ```
//!
//! The decoded variable will now contain the original text.
//!
//! ## License
//!
//! This Rust BPE library is licensed under the [MIT License](https://opensource.org/licenses/MIT). Feel free to use, modify, and distribute it as you like.



pub struct BPE;

pub struct Vocabulary;

impl BPE {
    pub fn new() -> BPE {
        BPE
    }

    pub fn build(&self, data: &String) -> Vocabulary {
        Vocabulary
    }

    pub fn encode(&self, )
}