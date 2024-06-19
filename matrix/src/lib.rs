pub mod ops;
pub mod mult;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl<T: Scalar<Item=T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Self(
            vec![
                vec![T::zero()]
            ]
        )
    }
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Self(
            vec![vec![T::zero(); col]; row]
        )
    }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Self::zero(n, n);
        for i in 0..m.0.len() {
            m.0[i][i] = T::one()
        }
        m
    }
}