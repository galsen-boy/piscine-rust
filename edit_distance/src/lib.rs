pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_len = source.len();
    let target_len = target.len();

    let mut matrix = vec![vec![0; target_len + 1]; source_len + 1];

    for i in 1..=source_len {
        matrix[i][0] = i;
    }

    for j in 1..=target_len {
        matrix[0][j] = j;
    }

    for i in 1..=source_len {
        for j in 1..=target_len {
            if source.chars().nth(i - 1).unwrap() == target.chars().nth(j - 1).unwrap() {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                matrix[i][j] = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1, matrix[i - 1][j - 1] + 1);
            }
        }
    }

    matrix[source_len][target_len]
}

pub fn min(a: usize, b: usize, c: usize) -> usize {
    if a < b && a < c {
        a
    } else if b < a && b < c {
        b
    } else {
        c
    }
}