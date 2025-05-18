pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let m = source_chars.len();
    let n = target_chars.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            if source_chars[i - 1] == target_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1]));
            }
        }
    }

    dp[m][n]
}


pub fn edit_distance2(source: &str, target: &str) -> usize {

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
