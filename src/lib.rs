use std::fmt::Debug;
use std::marker::PhantomData;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use num_traits::one;

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

struct Dimensions {
    rows: usize,
    cols: usize,
}

struct Matrix<T, Order> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
    _order: PhantomData<Order>,
}

impl<T: Default + Copy + FromStr + Debug, O: Order> Matrix<T, O> {
    pub fn new(num_rows: usize, num_cols: usize) -> Result<Self, String> {
        if num_rows * num_cols == 0 {
            return Err("Number of rows or number of columns cannot be 0.".to_string());
        }

        let data = vec![T::default(); num_rows * num_cols];

        Ok(Self {
            num_rows,
            num_cols,
            data,
            _order: PhantomData,
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

    pub fn dims(&self) -> Dimensions {
        Dimensions { rows: self.num_rows, cols: self.num_cols }
    }

    pub fn from_file(file: &mut File) -> Result<Self, String> {
        let mut reader = BufReader::new(file);

        let mut num_rows = 0;
        let mut num_cols = 0;
        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap_or_default();
            let values: Vec<&str> = line.split_whitespace().collect();
            if i == 0 {
                match (values.get(0), values.get(1)) {
                    (Some(rows), Some(cols)) => {
                        num_rows = usize::from_str(rows).unwrap_or_default();
                        num_cols = usize::from_str(cols).unwrap_or_default();
                        continue;
                    }
                    _ => return Err("File is empty or has invalid format.".to_string()),
                }
            }
        }

        Ok(Self {
            num_rows,
            num_cols,
            data: vec![T::default(); 1],
            _order: PhantomData,
        })
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
    use std::path::PathBuf;
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

    #[test]
    fn identity() {
        let mut m1: Matrix<usize, ColMajor> = Matrix::new(2, 2).unwrap();
        m1.set_identity().unwrap();
        assert_eq!(m1[(0, 1)], usize::default());
        assert_eq!(m1[(1, 0)], usize::default());
        assert_eq!(m1[(0, 0)], 1);
        assert_eq!(m1[(1, 1)], 1);
        assert_eq!(m1.dims().rows, 2);
        assert_eq!(m1.dims().cols, 2);

        let mut m1: Matrix<usize, ColMajor> = Matrix::new(3, 2).unwrap();
        assert!(m1.set_identity().is_err());

        let mut m1: Matrix<usize, ColMajor> = Matrix::new(2, 2).unwrap();
        assert!(m1.set_identity().is_ok())
    }

    #[test]
    fn is_square() {
        let m1: Matrix<usize, ColMajor> = Matrix::new(3, 3).unwrap();
        assert!(m1.is_square());

        let m1: Matrix<usize, ColMajor> = Matrix::new(3, 2).unwrap();
        assert!(!m1.is_square());
    }

    #[test]
    fn from_file() {
        let path = PathBuf::from("data/input.txt");
        let mut file = File::open(path).unwrap();

        let result: Matrix<f64, ColMajor> = Matrix::from_file(&mut file).unwrap();

        assert_eq!(result.dims().rows, 4);
        assert_eq!(result.dims().cols, 5);
    }
}