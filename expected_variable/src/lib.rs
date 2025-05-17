use convert_case::{Case, Casing};
use edit_distance as Ed;

pub fn expected_variable(expected: &str, s: &str) -> Option<String> {
    if !s.is_case(Case::Camel) && !s.is_case(Case::Snake) && !s.is_case(Case::UpperCamel) {
        None
    } else {
        let s = &s.to_lowercase();
        let expected = &expected.to_lowercase();
        let edit = Ed::edit_distance(s, expected);
        let res = 100 - ((edit * 100) / s.len());
        if res <= 50 {
            None
        } else {
            Some(res.to_string() + "%")
        }
    }
}
