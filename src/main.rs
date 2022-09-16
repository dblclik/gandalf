mod cipher;
mod cryptanalysis;
mod io;
mod utils;

use clap::{Args, Parser, Subcommand};
use std::option;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum AttackOptions {
    Xor,
}

impl FromStr for AttackOptions {
    type Err = ();

    fn from_str(input: &str) -> Result<AttackOptions, Self::Err> {
        match input.to_lowercase().as_str() {
            "xor" => Ok(AttackOptions::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Turns a string into a hex string
    Hexlify(InputString),
    /// Converts a hex string into URL-safe b64
    HexToB64(HexString),
    /// Takes in two hex strings of equal length and computes the XOR of them
    XorHexStrings(XorHexStrings),
    /// Takes in a hex string and shows the result of attempting single-byte xor decrypt
    SingleByteXorDecrypt(HexString),
    /// Command to attack a file of ciphertexts
    AttackFile(FileOptions),
    /// Encrypts a file with optionally repeating XOR
    XorEncrypt(XorEncryptOptions),
}

#[derive(Args, Clone, Debug)]
struct InputString {
    #[clap(value_parser)]
    string: Option<String>,
}

#[derive(Args, Clone, Debug)]
struct HexString {
    #[clap(value_parser)]
    hex_string: Option<String>,
}

#[derive(Args, Clone, Debug)]
struct XorHexStrings {
    #[clap(value_parser)]
    hex_string_a: Option<String>,
    hex_string_b: Option<String>,
}

#[derive(Args, Clone, Debug)]
struct XorEncryptOptions {
    #[clap(value_parser)]
    input_path: Option<String>,
    private_key: Option<String>,
    repeating: Option<bool>,
}

#[derive(Args, Clone, Debug)]
struct FileOptions {
    #[clap(value_parser)]
    input_path: Option<String>,
    separator: Option<u8>,
    attack_scheme: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Hexlify(input) => {
            println!(
                "{}",
                utils::hex::encode_hex(input.to_owned().string.unwrap().as_bytes())
            )
        }
        Commands::HexToB64(input_string) => {
            let hex_input = input_string.to_owned().hex_string.unwrap();
            let hex_bytes = utils::hex::decode_hex(&hex_input).unwrap();
            let b64_result = utils::xcode::urlsafe_base64_encode(&hex_bytes);
            println!("Base64 Version is: {}", b64_result)
        }
        Commands::XorHexStrings(input_strings) => {
            let hex_input_a = input_strings.to_owned().hex_string_a.unwrap();
            let hex_input_b = input_strings.to_owned().hex_string_b.unwrap();
            let hex_bytes_a = utils::hex::decode_hex(&hex_input_a).unwrap();
            let hex_bytes_b = utils::hex::decode_hex(&hex_input_b).unwrap();
            let result = utils::hex::encode_hex(
                &utils::bit_ops::xor_bytes(hex_bytes_a, hex_bytes_b).unwrap(),
            );
            println!("Hex Version of XOR operation is: {}", result)
        }
        Commands::SingleByteXorDecrypt(input) => {
            let hex_input = input.to_owned().hex_string.unwrap();
            let hex_bytes = utils::hex::decode_hex(&hex_input).unwrap();
            println!("{:?}", hex_bytes);
            let results = cryptanalysis::get_likely_xor_byte(&hex_bytes);
            println!("XOR Analysis Result is:");
            println!("  - Likely XOR Byte: {}", results.xor_byte);
            println!("  - Result Score: {}", results.score);
            println!("  - Result Plaintext: {}", results.plaintext);
        }
        Commands::AttackFile(file_input) => {
            let split_file_contents = io::split_file(
                Path::new(&file_input.to_owned().input_path.unwrap()),
                file_input.to_owned().separator.unwrap(),
            );
            println!("Retrieved {} results from file", split_file_contents.len());
            let attack = match AttackOptions::from_str(
                file_input.to_owned().attack_scheme.unwrap().as_str(),
            ) {
                Ok(x) => x,
                Err(_) => panic!(
                    "Attack option {} has not been implemented!",
                    file_input.to_owned().attack_scheme.unwrap()
                ),
            };
            match attack {
                AttackOptions::Xor => {
                    let mut max_attack_score = cryptanalysis::XorAnalysisOutput::default();
                    let mut max_score = 0;
                    for vec in split_file_contents {
                        let hex_bytes =
                            utils::hex::decode_hex(String::from_utf8(vec).unwrap().as_str())
                                .unwrap();
                        let results = cryptanalysis::get_likely_xor_byte(&hex_bytes);
                        if results.score > max_score {
                            max_score = results.score;
                            max_attack_score = results.clone()
                        }
                    }
                    println!("XOR Analysis Result is:");
                    println!("  - Likely XOR Byte: {}", max_attack_score.xor_byte);
                    println!("  - Result Score: {}", max_attack_score.score);
                    println!("  - Result Plaintext: {}", max_attack_score.plaintext);
                }
            }
        }
        Commands::XorEncrypt(encrypt_options) => {
            let options = encrypt_options.to_owned();
            let file_contents = io::read_file(Path::new(&options.input_path.unwrap()));

            let xor_cipher = cipher::xor::XorCipher {
                private_key: &utils::hex::decode_hex(options.private_key.unwrap().as_str())
                    .unwrap(),
                repeating: options.repeating.unwrap(),
            };
            let ciphertext = xor_cipher.encrypt(&file_contents);
            println!("{}", utils::hex::encode_hex(&ciphertext));
        }
    }
}
