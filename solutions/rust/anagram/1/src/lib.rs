use std::collections::HashSet;


pub fn normalize(word: &str) -> String {
    let lower = word.to_lowercase();
    let mut chars: Vec<char> = lower.chars().collect();
    chars.sort();
    let res: String = chars.into_iter().collect();
    res
}



pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let target:String = normalize(word);

    possible_anagrams
    .iter()
    .filter(|candidate|{
        let norm_candidate = normalize(*candidate);
        let is_anagram = norm_candidate == target;
        let not_same = candidate.to_lowercase() != word.to_lowercase();
        not_same && is_anagram
    })
    .copied()
    .collect()
}
