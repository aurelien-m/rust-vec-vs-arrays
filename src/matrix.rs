use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;
use std::fmt;

pub struct Vec2DMatrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Vec2DMatrix<T>
where
    Standard: Distribution<T>,
{
    pub fn new_rand(size: (u8, u8)) -> Vec2DMatrix<T> {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _ in 0..size.0 {
            data.push(Vec::new());
        }

        for sub_vec in &mut data {
            for _ in 0..size.1 {
                sub_vec.push(rng.gen::<T>());
            }
        }

        Vec2DMatrix { data }
    }
}

impl<T: std::fmt::Display> fmt::Display for Vec2DMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
