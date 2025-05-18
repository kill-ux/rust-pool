use std::{clone, ops::RangeInclusive};

fn len(rng: RangeInclusive<char>) -> usize {
    let mut le: usize = 0;
    for _ in rng {
        le += 1;
    }
    le
}

pub fn get_diamond(c: char) -> Vec<String> {
    let mut new_vec: Vec<String> = Vec::new();
    let le = len(('A'..=c));
    let mut j: i32 = c as i32 - b'A' as i32;
    let mut i: i32 = 0;
    for (_, ch) in ('A'..=c).enumerate() {
        let s = " ".repeat(j as usize) + &ch.to_string() + &" ".repeat(i as usize) ;
        let mut res = s.clone();
        res.pop();
        let res = res.chars().rev().collect::<String>();
        new_vec.push(s + &res);
        j -= 1;
        i += 1
    }

    let mut r = new_vec.clone() ;
    r.pop();
    let r: Vec<String> = r.into_iter().rev().collect();

    for a in r {
        new_vec.push(a);
    }

    new_vec 
}
