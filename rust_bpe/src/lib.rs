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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type TknId = u32;
pub type TknDiagram = (TknId, TknId);
pub type TknMaxAmount = TknId;

/// A Token is an enum with two variants: `Unit` and `Composition`.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Serialize)]
pub enum Token {
    /// A `Unit` consists of an individual character.
    Unit(char),
    /// A `Composition` is a composition of two token ids.
    Composition(TknId, TknId),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Vocabulary {
    tkns: HashMap<TknId, Token>,
    ids: HashMap<Token, TknId>,
    id_to_string: Option<HashMap<TknId, String>>,
    size: TknMaxAmount,
}

impl Vocabulary {
    /// Creates a new vocabulary with no tokens.
    pub fn new() -> Vocabulary {
        Vocabulary {
            tkns: HashMap::new(),
            ids: HashMap::new(),
            id_to_string: None,
            size: 0,
        }
    }
    /// Returns the number of tokens in the vocabulary.
    pub fn len(&self) -> TknMaxAmount {
        self.size
    }
    /// Pushes a token into the vocabulary and returns its id.
    pub fn push(&mut self, tkn: Token) -> TknId {
        let next_id = self.len();
        let id = self.ids.entry(tkn.clone()).or_insert_with(|| {
            self.size += 1;
            next_id
        });
        self.tkns.insert(*id, tkn);
        *id
    }
    /// Decodes a token into a string.
    pub fn decode_single(&self, id: &TknId, s: &mut String) {
        let tkn = self.tkns.get(id).expect("Token ID should be valid.");
        match tkn {
            Token::Unit(ch) => s.push(*ch),
            Token::Composition(left, right) => {
                self.decode_single(left, s);
                self.decode_single(right, s);
            }
        }
    }

    /// Decodes a sequence of token ids into a string.
    /// Will skip over unknown ids!
    pub fn decode(&mut self, ids: &[TknId], s: &mut String) {
        if self.id_to_string.is_none() {
            self.id_to_string = Some(
                self.tkns
                    .iter()
                    .map(|(id, _)| {
                        let mut id_string = String::new();
                        self.decode_single(id, &mut id_string);
                        (*id, id_string)
                    })
                    .collect(),
            );
        }
        for id in ids {
            if let Some(id_string) = self.id_to_string.as_ref().unwrap().get(id) {
                s.push_str(id_string);
            }
        }
    }
    /// Preinitializes the vocabulary with all the individual characters.
    /// Outputs a vector of token ids that correspond to the characters.
    fn preinitialize_vocabulary(&mut self, text_data: &str) -> Vec<TknId> {
        println!(
            "Preinitializing vocabulary with {} characters",
            text_data.len()
        );
        let mut converted_text = vec![];
        for c in text_data.chars() {
            let id = self.push(Token::Unit(c));
            converted_text.push(id);
        }
        converted_text
    }
    /// Converts a TknDiagram into a new token and adds it to the vocabulary.
    /// If one of ids in the id pair aren't valid token, it will return None.
    fn new_id(&mut self, diagram: TknDiagram) -> Option<TknId> {
        let (idleft, idright) = diagram;
        if let (Some(_), Some(_)) = (self.tkns.get(&idleft), self.tkns.get(&idright)) {
            Some(self.push(Token::Composition(idleft, idright)))
        } else {
            None
        }
    }
    /// WIP
    pub fn learn(
        &mut self,
        data: &str,
        merges: TknMaxAmount,
        replacements: usize,
        cutoff: TknMaxAmount,
    ) -> Vec<TknId> {
        println!(
            "Learning BPE with {} merges, {} replacements, and with a cutoff of {}",
            merges, replacements, cutoff
        );
        let mut cur_i = 0;
        let mut text: Vec<TknId> = self.preinitialize_vocabulary(data);
        let mut new_text: Vec<TknId> = vec![];
        let mut counts: HashMap<TknDiagram, TknMaxAmount> = HashMap::new();
        for _ in 0..merges {
            if cur_i == merges {
                break;
            }
            digram_count(&text, &mut counts);
            let mut top_digrams = top_n_digrams(&counts, replacements, cutoff);
            if top_digrams.is_empty() {
                break;
            }
            while let Some(digram) = top_digrams.pop() {
                let new_id = self.new_id(digram.0);
                let new_id = new_id.unwrap();
                let mut i = 0;
                while i < text.len() {
                    match text.get(i..i + 2) {
                        Some(pair) if (pair[0], pair[1]) == digram.0 => {
                            new_text.push(new_id);
                            i += 2;
                        }
                        _ => {
                            new_text.push(text[i]);
                            i += 1;
                        }
                    }
                }
                let tmp = text;
                text = new_text;
                new_text = tmp;
                new_text.clear();
                cur_i += 1;
                println!(
                    "Current iteration: {}, number of tokens: {}",
                    cur_i,
                    self.len()
                );
            }
            counts.clear();
        }
        text
    }
}

/// Counts all the token id pairs if given a hash map and a slice of tokens.
fn digram_count(text: &[TknId], id_to_count: &mut HashMap<TknDiagram, TknMaxAmount>) {
    for pair in text.windows(2) {
        id_to_count
            .entry((pair[0], pair[1]))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
}
/// Return the `n` most common token id pairs in descending order that have a count greater than `min`.
fn top_n_digrams(
    diagram_to_count: &HashMap<TknDiagram, TknMaxAmount>,
    n: usize,
    min: TknMaxAmount,
) -> Vec<(TknDiagram, TknMaxAmount)> {
    let mut top_n: Vec<(TknDiagram, TknMaxAmount)> = diagram_to_count
        .iter()
        .map(|(diagram, count)| (*diagram, *count))
        .filter(|&(_, count)| count > min)
        .collect();
    top_n.sort_by_key(|&(_, count)| count);
    top_n.reverse();
    top_n.truncate(n);
    println!("{:?}", top_n);
    top_n
}

pub fn print_top_n_tokens(vocab: &mut Vocabulary, n: usize) {
    let ids_clone = vocab.ids.clone();
    let mut tokens: Vec<&Token> = ids_clone.keys().collect();
    tokens.sort_by_key(|token| ids_clone.get(token).unwrap());
    tokens.reverse();
    for token in tokens.iter().map(|tkn| ids_clone.get(tkn).unwrap()).take(n) {
        let slice = std::slice::from_ref(token);
        let mut token_str = String::new();
        vocab.decode(slice, &mut token_str);
        println!("{tkn_str:?}", tkn_str = token_str);
    }
}
