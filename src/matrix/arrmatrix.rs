use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;
use std::fmt;
use std::ops;

pub struct Arr2DMatrix<'a, T: 'a> {
    data: &'a mut [&'a mut [T]],
    shape: (u32, u32),
}

impl<T> Arr2DMatrix<'_, T>
where
    Standard: Distribution<T>,
    T: Copy + 'static + std::ops::Add<Output = T>,
{
    pub fn new<'a>(data: &'a mut [&'a [T]]) -> Arr2DMatrix<'a, T> {
        Arr2DMatrix { data, shape: (data.len() as u32, data[0].len() as u32) }
    }

    pub fn add_to(&self, other: Arr2DMatrix<T>, output: &mut Arr2DMatrix<T>) {
        for i in 0..self.shape.0 as usize {
            for j in 0..self.shape.1 as usize {
                output.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
    }
}

impl<T: std::fmt::Display + Copy> fmt::Display for Arr2DMatrix<'_, T> {
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

