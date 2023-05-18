use matrijs::{matrix, Matrix};

// From the README.
fn main() {
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
}
