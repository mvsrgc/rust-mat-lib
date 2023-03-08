struct Matrix<T> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<Vec<T>>,
}

impl<T: Default + Copy> Matrix<T> {
    pub fn new(num_rows: usize, num_cols: usize) -> Result<Self, String> {
        if num_rows <= 0 {
            return Err("Number of rows cannot be less than or equal to 0.".to_string());
        };

        if num_cols <= 0 {
            return Err("Number of columns cannot be less than or equal to 0.".to_string());
        };

        let data = vec![vec![T::default(); num_cols]; num_rows];

        Ok(Self {
            num_rows,
            num_cols,
            data,
        })
    }

    pub fn is_square(&self) -> bool {
        self.num_rows == self.num_cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_matrix() {
        let matrix1: Matrix<usize> = Matrix::new(2, 3).unwrap();

        assert_eq!(matrix1.num_rows, 2);
        assert_eq!(matrix1.num_cols, 3);
        assert_eq!(matrix1.data, vec![vec![0,0,0], vec![0,0,0]]);
        assert_eq!(matrix1.is_square(), false);

        assert!(Matrix::<usize>::new(0,0).is_err());
    }
}
