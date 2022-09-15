use crate::utils::bit_ops;

pub mod frequency;

pub fn get_likely_xor_byte(input_bytes: &[u8], depth: usize, upper: bool) -> Vec<String> {
    let frequencies = frequency::freq_score(input_bytes);
    let mut count_vec: Vec<_> = frequencies.iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(b.1).reverse());
    let mut result_vecs: Vec<String> = vec![];
    for i in 0usize..depth {
        let original_char: &u8;
        if upper {
            original_char = &frequency::LETTER_FREQUENCY_ORDER_UPPER[i];
        } else {
            original_char = &frequency::LETTER_FREQUENCY_ORDER_LOWER[i];
        }
        let xor_char = count_vec[0].0 ^ original_char;

        let xor_array = vec![xor_char; input_bytes.len()];
        let original_bytes = bit_ops::xor_bytes(xor_array, input_bytes.to_vec()).unwrap();
        result_vecs.push(String::from_utf8(original_bytes).unwrap());
    }
    result_vecs
}

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
        let orig_string = b"HE EVEN SET";
        let xor_byte = 55u8;
        let xor_array = vec![xor_byte; orig_string.len()];
        let encrypted_bytes = bit_ops::xor_bytes(xor_array, orig_string.to_vec()).unwrap();
        let potential_strings = get_likely_xor_byte(&encrypted_bytes, 1, true);
        assert_eq!(potential_strings[0], String::from("HE EVEN SET"));
    }
}
