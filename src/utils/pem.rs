// use rsa::{pkcs1::DecodeRsaPublicKey, RsaPublicKey};
// use std::fmt;

// use crate::utils::parse_jwks;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum PemError {
//     RequestError,
//     InvalidJwksKey,
//     NotImplementedErrror,
//     Pkcs1Error(rsa::pkcs1::Error),
// }

// impl From<rsa::pkcs1::Error> for PemError {
//     fn from(e: rsa::pkcs1::Error) -> Self {
//         PemError::Pkcs1Error(e)
//     }
// }

// impl fmt::Display for PemError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PemError::Pkcs1Error(e) => e.fmt(f),
//             PemError::RequestError => write!(f, "Error requesting the PEM file from URL"),
//             PemError::InvalidJwksKey => write!(f, "No RSA Public Key provided by JWKS URL"),
//             PemError::NotImplementedErrror => {
//                 write!(f, "Functionality required has not been implemented yet...")
//             }
//         }
//     }
// }

// impl std::error::Error for PemError {}

// pub fn get_pem_from_url(pem_url: &str) -> Result<String, PemError> {
//     let body = match reqwest::blocking::get(pem_url) {
//         Ok(r) => match r.text() {
//             Ok(t) => t,
//             Err(_e) => return Err(PemError::RequestError),
//         },
//         Err(_e) => return Err(PemError::RequestError),
//     };
//     match RsaPublicKey::from_pkcs1_pem(&body) {
//         Ok(_b) => Ok(body),
//         Err(e) => Err(PemError::Pkcs1Error(e)),
//     }
// }

// pub fn get_pem_from_jwks(jwks_url: &str) -> Result<String, PemError> {
//     let body = match reqwest::blocking::get(jwks_url) {
//         Ok(r) => match r.text() {
//             Ok(t) => t,
//             #[allow(clippy::needless_return)]
//             Err(_e) => return Err(PemError::RequestError),
//         },
//         #[allow(clippy::needless_return)]
//         Err(_e) => return Err(PemError::RequestError),
//     };
//     let jwks = parse_jwks(&body);

//     // TODO - this assumes the first key is the right key...
//     //    ...if we keep going using this w/o a service registry, we
//     //    should allow the user to specify the kid
//     let jwk = &jwks.keys[0].rsa_public_key();

//     let rsa_public_key;
//     if let Some(key) = jwk.to_owned() {
//         rsa_public_key = key;
//     } else {
//         return Err(PemError::InvalidJwksKey);
//     }

//     // We need to get the PEM from the jwk
//     //  if we have an x5u, use that
//     let x5u_url;
//     if let Some(url) = &rsa_public_key.generic.x5u {
//         x5u_url = url.to_owned();
//         let body = get_pem_from_url(&x5u_url)?;
//         match RsaPublicKey::from_pkcs1_pem(&body) {
//             Ok(_b) => Ok(body),
//             Err(e) => Err(PemError::Pkcs1Error(e)),
//         }
//     } else {
//         // if we don't have an x5u, build from n and e
//         // let pem_data = rsa_public_key.to_pem()?;

//         // TODO - implement...
//         // let n = get_rsa_int(&rsa_public_key.n);
//         // let e = get_rsa_int(&rsa_public_key.e);
//         // let std_rsa_pub_key = RsaPublicKey::new(n, e).unwrap();
//         // let pem_file =
//         //     rsa::pkcs1::RsaPublicKey::to_pem(&std_rsa_pub_key, rsa::pkcs1::LineEnding::LF);
//         Err(PemError::NotImplementedErrror)
//     }
// }

// #[cfg(test)]
// mod tests {

//     use super::*;
//     #[test]
//     fn test_get_pem_from_url() {
//         let pem = "-----BEGIN RSA PUBLIC KEY-----
// MIIBCgKCAQEAr9EZ2cwHWiG3qvcgz32AjFo3ZHGLLSH1/t+sEVOMsJ9kDwUisrDB
// 4wOn9zXiJqWcF1OuED6yzaWpo/3wAhawyzwhYD97nbGlEITLd7uBx46rNulJKJoN
// xYK5MXOGYr0vM9Wim3GEIyrNG9gAgsk/iQaWB5AkNfghGK4qHbE2AqCO8VTm+t9h
// DgIzgBBIocz18FknGM4L8E1lWIyIkTUoUmITz52DLe143O+hGDu4NSBx7U3tVkbX
// VsMDPMy/BmFhuZlMePiXdLjzSd+xJ9eXDe4+nMoGme6fsVdKp8UC3E3JAcLhiT+d
// 5uZbk3Xm1BefRWj8h2InUanYkVhOQme02wIDAQAB
// -----END RSA PUBLIC KEY-----\n";
//         let pem_string =
//             get_pem_from_url("https://secretsanta.posd.avm.oliveai.io/public/key.pem").unwrap();
//         assert_eq!(&pem_string, pem);
//     }

//     fn test_get_pem_from_jwks() {
//         let pem = "-----BEGIN RSA PUBLIC KEY-----
// MIIBCgKCAQEAr9EZ2cwHWiG3qvcgz32AjFo3ZHGLLSH1/t+sEVOMsJ9kDwUisrDB
// 4wOn9zXiJqWcF1OuED6yzaWpo/3wAhawyzwhYD97nbGlEITLd7uBx46rNulJKJoN
// xYK5MXOGYr0vM9Wim3GEIyrNG9gAgsk/iQaWB5AkNfghGK4qHbE2AqCO8VTm+t9h
// DgIzgBBIocz18FknGM4L8E1lWIyIkTUoUmITz52DLe143O+hGDu4NSBx7U3tVkbX
// VsMDPMy/BmFhuZlMePiXdLjzSd+xJ9eXDe4+nMoGme6fsVdKp8UC3E3JAcLhiT+d
// 5uZbk3Xm1BefRWj8h2InUanYkVhOQme02wIDAQAB
// -----END RSA PUBLIC KEY-----\n";
//         let pem_string =
//             get_pem_from_jwks("https://secretsanta.posd.avm.oliveai.io/public/jwks.json").unwrap();
//         assert_eq!(&pem_string, pem);
//     }
// }
