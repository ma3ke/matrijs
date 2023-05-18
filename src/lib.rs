use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

type F = f32;

#[derive(Debug, Clone, PartialEq)]
/// A 2-dimensional matrix.
///
/// # Example
///
/// ```
/// use matrijs::Matrix;
///
/// let mut m = Matrix::new(2, 2, &[0.0, 1.0, -1.0, 0.0]);
///
/// m += 1.0;
/// assert_eq!(m, Matrix::new(2, 2, &[1.0, 2.0, 0.0, 1.0]));
/// ```
///
/// # Note
///
/// The implementation is row-major, at the moment.
pub struct Matrix {
    cols: usize,
    rows: usize,
    array: Vec<F>, // length == cols * rows
}

impl Matrix {
    /// Returns the cols of this [`Matrix`].
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the rows of this [`Matrix`].
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns a reference to the internal array of this [`Matrix`].
    pub fn array(&self) -> &[F] {
        &self.array
    }

    /// Returns the shape of this [`Matrix`] as a tuple of (`cols`, `rows`).
    pub fn shape(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }
}

impl Matrix {
    /// Get a slice to the `index`th row.
    ///
    /// # Panics
    ///
    /// If `index` >= `rows`, this function will panic.
    pub fn row(&self, index: usize) -> &[F] {
        &self.array[index * self.cols..(index + 1) * self.cols]
    }

    /// Get a mutable slice to the `index`th row.
    ///
    /// # Panics
    ///
    /// If `index` >= `rows`, this function will panic.
    pub fn row_mut(&mut self, index: usize) -> &mut [F] {
        &mut self.array[index * self.cols..(index + 1) * self.cols]
    }

    /// Get an owned Vec of the `index`th column.
    ///
    /// # Panics
    ///
    /// If `index` >= `cols`, this function will panic.
    pub fn col(&self, index: usize) -> Vec<F> {
        let mut col = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            col.push(self[(i, index)])
        }

        col
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, array: &[F]) -> Self {
        assert_eq!(
            array.len(),
            cols * rows,
            "The length of array must be equal to cols * rows"
        );

        Self {
            cols,
            rows,
            array: array.to_vec(),
        }
    }

    pub fn with_value(rows: usize, cols: usize, value: F) -> Self {
        Self {
            cols,
            rows,
            array: vec![value; cols * rows],
        }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self::with_value(rows, cols, 0.0)
    }

    pub fn one(rows: usize, cols: usize) -> Self {
        Self::with_value(rows, cols, 1.0)
    }

    pub fn identity(size: usize) -> Self {
        let mut i = Self::zero(size, size);
        for index in 0..size {
            i[(index, index)] = 1.0
        }

        i
    }

    pub fn diagonal(array: &[F]) -> Self {
        let size = array.len();

        let mut d = Self::zero(size, size);
        for (index, value) in (0..size).zip(array) {
            d[(index, index)] = *value
        }

        d
    }
}

impl Matrix {
    /// Transpose a [`Matrix`] in place.
    pub fn transpose(&mut self) {
        let mut new_array = Vec::with_capacity(self.rows * self.cols);
        for col in (0..self.cols).map(|j| self.col(j)) {
            new_array.extend_from_slice(&col)
        }
        (self.cols, self.rows) = (self.rows, self.cols);
        self.array = new_array;
    }

    /// Return a transposed [`Matrix`]. This is done by cloning the original and returning the
    /// transposed clone.
    pub fn t(&self) -> Self {
        let mut m = self.clone();
        m.transpose();

        m
    }
}

impl Matrix {
    // TODO: Maybe I could simply do a transpose followed by append_row then transpose again
    // instead. But that might be slower because I would need to do a lot of shuffling until I
    // implement a more efficient way of transposing in place.
    pub fn append_col(&mut self, col: &[F]) {
        assert_eq!(
            col.len(),
            self.rows,
            "The length of col array must be equal to the number of rows"
        );

        let new_cols = self.cols + 1;

        let mut new_array = Vec::with_capacity(new_cols * self.rows);
        for i in 0..self.rows {
            let old_row = self.row(i);
            new_array.extend_from_slice(old_row);
            new_array.push(col[i])
        }

        self.cols += 1;
        self.array = new_array;

        debug_assert_eq!(self.array.len(), self.cols * self.rows);
    }

    pub fn append_row(&mut self, row: &[F]) {
        assert_eq!(
            row.len(),
            self.cols,
            "The length of row array must be equal to the number of columns"
        );

        self.rows += 1;
        self.array.extend_from_slice(row);

        debug_assert_eq!(self.array.len(), self.cols * self.rows);
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = F;

    /// Get element by `(row, col)`.
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        &self.row(row)[col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    /// Get mutable element by `(row, col)`.
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        &mut self.row_mut(row)[col]
    }
}

/* scalar math */

impl Add<F> for Matrix {
    type Output = Self;

    fn add(mut self, rhs: F) -> Self::Output {
        self.array.iter_mut().for_each(|elem| *elem += rhs);
        self
    }
}

impl Sub<F> for Matrix {
    type Output = Self;

    fn sub(mut self, rhs: F) -> Self::Output {
        self.array.iter_mut().for_each(|elem| *elem -= rhs);
        self
    }
}

impl Mul<F> for Matrix {
    type Output = Self;

    fn mul(mut self, rhs: F) -> Self::Output {
        self.array.iter_mut().for_each(|elem| *elem *= rhs);
        self
    }
}

impl Div<F> for Matrix {
    type Output = Self;

    fn div(mut self, rhs: F) -> Self::Output {
        self.array.iter_mut().for_each(|elem| *elem /= rhs);
        self
    }
}

impl AddAssign<F> for Matrix {
    fn add_assign(&mut self, rhs: F) {
        self.array.iter_mut().for_each(|elem| *elem += rhs)
    }
}

impl SubAssign<F> for Matrix {
    fn sub_assign(&mut self, rhs: F) {
        self.array.iter_mut().for_each(|elem| *elem -= rhs)
    }
}

impl MulAssign<F> for Matrix {
    fn mul_assign(&mut self, rhs: F) {
        self.array.iter_mut().for_each(|elem| *elem *= rhs)
    }
}

impl DivAssign<F> for Matrix {
    fn div_assign(&mut self, rhs: F) {
        self.array.iter_mut().for_each(|elem| *elem /= rhs)
    }
}

/* matrix operations */

impl Add for Matrix {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        // FIXME: Check sizes.
        self.array
            .iter_mut()
            .zip(rhs.array())
            .for_each(|(a, b)| *a += b);

        return self;
    }
}

impl Sub for Matrix {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        // FIXME: Check sizes.
        self.array
            .iter_mut()
            .zip(rhs.array())
            .for_each(|(a, b)| *a -= b);

        return self;
    }
}

impl Div for Matrix {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        // FIXME: Check sizes.
        self.array
            .iter_mut()
            .zip(rhs.array())
            .for_each(|(a, b)| *a /= b);

        return self;
    }
}

// TODO: Is there a way to make this less error-prone?
impl Mul for Matrix {
    type Output = Self;

    /// # Note
    ///
    /// This is a element by element multiplication, not a dot product or cross product.
    fn mul(mut self, rhs: Self) -> Self::Output {
        // FIXME: Check sizes.
        self.array
            .iter_mut()
            .zip(rhs.array())
            .for_each(|(a, b)| *a *= b);

        return self;
    }
}

impl Matrix {
    /// The dot product between two matrices.
    ///
    /// From m × n matrix A and n × p matrix B, we can calculate the dot product AB = C where C
    /// becomes an m × p matrix. The following approach is used.
    ///
    /// ```txt
    /// c_ij = a_i1 * b_1j + ... + a_in * b_nj
    ///
    ///         n
    /// c_ij =  Σ  a_ik * b_kj
    ///        k=1
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use matrijs::Matrix;
    ///
    /// let a = Matrix::new(2, 2, &[0.0, 1.0, 2.0, 3.0]);
    /// let b = Matrix::new(2, 3, &[4.0, 5.0, 6.0,  7.0, 8.0, 9.0]);
    ///
    /// // Multiplication of `i` and `a` should be idempotent.
    /// let i = Matrix::identity(2);
    /// assert_eq!(i.clone().dot(a.clone()), a);
    ///
    /// assert_eq!(a.dot(b), Matrix::new(2, 3, &[7.0, 8.0, 9.0,  29.0, 34.0, 39.0]));
    /// ```
    ///
    /// # Panics
    ///
    /// Of course, the dot product can only be calculated for matrices where the 'inner' size is
    /// the same (i.e., n in `m × n dot n × p`).
    ///
    /// ```should_panic
    /// # use matrijs::Matrix;
    /// let e = Matrix::one(3, 4);
    /// let f = Matrix::one(3, 2);
    ///
    /// e.dot(f);
    /// ```
    pub fn dot(self, rhs: Self) -> Self {
        assert_eq!(self.cols, rhs.rows);

        let m = self.rows;
        let n = self.cols;
        let p = rhs.cols;

        let a = self;
        let b = rhs;
        let mut c = Matrix::zero(m, p);

        for j in 0..p {
            for i in 0..m {
                let c_ij = &mut c[(i, j)];
                for k in 0..n {
                    let a_ik = a[(i, k)];
                    let b_kj = b[(k, j)];
                    *c_ij += a_ik * b_kj;
                }
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let _m = Matrix::new(2, 3, &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        let _o = Matrix::zero(2, 3);
        let _l = Matrix::one(2, 3);
        let _v = Matrix::with_value(2, 3, std::f32::consts::PI);
        let _i = Matrix::identity(2);
        let _d = Matrix::diagonal(&[0.0, 1.0, 2.0, 3.0]);
    }

    #[test]
    fn zero() {
        let o = Matrix::zero(2, 2);
        let arr = &[0.0; 2 * 2];
        let manual_o = Matrix::new(2, 2, arr);

        assert_eq!(o, manual_o)
    }

    #[test]
    fn one() {
        let l = Matrix::one(2, 2);
        let arr = &[1.0; 2 * 2];
        let manual_l = Matrix::new(2, 2, arr);

        assert_eq!(l, manual_l)
    }

    #[test]
    #[should_panic]
    fn creation_too_long() {
        // arr has 6 elements, not 2 * 2 == 4. Creating a 2 by 2 matrix from arr should thus panic.
        let arr = &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let _m = Matrix::new(2, 2, arr);
    }

    #[test]
    fn identity() {
        let i = Matrix::identity(3);

        #[rustfmt::skip]
        let arr = &[
            1., 0., 0.,
            0., 1., 0.,
            0., 0., 1.,
        ];
        let manual_i = Matrix::new(3, 3, arr);

        assert_eq!(i, manual_i)
    }

    #[test]
    fn diagonal() {
        let d = Matrix::diagonal(&[1.0, 6.0, 1.0]);

        #[rustfmt::skip]
        let arr = &[
            1., 0., 0.,
            0., 6., 0.,
            0., 0., 1.,
        ];
        let manual_d = Matrix::new(3, 3, arr);

        assert_eq!(d, manual_d)
    }

    #[test]
    fn transpose() {
        #[rustfmt::skip]
        let arr = &[
            0., 1., 2.,
            3., 4., 5.,
            6., 7., 8.,
        ];
        let m = Matrix::new(3, 3, arr);

        #[rustfmt::skip]
        let arr_t = &[
            0., 3., 6.,
            1., 4., 7.,
            2., 5., 8.,
        ];
        let m_t_manual = Matrix::new(3, 3, arr_t);

        assert_eq!(m.t(), m_t_manual);

        // Check that m has not changed due to the previous m.t().
        assert_ne!(m, m_t_manual);
        // Make m mutable so we can transpose it in place.
        let mut m = m;
        m.transpose();
        // m should be transposed, now.
        assert_eq!(m, m_t_manual);
    }

    #[test]
    fn grow_col() {
        let mut m = Matrix::one(3, 2);
        let col = [0.0; 3];
        m.append_col(&col);

        #[rustfmt::skip]
        let arr = &[
            1., 1., 0.,
            1., 1., 0.,
            1., 1., 0.,
        ];
        let manual_m = Matrix::new(3, 3, arr);
        assert_eq!(m, manual_m)
    }

    #[test]
    fn grow_row() {
        let mut m = Matrix::one(2, 3);
        let row = [0.0; 3];
        m.append_row(&row);

        #[rustfmt::skip]
        let arr = &[
            1., 1., 1.,
            1., 1., 1.,
            0., 0., 0.,
        ];
        let manual_m = Matrix::new(3, 3, arr);
        assert_eq!(m, manual_m)
    }

    #[test]
    fn grow_col_transposed() {
        let mut m = Matrix::one(3, 2);
        let col = [0.0; 3];
        // Transpose it so we can grow a row onto m.
        m.transpose();
        m.append_row(&col);
        // Transpose m again. A column should now have been tacked on.
        m.transpose();

        #[rustfmt::skip]
        let arr = &[
            1., 1., 0.,
            1., 1., 0.,
            1., 1., 0.,
        ];
        let manual_m = Matrix::new(3, 3, arr);
        assert_eq!(m, manual_m)
    }
}
