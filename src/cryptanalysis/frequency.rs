#![allow(dead_code)]
use phf::phf_map;
use std::collections::HashMap;

pub const LETTER_FREQUENCY_ORDER_UPPER: &[u8] = b"ETAOINSRHLDCUMFPGWYBVKXJQZ";

pub const LETTER_FREQUENCY_ORDER_LOWER: &[u8] = b"etaoinsrhldcumfpgwybvkxjqz";

pub static LETTER_SCORES_LOWER: phf::Map<u8, usize> = phf_map! {
    101u8 => 21912, 116u8 => 16587, 97u8 => 14810, 111u8 => 14003, 105u8 => 13318, 110u8 => 12666,
115u8 => 11450, 114u8 => 10977, 104u8 => 10795, 100u8 => 7874, 108u8 => 7253, 117u8 => 5246, 99u8 => 4943,
109u8 => 4761, 102u8 => 4200, 121u8 => 3853, 119u8 => 3819, 103u8 => 3693, 112u8 => 3316, 98u8 => 2715,
118u8 => 2019, 107u8 => 1257, 120u8 => 315, 113u8 => 205, 106u8 => 188, 122u8 => 128, 69u8 => 21912,
84u8 => 16587, 65u8 => 14810, 79u8 => 14003, 73u8 => 13318, 78u8 => 12666, 83u8 => 11450, 82u8 => 10977,
72u8 => 10795, 68u8 => 7874, 76u8 => 7253, 85u8 => 5246, 67u8 => 4943, 77u8 => 4761, 70u8 => 4200, 89u8 => 3853,
87u8 => 3819, 71u8 => 3693, 80u8 => 3316, 66u8 => 2715, 86u8 => 2019, 75u8 => 1257, 88u8 => 315, 81u8 => 205,
74u8 => 188, 90u8 => 128, 32u8 => 25000
};

pub fn freq_score(input_string: &[u8]) -> HashMap<u8, usize> {
    let mut m: HashMap<u8, usize> = HashMap::new();
    for x in input_string {
        *m.entry(x.clone()).or_default() += 1;
    }
    m
}

pub fn string_score(input_string: &[u8]) -> usize {
    let resulting_score = input_string
        .iter()
        .map(|x| LETTER_SCORES_LOWER.get(x).unwrap_or_else(|| &0))
        .sum();
    resulting_score
}
