mod args;
mod generator;

use args::get_args;
use generator::generate_password;

fn main() {
    let args = get_args();
    let password = generate_password(
        args.length,
        args.include_uppercase,
        args.include_numbers,
        args.include_special_chars,
    );
    println!("Generated Password: {}", password);
}
