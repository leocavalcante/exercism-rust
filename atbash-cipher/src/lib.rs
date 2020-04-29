fn process(char: char) -> char {
    if char.is_digit(10) {
        char
    } else {
        char::from(122 - ((char as u8) - 97))
    }
}

pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(process)
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && i % 5 == 0 {
                acc.push(' ');
            }

            acc.push(c);
            acc
        })
}

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(process)
        .collect()
}
