pub fn edit_distance(source: &str, target: &str) -> usize {
    if source.is_empty() {
        return target.chars().count();
    }
    if target.is_empty() {
        return source.chars().count();
    }
    let first_source = source.chars().next().unwrap();
    let first_target = target.chars().next().unwrap();
    let source_tail = &source[first_source.len_utf8()..];
    let target_tail = &target[first_target.len_utf8()..];
    if first_source == first_target {
        edit_distance(source_tail, target_tail)
    } else {
        let delete = edit_distance(source_tail, target);
        let insert = edit_distance(source, target_tail);
        let substitute = edit_distance(source_tail, target_tail);
        1 + delete.min(insert.min(substitute))
    }
}
