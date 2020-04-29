fn process(char: char) -> String {
    if char.is_digit(10) {
        char.to_string()
    } else {
        char::from(122 - ((char as u8) - 97)).to_string()
    }
}

pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(process)
        .collect::<Vec<String>>()
        .chunks(5)
        .map(|c| c.join(""))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(process)
        .collect()
}
