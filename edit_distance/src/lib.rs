pub fn edit_distance(source: &str, target: &str) -> usize {
    let l1 = source.len();
    let l2 = target.len();
    let mut matrix: Vec<Vec<usize>> = Vec::with_capacity(l1);
    for i in 0..=l1 {
        let mut vec2: Vec<usize> = vec![];
        if i == 0 {
            for j in 0..=l2 {
                vec2.push(j);
            }
        } else {
            for k in 0..=l2 {
                if k == 0 {
                    vec2.push(i);
                } else {
                    vec2.push(0);
                }
            }
        }
        matrix.push(vec2);
    }

    for i in 1..=l1 {
        for j in 1..=l2 {
            if i != 0 && j != 0 {
                if source[i-1..i] == target[j-1..j] {
                    matrix[i][j] = matrix[i - 1][j - 1]
                } else {
                    let min = matrix[i - 1][j - 1].min(matrix[i][j - 1].min(matrix[i - 1][j]));
                    matrix[i][j] = min + 1;
                }
            }
        }
    }

    matrix[l1][l2]
}
