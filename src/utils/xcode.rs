pub fn urlsafe_base64_decode(input: &str) -> Vec<u8> {
    let modified_url_safe_config = base64::Config::new(base64::CharacterSet::UrlSafe, false);
    base64::decode_config(input, modified_url_safe_config).unwrap()
}

pub fn urlsafe_base64_encode(input: &[u8]) -> String {
    let modified_url_safe_config = base64::Config::new(base64::CharacterSet::UrlSafe, false);
    base64::encode_config(input, modified_url_safe_config)
}
