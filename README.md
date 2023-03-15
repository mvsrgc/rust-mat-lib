# rust-mat-lib

rust-mat-lib provides a simple, generic implementation of matrices in Rust with support for both row-major and column-major storage orders. It offers basic operations like creating new matrices, setting identity matrices, and reading matrices from CSV files.

## Features

- Generic matrix implementation
- Support for both row-major and column-major storage orders
- Basic matrix operations
  - Create a new matrix
  - Set an identity matrix
  - Check if a matrix is square
  - Transpose a matrix (not yet implemented)
  - Access matrix elements
- Read matrices from CSV files

## Usage

Add the library to your project by including it in your `Cargo.toml` file.

```toml
[dependencies]
rust-mat-lib = "soon"
```

Import the library and its traits in your Rust source file.

```rust
use matrix_lib::{Matrix, Order, RowMajor, ColMajor};
use serde::Deserialize;
use std::fs::File;
```

### Create a new matrix

```rust
let matrix: Matrix<f64, RowMajor> = Matrix::new(3, 3).unwrap();
```

### Set an identity matrix

```rust
let mut matrix: Matrix<f64, RowMajor> = Matrix::new(3, 3).unwrap();
matrix.set_identity().unwrap();
```

### Check if a matrix is square

```rust
let matrix: Matrix<f64, RowMajor> = Matrix::new(3, 3).unwrap();
assert!(matrix.is_square());
```

### Read a matrix from a CSV file

```rust
let mut file = File::open("path/to/your/csv/file.csv").unwrap();
let matrix = Matrix::<f64, RowMajor>::from_file(&mut file).unwrap();
```

## Example

The following example shows how to use the library to create a new matrix, set it as an identity matrix, and access its elements.

```rust
use matrix_lib::{Matrix, Order, RowMajor, ColMajor};
use std::fs::File;

fn main() {
    // Create a new 3x3 matrix with row-major order
    let mut matrix: Matrix<f64, RowMajor> = Matrix::new(3, 3).unwrap();

    // Set the matrix as an identity matrix
    matrix.set_identity().unwrap();

    // Access the elements of the matrix
    println!("Element at (0, 1): {}", matrix[(0, 1)]);
    println!("Element at (1, 0): {}", matrix[(1, 0)]);
    println!("Element at (0, 0): {}", matrix[(0, 0)]);
    println!("Element at (1, 1): {}", matrix[(1, 1)]);
}
```

## License

This library is available under the [MIT License](https://opensource.org/licenses/MIT).
