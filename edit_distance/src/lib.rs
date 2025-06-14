pub fn edit_distance(source: &str, target: &str) -> usize {
    let l1 = source.len();
    let l2 = target.len();
    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(l1);
    for i in 0..=l1 {
        matrix.push( vec![]);
        if i == 0 {
            for j in 0..=l2 {
                matrix[i].push(j);
            }
        } else {
            for j in 0..=l2 {
                if j == 0 {
                    matrix[i].push(i);
                } else if j == 0 && i == 0 {
                    matrix[i].push(0);
                } else {
                    if source[i - 1..i] == target[j - 1..j] {
                        let a = matrix[i - 1][j - 1];
                        matrix[i].push(a);
                    } else {
                        let min = matrix[i - 1][j - 1].min(matrix[i][j - 1].min(matrix[i - 1][j]));
                        matrix[i].push(min + 1);
                    }
                }
            }
        }
    }
    matrix[l1][l2]
}
