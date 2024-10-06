use std::ops;

/// represents a generic matrix
/// I is rows, J is columns
pub struct Matrix<const I: usize, const J: usize> {
    pub values: [[f32; J]; I],
}

impl<const I: usize, const J: usize> Matrix<I, J> {
    pub fn new(values: [[f32; J]; I]) -> Self {
        Self { values }
    }
    pub fn identity() -> Option<Self> {
        if I != J {
            return None;
        }
        let mut matrix = Self::new([[0.0; J]; I]);
        for i in 0..I {
            let _ = matrix.set((i, i), 1.0);
        }
        return Some(matrix);
    }
}

impl<const I: usize, const J: usize> Matrix<I, J> {
    pub fn get_size(&self) -> (usize, usize) {
        (I, J)
    }
    pub fn get(&self, ij: (usize, usize)) -> Option<f32> {
        self.values.get(ij.0)?.get(ij.1).copied()
    }
    pub fn set(&mut self, ij: (usize, usize), value: f32) -> Result<(), ()> {
        if ij.0 >= I || ij.1 >= J {
            return Err(());
        }

        self.values[ij.0][ij.1] = value;
        return Ok(());
    }

    pub fn matrix_multiply<const J2: usize>(&self, rhs: Matrix<J, J2>) -> Matrix<I, J2> {
        let mut m = Matrix::new([[0.0; J2]; I]);
        for i in 0..I {
            for j in 0..J2 {
                let mut total = 0.0;
                for js in 0..J {
                    total += self.get((i, js)).unwrap() * rhs.get((js, j)).unwrap();
                }
                let _ = m.set((i, j), total);
            }
        }
        return m;
    }

    pub fn scalar_multiply(mut self, rhs: f32) -> Self {
        for i in 0..I {
            for j in 0..J {
                _ = self.set((i, j), self.get((i, j)).unwrap() * rhs);
            }
        }

        return self;
    }
}

impl<const I: usize, const J: usize> Clone for Matrix<I, J> {
    fn clone(&self) -> Self {
        Matrix {
            values: self.values.clone(),
        }
    }
}

impl<const I: usize, const J: usize> ops::Add for Matrix<I, J> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..I {
            for j in 0..J {
                _ = self.set((i, j), self.get((i, j)).unwrap() + rhs.get((i, j)).unwrap());
            }
        }

        return self;
    }
}
impl<const I: usize, const J: usize> ops::Sub for Matrix<I, J> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..I {
            for j in 0..J {
                _ = self.set((i, j), self.get((i, j)).unwrap() - rhs.get((i, j)).unwrap());
            }
        }

        return self;
    }
}
impl<const I: usize, const J: usize, const J2: usize> ops::Mul<Matrix<J, J2>> for Matrix<I, J> {
    type Output = Matrix<I, J2>;

    fn mul(self, rhs: Matrix<J, J2>) -> Self::Output {
        self.matrix_multiply(rhs)
    }
}
impl<const I: usize, const J: usize> ops::Mul<f32> for Matrix<I, J> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scalar_multiply(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let mut my_matrix = Matrix::new([[1.0, 5.0], [4.0, 2.0]]);

        let _ = my_matrix.set((1, 0), 10.0).unwrap();

        let other = Matrix::new([[1.0, 5.0], [10.0, 2.0]]);

        assert_eq!(my_matrix.values, other.values);
    }

    #[test]
    fn add() {
        let mut my_matrix = Matrix::<3, 3>::identity().unwrap();

        my_matrix = my_matrix + Matrix::<3, 3>::identity().unwrap();

        assert_eq!(
            my_matrix.values,
            [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]]
        );
    }

    #[test]
    fn matrix_multiply() {
        let matrix = Matrix::new([[4.0, 5.0, 7.0], [2.0, 1.0, 0.0]]);
        let other = Matrix::new([[2.0, 3.0], [8.0, 9.0], [1.0, 1.0]]);
        let result = Matrix::new([[55.0, 64.0], [12.0, 15.0]]);

        let matrix: Matrix<2, 2> = matrix * other;

        assert_eq!(matrix.values, result.values);

        let result = matrix.clone() * Matrix::identity().unwrap();

        assert_eq!(matrix.values, result.values)
    }

    #[test]
    fn scalar_multiply() {
        let matrix = Matrix::new([[4.0, 5.0, 7.0], [2.0, 1.0, 0.0]]);

        let result = matrix * 2.0;

        assert_eq!(result.values, [[8.0, 10.0, 14.0], [4.0, 2.0, 0.0]]);
    }
}
