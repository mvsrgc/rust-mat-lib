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
    pub fn new(num_rows: usize, num_cols: usize) -> Result<Self, String> {
        if num_rows * num_cols == 0 {
            return Err("Number of rows or number of columns cannot be 0.".to_string());
        }

        let data = vec![T::default(); num_rows * num_cols];

        Ok(Self {
            num_rows,
            num_cols,
            data,
            _order: PhantomData
        })
    }
    pub fn set_identity(&mut self) -> Result<(), String>
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

    pub fn transpose(&self) -> Result<Self, String> {
        std::unimplemented!()
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
        let m1: Matrix<usize, ColMajor> = Matrix::new(2, 3).unwrap();

        assert_eq!(m1.num_rows, 2);
        assert_eq!(m1.num_cols, 3);
        assert_eq!(m1.data, vec![0, 0, 0, 0, 0, 0]);
        assert_eq!(m1.data.len(), 6);
        assert!(!m1.is_square());

        assert!(Matrix::<usize, ColMajor>::new(0, 0).is_err());
    }
}
