mod utils;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts a hex string into URL-safe b64
    HexToB64(HexString),
    /// Takes in two hex strings of equal length and computes the XOR of them
    XorHexStrings(XorHexStrings),
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

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
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
    }
}
