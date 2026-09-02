use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut map: HashMap<char, usize> = HashMap::new();

    for c in sentence.to_lowercase().chars().filter(|c| c.is_alphabetic()){
        *map.entry(c).or_insert(0)+=1;
    }
    map.len() == 26
}
