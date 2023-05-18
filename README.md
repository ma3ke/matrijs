# Matr<sub><i>ij</i></sub>s 🔢

A small 2D f64 matrix library.

There are many like it, but this one is mine.

_Note: The example below can be found in `examples/basic.rs`._

```rust
// The matrix! macro allows for quick initialization.

// m = | 0.0  1.0 |
//     |-1.0  0.0 |
let mut m = matrix![0.0, 1.0; -1.0, 0.0];

// Scalar math.
m += 1.0;
m *= -10.0;

// You can also create a Matrix manually.
let m_expected = Matrix::new(2, 2, &[-10.0, -20.0, 0.0, -10.0]);
assert_eq!(m, m_expected);

// a = | 0.0  1.0 |
//     | 2.0  3.0 |
// b = | 4.0  5.0  6.0 |
//     | 7.0  8.0  9.0 |
let a = matrix![0.0, 1.0; 2.0, 3.0];
let b = matrix![4.0, 5.0, 6.0; 7.0, 8.0, 9.0];

// The dot product of `i` and `a` should be equal to `a` (idempotence).
let i = Matrix::identity(2);
assert_eq!(i.dot(&a), a);

assert_eq!(a.dot(&b), matrix![7.0, 8.0, 9.0; 29.0, 34.0, 39.0]);

// You can append rows and columns to expand what you're working with.
let mut ones = Matrix::one(2, 2);
ones.append_row(matrix![0.0, 0.0].array());

assert_eq!(
    ones,
    matrix![
        1.0, 1.0;
        1.0, 1.0;
        0.0, 0.0
    ]
);

// When in doubt, take a look at the shape of the matrix.
assert_eq!(ones.shape(), (3, 2)) // 3 rows, 2 columns
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
