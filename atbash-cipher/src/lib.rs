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
        .enumerate()
        .fold(Vec::<String>::new(), |mut acc, (i, c)| {
            if i > 0 && i % 5 == 0 {
                acc.push(" ".into());
            }

            acc.push(c);
            acc
        })
        .join("")
}

pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(process)
        .collect()
}
