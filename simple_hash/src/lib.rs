use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut new_hash: HashMap<&'a str, usize> = HashMap::new();
    for word in words {
        new_hash.entry(*word).and_modify(|c| *c += 1 ).or_insert(1);
    }
    new_hash
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
