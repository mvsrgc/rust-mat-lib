struct Matrix {
    num_rows: usize,
    num_cols: usize,
    data: Vec<Vec<usize>>,
    is_square: isize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_matrix() {
        let matrix1 = Matrix {
            num_rows: 3,
            num_cols: 3,
            data: vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]],
            is_square: 1,
        };

        assert_eq!(matrix1.num_cols, 3);
        assert_eq!(matrix1.num_rows, 3);
        assert_eq!(matrix1.data[0][0], 1);
        assert_eq!(matrix1.data[0][1], 2);
        assert_eq!(matrix1.data[0][2], 3);
        assert_eq!(matrix1.data[1][0], 4);
        assert_eq!(matrix1.data[1][1], 5);
        assert_eq!(matrix1.data[1][2], 6);
        assert_eq!(matrix1.data[2][0], 7);
        assert_eq!(matrix1.data[2][1], 8);
        assert_eq!(matrix1.data[2][2], 9);
        assert_eq!(matrix1.is_square, 1);
    }
}
