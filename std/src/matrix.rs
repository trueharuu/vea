use std::{ ops::{ Add, Sub, Mul, Not, Index, IndexMut, FnMut }, slice::SliceIndex };

#[derive(Clone)]
pub struct Matrix<T, const N: usize, const M: usize>([[T; M]; N]);

impl<T, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new(values: [[T; M]; N]) -> Self {
        Self(values)
    }

    pub const fn size(&self) -> usize {
        N * M
    }

    pub const fn rows(&self) -> usize {
        N
    }

    pub const fn columns(&self) -> usize {
        M
    }

    pub fn transpose(&self) -> Matrix<T, M, N> where T: Default + Copy {
        !self.clone()
    }

    pub fn multiply<U>(&self, other: &Matrix<T, M, N>)
        where T: Mul<Output = U> + Copy, U: Default + Copy + Add<Output = U>
    {
        let ar = self.rows();
        let ac = self.columns();
        let bc = other.columns();
        let mut m = [[U::default(); M]; N];

        for r in 0..ar {
            for c in 0..bc {
                for i in 0..ac {
                    m[r][c] = m[r][c] + self[r][i] * other[i][c];
                }
            }
        }
    }

    pub fn swap_row(&mut self, i: usize, j: usize) -> &mut Self where T: Clone {
        (self[i], self[j]) = (self[j].clone(), self[i].clone());
        self
    }

    pub fn row_mul<U>(&mut self, i: usize, by: U) -> &mut Self
        where T: Clone + Mul<U, Output = T>, U: Clone
    {
        for j in 0..M {
            self[i][j] = self[i][j].clone() * by.clone();
        }

        self
    }

    pub fn submatrix(&self, x: usize, y: usize) -> Matrix<T, { N - 1 }, { M - 1 }>
        where T: Default + Copy
    {
        assert!(x < N);
        assert!(y < M);
        let mut m = [[T::default(); M - 1]; N - 1];

        for i in 0..N {
            for j in 0..M {
                m[i - (if i >= y { 1 } else { 0 })][j - (if j >= y { 1 } else { 0 })] = self[i][j];
            }
        }

        Matrix(m)
    }

    pub fn identity<const P: usize>(zero: T, one: T) -> Matrix<T, P, P> where T: Copy {
        let mut m = [[zero; P]; P];

        for i in 0..P {
            m[i][i] = one;
        }

        Matrix(m)
    }

    pub fn map<U, F>(&self, f: F) -> Matrix<U, N, M> where F: FnMut(T) -> U + Copy, T: Clone {
        Matrix(self.0.clone().map(|x| x.map(f)))
    }

    pub fn to_n(&self) -> [T; N * M] where T: Copy {
        let mut out = [None; N * M];
        for i in 0..N {
            for j in 0..M {
                out[i * N + j] = Some(self[i][j]);
            }
        }

        out.map(|x| x.unwrap())
    }

    pub fn get_n(&self, n: usize) -> T where T: Clone {
        let d = n / N;
        let m = n % N;
        self[d][m].clone()
    }
}

impl<T, U, R, const N: usize, const M: usize> Add<Matrix<U, N, M>>
    for Matrix<T, N, M>
    where T: Add<U, Output = R> + Clone, U: Clone, R: Copy
{
    type Output = Matrix<R, N, M>;
    fn add(self, rhs: Matrix<U, N, M>) -> Self::Output {
        let mut buf = [[None; M]; N];
        for i in 0..N {
            for j in 0..M {
                buf[i][j] = Some(self.0[i][j].clone() + rhs.0[i][j].clone());
            }
        }

        Matrix(buf).map(|x| x.unwrap())
    }
}

impl<T, U, R, const N: usize, const M: usize> Sub<Matrix<U, N, M>>
    for Matrix<T, N, M>
    where T: Sub<U, Output = R> + Clone, U: Clone, R: Copy
{
    type Output = Matrix<R, N, M>;
    fn sub(self, rhs: Matrix<U, N, M>) -> Self::Output {
        let mut buf = [[None; M]; N];
        for i in 0..N {
            for j in 0..M {
                buf[i][j] = Some(self.0[i][j].clone() - rhs.0[i][j].clone());
            }
        }

        Matrix(buf).map(|x| x.unwrap())
    }
}

impl<T, U, R, const N: usize, const M: usize> Mul<U>
    for Matrix<T, N, M>
    where T: Mul<U, Output = R> + Clone, U: Clone, R: Copy
{
    type Output = Matrix<R, N, M>;
    fn mul(self, rhs: U) -> Self::Output {
        let mut buf = [[None; M]; N];
        for i in 0..N {
            for j in 0..M {
                buf[i][j] = Some(self.0[i][j].clone() * rhs.clone());
            }
        }

        Matrix(buf).map(|x| x.unwrap())
    }
}

impl<T, const N: usize, const M: usize> Not for Matrix<T, N, M> where T: Copy {
    type Output = Matrix<T, M, N>;
    fn not(self) -> Self::Output {
        let mut buf = [[None; N]; M];
        for i in 0..N {
            for j in 0..M {
                buf[j][i] = Some(self.0[i][j].clone());
            }
        }

        Matrix(buf).map(|x| x.unwrap())
    }
}

impl<T, U, const N: usize, const M: usize> Index<U>
    for Matrix<T, N, M>
    where U: SliceIndex<[[T; M]]>
{
    type Output = U::Output;
    fn index(&self, index: U) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, U, const N: usize, const M: usize> IndexMut<U>
    for Matrix<T, N, M>
    where U: SliceIndex<[[T; M]]>
{
    fn index_mut(&mut self, index: U) -> &mut <U as SliceIndex<[[T; M]]>>::Output {
        &mut self.0[index]
    }
}