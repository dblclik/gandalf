pub mod hex;
// pub mod pem;
pub mod bit_ops;
pub mod xcode;

#[derive(Debug)]
pub struct RandBytes {
    pub rand_vec: Vec<u8>,
}

impl RandBytes {
    pub fn new(len: usize) -> RandBytes {
        RandBytes {
            rand_vec: vec![0u8; len],
        }
    }
}

fn get_random_buf(x: usize) -> Result<RandBytes, getrandom::Error> {
    let mut buf = RandBytes::new(x);
    getrandom::getrandom(&mut buf.rand_vec)?;
    Ok(buf)
}

pub fn get_random_32() -> RandBytes {
    let rand_bytes = get_random_buf(32);
    match rand_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("There was an error generating the random bytes {}", e)
        }
    }
}

pub fn get_random_12() -> RandBytes {
    let rand_bytes = get_random_buf(12);
    match rand_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("There was an error generating the random bytes {}", e)
        }
    }
}

pub fn get_random_8() -> RandBytes {
    let rand_bytes = get_random_buf(8);
    match rand_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("There was an error generating the random bytes {}", e)
        }
    }
}

pub fn get_random(bytes_len: usize) -> RandBytes {
    let rand_bytes = get_random_buf(bytes_len);
    match rand_bytes {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("There was an error generating the random bytes {}", e)
        }
    }
}

// pub fn parse_jwks(input: &str) -> jsonwebkey_convert::JsonWebKeySet {
//     input.parse().unwrap()
// }

// pub fn get_rsa_int(b: &jsonwebkey_convert::Base64BigUint) -> rsa::BigUint {
//     rsa::BigUint::from_bytes_be(
//         &base64::decode_config(&b.to_base64url(), base64::URL_SAFE_NO_PAD).unwrap(),
//     )
// }

#[cfg(test)]
mod tests {
    use crate::utils::bit_ops::xor_bytes;

    use super::{hex::decode_hex, hex::encode_hex, xcode::urlsafe_base64_encode, *};
    #[test]
    fn test_rand32() {
        let result = get_random_32();
        assert_eq!(result.rand_vec.len(), 32);
    }

    #[test]
    fn test_rand8() {
        let result = get_random_8();
        assert_eq!(result.rand_vec.len(), 8);
    }

    #[test]
    fn test_rand() {
        let result = get_random(237);
        assert_eq!(result.rand_vec.len(), 237);
    }

    #[test]
    fn test_hex_to_b64() {
        let hex_str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let target_b64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let hex_bytes = decode_hex(hex_str).unwrap();
        let b64_result = urlsafe_base64_encode(&hex_bytes);
        assert_eq!(target_b64.to_owned(), b64_result);
    }

    #[test]
    fn test_byte_array_xor() {
        let hex_str_a = "1c0111001f010100061a024b53535009181c";
        let hex_str_b = "686974207468652062756c6c277320657965";
        let target_hex = "746865206b696420646f6e277420706c6179";
        let hex_bytes_a = decode_hex(hex_str_a).unwrap();
        let hex_bytes_b = decode_hex(hex_str_b).unwrap();
        let hex_result = encode_hex(&xor_bytes(hex_bytes_a, hex_bytes_b).unwrap());
        assert_eq!(target_hex.to_owned(), hex_result);
    }
}
