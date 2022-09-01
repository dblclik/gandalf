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
    HexToB64(HexConvertB64),
}

#[derive(Args, Clone, Debug)]
struct HexConvertB64 {
    #[clap(value_parser)]
    hex_string: Option<String>,
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
    }
}
