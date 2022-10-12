#![allow(dead_code)]
use std::{collections::HashMap, slice::Chunks};

use crate::utils::bit_ops;
// use std::collections::HashMap;

pub mod frequency;

#[derive(Clone, Debug, PartialEq)]
pub struct XorAnalysisOutput {
    pub xor_byte: u8,
    pub plaintext: String,
    pub score: usize,
}

impl Default for XorAnalysisOutput {
    fn default() -> Self {
        return XorAnalysisOutput {
            xor_byte: 0u8,
            plaintext: String::default(),
            score: 0,
        };
    }
}

// pub fn score
pub fn get_likely_xor_byte(input_bytes: &[u8]) -> XorAnalysisOutput {
    let mut max_result: XorAnalysisOutput = XorAnalysisOutput::default();
    let mut max_score: usize = 0;
    for i in 0..255u8 {
        let xor_array = vec![i; input_bytes.len()];
        let original_bytes = bit_ops::xor_bytes(xor_array, input_bytes.to_vec()).unwrap();
        let original_bytes_score = frequency::string_score(&original_bytes);
        if original_bytes_score > max_score {
            max_score = original_bytes_score;
            max_result = XorAnalysisOutput {
                xor_byte: i.clone(),
                plaintext: String::from_utf8(original_bytes).unwrap_or(String::new()),
                score: max_score.clone(),
            }
        }
    }
    max_result
}

pub fn repeating_key_xor_attack(
    input_bytes: &[u8],
    min_size: u64,
    max_size: u64,
    averaging_depth: u64,
    return_results: u64,
) -> Vec<XorAnalysisOutput> {
    let most_likely_results: Vec<XorAnalysisOutput> = vec![];
    if return_results > (max_size - min_size) {
        panic!("Cannot provide back more results than the number of possibilities evaluated!");
    }

    let mut key_size_hashmap: HashMap<u64, u64> = HashMap::new();
    for ks in min_size..(max_size + 1) {
        let mut bytes_iter = input_bytes.chunks(ks.try_into().unwrap());
        let mut accumulated_diffs = 0u64;
        let prev_chunk = bytes_iter.next().unwrap();
        for _rounds in 0..averaging_depth {
            let next_chunk = bytes_iter.next();
            match next_chunk {
                Some(chunk) => {
                    let dist = bit_ops::hamming_distance(prev_chunk, chunk).unwrap() / ks;
                    accumulated_diffs = accumulated_diffs + dist;
                }
                None => {
                    break;
                }
            }
        }
        key_size_hashmap.insert(ks, accumulated_diffs);
    }
    most_likely_results
}

// handy code for converting a hashmap of counts into a reverse sorted vec
// let frequencies = frequency::freq_score(input_bytes);
// let mut count_vec: Vec<_> = frequencies.iter().collect();
// count_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
// let mut result_vecs: Vec<String> = vec![];

#[cfg(test)]
mod tests {
    use crate::cryptanalysis::frequency::freq_score;
    use crate::cryptanalysis::get_likely_xor_byte;
    use crate::utils::bit_ops;

    #[test]
    fn test_byte_freq() {
        let bytes_array = [55u8, 65u8, 66u8, 65u8, 56u8, 55u8];
        let frequencies = freq_score(&bytes_array);
        assert_eq!(frequencies.get(&55u8).unwrap().clone(), 2);
    }

    #[test]
    fn test_likely_xor() {
        let orig_string = b"He EVEN SET";
        let xor_byte = 55u8;
        let xor_array = vec![xor_byte; orig_string.len()];
        let encrypted_bytes = bit_ops::xor_bytes(xor_array, orig_string.to_vec()).unwrap();
        let potential_strings = get_likely_xor_byte(&encrypted_bytes);
        assert_eq!(potential_strings.plaintext, String::from("He EVEN SET"));
        assert_eq!(potential_strings.xor_byte, xor_byte);
    }
}
