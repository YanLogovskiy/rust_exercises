use std::collections::HashSet;

pub fn normalize(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_normalized = normalize(word);
    let mut result = HashSet::new();

    for candidate in possible_anagrams {
        if candidate.to_lowercase() == word_lower {
            continue;
        }
        if normalize(candidate) == word_normalized {
            result.insert(*candidate);
        }
    }

    result
}
