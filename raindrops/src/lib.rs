pub fn raindrops(n: u32) -> String {
    match [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .flat_map(|&(factor, sound)| if n % factor == 0 { Some(sound) } else { None })
        .collect::<Vec<&str>>()
        .as_slice()
    {
        [] => n.to_string(),
        raindrops => raindrops.join(""),
    }
}
