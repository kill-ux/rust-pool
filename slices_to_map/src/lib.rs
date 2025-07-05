use std::{collections::HashMap, hash::Hash};

pub fn slices_to_map<'a, T: Hash + Eq, U: Eq>(a: &'a [T], b: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    let len = a.len().min(b.len());
    for i in 0..len {
        map.insert(&a[i], &b[i]);
    }
    map
}
