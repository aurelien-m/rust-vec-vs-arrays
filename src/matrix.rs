use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;
use std::fmt;
use std::ops;

pub struct Vec2DMatrix<T: Copy> {
    data: Vec<Vec<T>>,
    shape: (u32, u32),
}

impl<T> Vec2DMatrix<T>
where
    Standard: Distribution<T>,
    T: Copy,
{
    pub fn new_rand(shape: (u32, u32)) -> Vec2DMatrix<T> {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _ in 0..shape.0 {
            data.push(Vec::new());
        }

        for sub_vec in &mut data {
            for _ in 0..shape.1 {
                sub_vec.push(rng.gen::<T>());
            }
        }

        Vec2DMatrix { data, shape }
    }
}

impl<T: std::ops::Add + ops::Add<Output = T> + Copy> ops::Add for Vec2DMatrix<T> {
    type Output = Vec2DMatrix<T>;

    fn add(self, _rhs: Vec2DMatrix<T>) -> Vec2DMatrix<T> {
        if self.shape != _rhs.shape {
            panic!("SHAPES MUST BE THE SAME (bad error handling whatever)");
        }

        let mut data = Vec::new();

        for i in 0..self.data.len() {
            data.push(Vec::new());
            for j in 0..self.data[i].len() {
                data[i].push(self.data[i][j] + _rhs.data[i][j]);
            }
        }

        Vec2DMatrix {
            data,
            shape: self.shape,
        }
    }
}

impl<T: std::fmt::Display + Copy> fmt::Display for Vec2DMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shape=({}, {})\n", self.shape.0, self.shape.1)?;

        let mut i = 0;
        let mut j = self.data[i].len() - 1;

        write!(f, "[[")?;
        for a in 0..j {
            write!(f, "{}, ", self.data[i][a])?;
        }
        write!(f, "{}]\n", self.data[i][j])?;

        i = self.data.len() - 1;
        for a in 1..i {
            write!(f, " [")?;
            j = self.data[a].len() - 1;
            for b in 0..j {
                write!(f, "{}, ", self.data[a][b])?;
            }
            write!(f, "{}]\n", self.data[a][j])?;
        }

        write!(f, " [")?;
        for a in 0..j {
            write!(f, "{}, ", self.data[i][a])?;
        }
        write!(f, "{}]]", self.data[i][j])?;

        Ok(())
    }
}
