use super::*;
use crate::vector::*;

impl<T, const R: usize, const C: usize> Matrix<T, R, C>
where
    T: Copy,
{
    pub fn from_row_vectors(rows: [Vector<T, C>; R]) -> Matrix<T, R, C> {
        Matrix::build(|row, column| rows[row][column])
    }

    pub fn from_column_vectors(columns: [Vector<T, R>; C]) -> Matrix<T, R, C> {
        Matrix::build(|row, column| columns[column][row])
    }
}
