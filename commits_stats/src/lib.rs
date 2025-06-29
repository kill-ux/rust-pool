use std::collections::HashMap;

use chrono::{DateTime, Datelike, Utc};

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    for ele in data.members() {
        let date = ele["commit"]["author"]["date"]
            .to_string()
            .parse::<DateTime<Utc>>()
            .unwrap();
        *map.entry(format!("{:?}", date.iso_week())).or_insert(0) += 1;
    }
    map
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for ele in data.members() {
        let login = ele["author"]["login"].to_string();
        *map.entry(login).or_insert(0) += 1;
    }
    map
}
