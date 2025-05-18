use convert_case::{Case, Casing};
use edit_distance as Ed;

pub fn expected_variable(expected: &str, s: &str) -> Option<String> {
    let s = &s.to_lowercase();
    let expected = &expected.to_lowercase();
    if !s.is_case(Case::Camel) && !s.is_case(Case::Snake) {
        None
    } else {
        let edit = Ed::edit_distance(s, expected);
        println!("{}",edit);
        let res: i64 = 100 - ((edit as i64 * 100) / s.len() as i64);
        if res <= 50 {
            None
        } else {
            Some(res.to_string() + "%")
        }
    }
}
