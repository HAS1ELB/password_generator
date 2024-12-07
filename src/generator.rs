use rand::Rng;

pub fn generate_password(
    length: usize,
    include_uppercase: bool,
    include_numbers: bool,
    include_special_chars: bool,
) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");

    if include_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if include_numbers {
        charset.push_str("0123456789");
    }
    if include_special_chars {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?/");
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}
