use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::marker::PhantomData;
use std::cmp;
use num_traits;

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

impl<T: Default + Copy, O: Order> Matrix<T, O> {
    #[inline]
    pub fn from_blind_fn<F: FnMut() -> T>(
        num_rows: usize,
        num_cols: usize,
        f: F,
    ) -> Result<Self, String> {
        if num_rows == 0 {
            return Err("Number of rows cannot be less than or equal to 0.".to_string());
        };

        if num_cols == 0 {
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
        Self::from_blind_fn(num_rows, num_cols, || elem)
    }

    pub fn from_vec(num_rows: usize, num_cols: usize, vec: Vec<T>) -> Result<Self, String>
    {
        if vec.len() != num_rows * num_cols {
            return Err("Input vector length does not match matrix dimensions.".to_string());
        }

        let mut cloned_vec = vec;

        Self::from_blind_fn(num_rows, num_cols, || cloned_vec.remove(0))
    }

    pub fn make_eye(&mut self) -> Result<(), String>
        where
            T: num_traits::One,
    {
        if !self.is_square() {
            return Err("Can't make non-square matrix an identity matrix.".to_string());
        }

        for i in 0..self.num_rows {
            self[(i, i)] = T::one();
        }

        Ok(())
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
    fn from_blind_fn() {
        let m1: Matrix<usize, ColMajor> = Matrix::from_blind_fn(2, 3, || 0).unwrap();

        assert_eq!(m1[(0, 0)], 0);
        assert_eq!(m1[(0, 1)], 0);
        assert_eq!(m1[(1, 0)], 0);
        assert_eq!(m1[(1, 1)], 0);

        let m1: Matrix<usize, ColMajor> = Matrix::from_blind_fn(3, 2, || 2).unwrap();

        assert_eq!(m1[(0, 0)], 2);
        assert_eq!(m1[(0, 1)], 2);
        assert_eq!(m1[(1, 0)], 2);
        assert_eq!(m1[(1, 1)], 2);
        assert_eq!(m1[(2, 0)], 2);
        assert_eq!(m1[(2, 1)], 2);

        let m1 = Matrix::<usize, RowMajor>::from_blind_fn(0, 3, || 2);

        assert!(m1.is_err());
    }

    #[test]
    fn from_elem() {
        let m1: Matrix<usize, ColMajor> = Matrix::from_elem(3, 3, 5).unwrap();

        for i in 0..m1.num_rows {
            for j in 0..m1.num_cols {
                assert_eq!(m1[(i, j)], 5);
            }
        }
    }

    #[test]
    fn test_from_vec() {
        let input_vec = vec![1, 2, 3, 4, 5, 6];

        let m1: Matrix<usize, ColMajor> = Matrix::from_vec(2, 3, input_vec).unwrap();

        assert_eq!(m1[(0, 0)], 1);
        assert_eq!(m1[(0, 1)], 2);
        assert_eq!(m1[(0, 2)], 3);
        assert_eq!(m1[(1, 0)], 4);
        assert_eq!(m1[(1, 1)], 5);
        assert_eq!(m1[(1, 2)], 6);

        let input_vec: Vec<usize> = vec![1, 2, 3, 4, 5];

        let m1: Result<Matrix<usize, ColMajor>, String> = Matrix::from_vec(2, 3, input_vec);

        assert!(m1.is_err());
        assert_eq!(
            m1.err().unwrap(),
            "Input vector length does not match matrix dimensions."
        );
    }

    #[test]
    fn make_matrix() {
        let m1: Matrix<usize, ColMajor> = Matrix::default(2, 3).unwrap();

        assert_eq!(m1.num_rows, 2);
        assert_eq!(m1.num_cols, 3);
        assert_eq!(m1.data, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(m1.data.len(), 6);
        assert!(!m1.is_square());

        assert!(Matrix::<usize, ColMajor>::default(0, 0).is_err());
    }

    #[test]
    fn random_is_filled() {
        let m1: Matrix<f32, ColMajor> = Matrix::random(3, 3).unwrap();
        let matrix2: Matrix<usize, ColMajor> = Matrix::random(3, 3).unwrap();
        let matrix3: Matrix<i8, ColMajor> = Matrix::random(3, 3).unwrap();
        assert!(m1.data.iter().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().any(|&x| x != 0));
        assert!(matrix3.data.iter().any(|&x| x != 0));

        let m1: Matrix<f32, ColMajor> = Matrix::square_random(3).unwrap();
        let matrix2: Matrix<usize, ColMajor> = Matrix::square_random(3).unwrap();
        let matrix3: Matrix<i8, ColMajor> = Matrix::square_random(3).unwrap();
        assert!(m1.data.iter().any(|&x| x != 0.0));
        assert!(matrix2.data.iter().any(|&x| x != 0));
        assert!(matrix3.data.iter().any(|&x| x != 0));
    }

    #[test]
    fn square() {
        let m1: Matrix<usize, ColMajor> = Matrix::square(3).unwrap();
        assert!(m1.is_square());

        let m2: Result<Matrix<usize, ColMajor>, String> = Matrix::square(0);
        assert!(m2.is_err());

        let m2: Result<Matrix<usize, ColMajor>, String> = Matrix::square(10);
        assert!(m2.is_ok());
    }

    #[test]
    fn square_random() {
        let m1: Matrix<usize, ColMajor> = Matrix::square_random(3).unwrap();
        assert!(m1.is_square());
        assert_eq!(m1.num_rows, 3);
        assert_eq!(m1.num_cols, 3);

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square_random(0);
        assert!(matrix2.is_err());

        let matrix2: Result<Matrix<usize, ColMajor>, String> = Matrix::square_random(10);
        assert!(matrix2.is_ok());
    }
}
