use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

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

    pub fn square(size: usize) -> Result<Self, String> {
        Self::new(size, size)
    }

    pub fn is_square(&self) -> bool {
        self.num_rows == self.num_cols
    }
}

impl<T> Matrix<T>
where
    T: Default + Copy,
    Standard: Distribution<T>,
{
    pub fn random(num_rows: usize, num_cols: usize) -> Result<Matrix<T>, String> {
        let mut matrix = Self::new(num_rows, num_cols)?;

        let mut rng = rand::thread_rng();

        for i in 0..num_rows {
            for j in 0..num_cols {
                matrix[i][j] = rng.gen();
            }
        }

        Ok(matrix)
    }

    pub fn square_random(size: usize) -> Result<Self, String> {
        Self::random(size, size)
    }
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.data[index]
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
        assert_eq!(matrix1.data, vec![vec![0, 0, 0], vec![0, 0, 0]]);
        assert_eq!(matrix1.is_square(), false);

        assert!(Matrix::<usize>::new(0, 0).is_err());
    }

    #[test]
    fn is_square() {
        assert!(Matrix::<usize>::new(3, 3).unwrap().is_square());
        assert!(!Matrix::<usize>::new(3, 2).unwrap().is_square());
    }

    #[test]
    fn random_returns_errors() {
        assert!(Matrix::<usize>::random(0, 0).is_err());
        assert!(Matrix::<usize>::random(1, 1).is_ok());
    }

    #[test]
    fn random_is_filled() {
        let matrix1: Matrix<f32> = Matrix::random(3, 3).unwrap();
        let matrix2: Matrix<usize> = Matrix::random(3, 3).unwrap();
        let matrix3: Matrix<i8> = Matrix::random(3, 3).unwrap();
        assert!(matrix1.data.iter().flatten().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().flatten().any(|&x| x != 0));
        assert!(matrix3.data.iter().flatten().any(|&x| x != 0));

        let matrix1: Matrix<f32> = Matrix::square_random(3).unwrap();
        let matrix2: Matrix<usize> = Matrix::square_random(3).unwrap();
        let matrix3: Matrix<i8> = Matrix::square_random(3).unwrap();
        assert!(matrix1.data.iter().flatten().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().flatten().any(|&x| x != 0));
        assert!(matrix3.data.iter().flatten().any(|&x| x != 0));
    }

    #[test]
    fn make_square() {
        let matrix1: Matrix<usize> = Matrix::square(3).unwrap();
        assert!(matrix1.is_square());

        let matrix2: Result<Matrix<usize>, String> = Matrix::square(0);
        assert!(matrix2.is_err());

        let matrix2: Result<Matrix<usize>, String> = Matrix::square(10);
        assert!(matrix2.is_ok());
    }

    #[test]
    fn make_square_random() {
        let matrix1: Matrix<usize> = Matrix::square_random(3).unwrap();
        assert!(matrix1.is_square());

        let matrix2: Result<Matrix<usize>, String> = Matrix::square_random(0);
        assert!(matrix2.is_err());

        let matrix2: Result<Matrix<usize>, String> = Matrix::square_random(10);
        assert!(matrix2.is_ok());
    }
}
