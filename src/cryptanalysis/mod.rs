#![allow(dead_code)]
use crate::utils::bit_ops;
// use std::collections::HashMap;

pub mod frequency;

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
                plaintext: String::from_utf8(original_bytes).unwrap(),
                score: max_score.clone(),
            }
        }
    }
    max_result
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
