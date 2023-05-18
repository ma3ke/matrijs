# Matr<sub><i>ij</i></sub>s 🔢

A small 2D f64 matrix library.

There are many like it, but this one is mine.

```rust
use matrijs::Matrix;

// m = | 0.0  1.0 |
//     |-1.0  0.0 |
let mut m = Matrix::new(2, 2, &[0.0, 1.0,  -1.0, 0.0]);
m += 1.0;
assert_eq!(m, Matrix::new(2, 2, &[1.0, 2.0,  0.0, 1.0]));

// a = | 0.0  1.0 |
//     | 2.0  3.0 |
// b = | 4.0  5.0  6.0 |
//     | 7.0  8.0  9.0 |
let a = Matrix::new(2, 2, &[0.0, 1.0, 2.0, 3.0]);
let b = Matrix::new(2, 3, &[4.0, 5.0, 6.0,  7.0, 8.0, 9.0]);

// Multiplication of `i` and `a` should be idempotent.
let i = Matrix::identity(2);
assert_eq!(i.dot(&a), a);

assert_eq!(a.dot(&b), Matrix::new(2, 3, &[7.0, 8.0, 9.0,  29.0, 34.0, 39.0]));
```

## Abilities

- Creation
    - From array: `Matrix::new(3, 2, &[0.0, 1.0,  2.0, 3.0,  4.0, 5.0])`
    - With value: `Matrix::with_value(3, 16.1)`
    - Zeros: `Matrix::zero(3)`
    - Ones: `Matrix::one(3)`
    - Identity: `Matrix::identity(3)`
    - Diagonal: `Matrix::diagonal(&[1.0, 3.0, 1.0, 2.0])`
- Operations
    - Transpose
        - In place: `m.transpose();`
        - By value: `m.t()`
    - Scalar operations
        - Addition, subtraction, multiplication, division: `b = a + 1.0`, `b *= 2`
    - Matrix-matrix operations
        - Entry-by-entry addition, subtraction, multiplication, division: `a + b`
        - Dot product: `a.dot(&b)`

## Implementation

The internal data structure is a `Vec` of entries, row after row.
That means that this is a _row-major_ implementation.

## Name

In Dutch, there is a word _matrijs_ (pronounce _mat-rice_) which has a common ancestor with the word _matrix_.
_Matrijs_ refers to molds or stamps, often when laid out in arrays.

I like the name for this library, because it contains the 'ij' digraph, which is very similar to the letters _i_ and _j_ as seen in notation for entries in a matrix, such as _a<sub>ij</sub>_.
