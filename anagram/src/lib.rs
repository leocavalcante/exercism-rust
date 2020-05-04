use std::collections::HashSet;

fn sort(input: &str) -> String {
    let mut chars: Vec<&str> = input.split("").collect();
    chars.sort();
    chars.join("")
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lc_word = word.to_lowercase();
    let sorted = sort(&lc_word);

    possible_anagrams
        .iter()
        .filter_map(|poss| {
            let lc_poss = poss.to_lowercase();

            if sort(&lc_poss) == sorted && lc_poss != lc_word {
                Some(*poss)
            } else {
                None
            }
        })
        .collect()
}
