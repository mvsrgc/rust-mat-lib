use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::marker::PhantomData;

pub trait Order {
    fn calc_index(pos: (usize, usize), dims: (usize, usize)) -> usize;
}

enum RowMajor {}
impl Order for RowMajor {
    fn calc_index(pos: (usize, usize), dims: (usize, usize)) -> usize {
        let (i, j) = pos;
        let (_, num_cols) = dims;
        i * num_cols + j
    }
}

enum ColMajor {}
impl Order for ColMajor {
    fn calc_index(pos: (usize, usize), dims: (usize, usize)) -> usize {
        let (i, j) = pos;
        let (num_rows, _) = dims;
        j * num_rows + i
    }
}

struct Matrix<T, Order> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
    _order: PhantomData<Order>,
}

impl<T: Default + Copy, Order> Matrix<T, Order> {
    #[inline]
    pub fn from_blind_fn<F: FnMut() -> T>(
        num_rows: usize,
        num_cols: usize,
        f: F,
    ) -> Result<Self, String> {
        if num_rows <= 0 {
            return Err("Number of rows cannot be less than or equal to 0.".to_string());
        };

        if num_cols <= 0 {
            return Err("Number of columns cannot be less than or equal to 0.".to_string());
        };

        let data = std::iter::repeat_with(f)
            .take(num_rows * num_cols)
            .collect();

        Ok(Self {
            num_rows,
            num_cols,
            data,
            _order: PhantomData,
        })
    }

    pub fn default(num_rows: usize, num_cols: usize) -> Result<Self, String>
    where
        T: Default,
    {
        Self::from_blind_fn(num_rows, num_cols, || T::default())
    }

    pub fn random(num_rows: usize, num_cols: usize) -> Result<Self, String>
    where
        Standard: Distribution<T>,
    {
        let mut rng = rand::thread_rng();
        Self::from_blind_fn(num_rows, num_cols, || rng.gen())
    }

    pub fn square(size: usize) -> Result<Self, String> {
        Self::default(size, size)
    }

    pub fn square_random(size: usize) -> Result<Self, String>
    where
        Standard: Distribution<T>,
    {
        Self::random(size, size)
    }

    pub fn from_elem(num_rows: usize, num_cols: usize, elem: T) -> Result<Self, String>
    where
        T: Clone,
    {
        Self::from_blind_fn(num_rows, num_cols, || elem.clone())
    }

    pub fn is_square(&self) -> bool {
        self.num_rows == self.num_cols
    }
}

impl<T, O: Order> std::ops::Index<(usize, usize)> for Matrix<T, O> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let idx = O::calc_index(index, (self.num_rows, self.num_cols));
        &self.data[idx]
    }
}

impl<T, O: Order> std::ops::IndexMut<(usize, usize)> for Matrix<T, O> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (row, col) = index;
        let idx = O::calc_index(index, (self.num_rows, self.num_cols));
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_matrix() {
        let matrix1: Matrix<usize, ColMajor> = Matrix::default(2, 3).unwrap();

        assert_eq!(matrix1.num_rows, 2);
        assert_eq!(matrix1.num_cols, 3);
        assert_eq!(matrix1.data, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(matrix1.is_square(), false);

        assert!(Matrix::<usize, ColMajor>::default(0, 0).is_err());
    }

    #[test]
    fn is_square() {
        assert!(Matrix::<usize, ColMajor>::default(3, 3)
            .unwrap()
            .is_square());
        assert!(!Matrix::<usize, ColMajor>::default(3, 2)
            .unwrap()
            .is_square());
    }

    #[test]
    fn random_returns_errors() {
        assert!(Matrix::<usize, ColMajor>::random(0, 0).is_err());
        assert!(Matrix::<usize, ColMajor>::random(1, 1).is_ok());
    }

    #[test]
    fn random_is_filled() {
        let matrix1: Matrix<f32, ColMajor> = Matrix::random(3, 3).unwrap();
        let matrix2: Matrix<usize, ColMajor> = Matrix::random(3, 3).unwrap();
        let matrix3: Matrix<i8, ColMajor> = Matrix::random(3, 3).unwrap();
        assert!(matrix1.data.iter().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().any(|&x| x != 0));
        assert!(matrix3.data.iter().any(|&x| x != 0));

        let matrix1: Matrix<f32, ColMajor> = Matrix::square_random(3).unwrap();
        let matrix2: Matrix<usize, ColMajor> = Matrix::square_random(3).unwrap();
        let matrix3: Matrix<i8, ColMajor> = Matrix::square_random(3).unwrap();
        assert!(matrix1.data.iter().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().any(|&x| x != 0));
        assert!(matrix3.data.iter().any(|&x| x != 0));
    }

    #[test]
    fn make_square() {
        let matrix1: Matrix<usize, ColMajor> = Matrix::square(3).unwrap();
        assert!(matrix1.is_square());

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square(0);
        assert!(matrix2.is_err());

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square(10);
        assert!(matrix2.is_ok());
    }

    #[test]
    fn make_square_random() {
        let matrix1: Matrix<usize, ColMajor> = Matrix::square_random(3).unwrap();
        assert!(matrix1.is_square());

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square_random(0);
        assert!(matrix2.is_err());

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square_random(10);
        assert!(matrix2.is_ok());
    }
}
