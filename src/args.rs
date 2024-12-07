use clap::Parser;

/// Command-line arguments parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    pub length: usize,

    /// Include uppercase letters
    #[arg(short = 'u', long = "include-uppercase", default_value_t = true)]
    pub include_uppercase: bool,

    /// Include numbers
    #[arg(short = 'n', long = "include-numbers", default_value_t = true)]
    pub include_numbers: bool,

    /// Include special characters
    #[arg(short = 's', long = "include-special-chars", default_value_t = true)]
    pub include_special_chars: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
