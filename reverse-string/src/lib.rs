pub fn reverse(input: &str) -> String {
    let mut parts: Vec<&str> = input.split("").collect();
    parts.reverse();
    parts.join("")
}
