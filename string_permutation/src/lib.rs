use simple_hash::*;
use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let new_s1 = word_frequency_counter(&s1.split("").collect::<Vec<_>>());
    let new_s2 = word_frequency_counter(&s1.split("").collect::<Vec<_>>());

    for (ch, &num) in new_s1.iter() {
        let count = *new_s2.get(ch).unwrap_or(&0);
        if count != num {
            return false;
        }
    }
    true
}
