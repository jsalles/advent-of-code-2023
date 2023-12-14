use itertools::Itertools;

pub type Matrix = Vec<Vec<char>>;

pub fn read_matrix(input: &str) -> Matrix {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

pub fn read_matrices(input: &str) -> Vec<Matrix> {
    input
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

pub fn transpose(map: &Matrix) -> Matrix {
    let mut transposed = Vec::new();
    (0..map[0].len()).for_each(|col| {
        let mut column = Vec::new();
        (0..map.len()).for_each(|row| {
            column.push(map[row][col]);
        });
        transposed.push(column);
    });

    transposed
}

pub fn reverse_rows(matrix: &Matrix) -> Matrix {
    matrix
        .iter()
        .map(|row| row.iter().copied().rev().collect_vec())
        .collect_vec()
}

pub fn print_matrix(map: &Matrix) {
    for r in map {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
}
