use std::collections::HashMap;

pub const LETTER_FREQUENCY_ORDER_UPPER: &[u8] = b"ETAOINSRHLDCUMFPGWYBVKXJQZ";

pub const LETTER_FREQUENCY_ORDER_LOWER: &[u8] = b"etaoinsrhldcumfpgwybvkxjqz";

pub fn freq_score(input_string: &[u8]) -> HashMap<u8, usize> {
    let mut m: HashMap<u8, usize> = HashMap::new();
    for x in input_string {
        *m.entry(x.clone()).or_default() += 1;
    }
    m
}
