use crate::utils::bit_ops::xor_bytes;

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct XorCipher<'a> {
    pub private_key: &'a [u8],
    pub repeating: bool,
}

impl XorCipher<'_> {
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        if !self.repeating && self.private_key.len() < plaintext.len() {
            println!("WARNING:  Encrypting plaintext with length {} with a non-repeating key of length {}.  Will result in partial encryption only.", plaintext.len(), self.private_key.len());
        }
        let mut ciphertext: Vec<u8> = vec![];
        let pt_iter = plaintext.chunks(self.private_key.len());
        let mut use_key = true;
        for chunk in pt_iter {
            if use_key {
                ciphertext.extend_from_slice(
                    &xor_bytes(chunk.to_vec(), self.private_key[..chunk.len()].to_vec()).unwrap(),
                );
                if !self.repeating {
                    use_key = false
                }
            } else {
                ciphertext.extend_from_slice(chunk);
            }
        }

        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Vec<u8> {
        if !self.repeating && self.private_key.len() < ciphertext.len() {
            println!("WARNING:  Encrypting plaintext with length {} with a non-repeating key of length {}.  Will result in partial encryption only.", ciphertext.len(), self.private_key.len());
        }
        let mut plaintext: Vec<u8> = vec![];
        let pt_iter = ciphertext.chunks(self.private_key.len());
        let mut use_key = true;
        for chunk in pt_iter {
            if use_key {
                plaintext.extend_from_slice(
                    &xor_bytes(chunk.to_vec(), self.private_key[..chunk.len()].to_vec()).unwrap(),
                );
                if !self.repeating {
                    use_key = false
                }
            } else {
                plaintext.extend_from_slice(chunk);
            }
        }

        plaintext
    }
}
