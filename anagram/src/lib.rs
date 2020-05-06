use std::collections::HashSet;

fn sort(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lc_word = word.to_lowercase();
    let sorted = sort(&lc_word);

    possible_anagrams
        .iter()
        .filter(|poss| {
            let lc_poss = poss.to_lowercase();
            sort(&lc_poss) == sorted && lc_poss != lc_word
        })
        .copied()
        .collect()
}
