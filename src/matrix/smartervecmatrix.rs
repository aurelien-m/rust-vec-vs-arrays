use rand::distributions::Standard;
use rand::prelude::*;
use rand::Rng;
use std::fmt;
use std::ops;

pub struct SmartVec2DMatrix<T: Copy> {
    data: Vec<T>,
    shape: (u32, u32),
}

impl<T> SmartVec2DMatrix<T>
where
    Standard: Distribution<T>,
    T: Copy,
{
    pub fn new_rand(shape: (u32, u32)) -> SmartVec2DMatrix<T> {
        let mut rng = rand::thread_rng();
        let mut data = Vec::new();

        for _ in 0..shape.0 * shape.1 {
            data.push(rng.gen::<T>());
        }

        SmartVec2DMatrix { data, shape }
    }
}

impl<T: std::ops::Add + ops::Add<Output = T> + Copy> ops::Add for SmartVec2DMatrix<T> {
    type Output = SmartVec2DMatrix<T>;

    fn add(self, _rhs: SmartVec2DMatrix<T>) -> SmartVec2DMatrix<T> {
        if self.shape != _rhs.shape {
            panic!("SHAPES MUST BE THE SAME (bad error handling whatever)");
        }

        // Faster than Vec::new();
        let mut data = Vec::with_capacity((self.shape.0 * self.shape.1) as usize);

        for i in 0..self.data.len() {
            data.push(self.data[i] + _rhs.data[i]);
        }

        SmartVec2DMatrix {
            data,
            shape: self.shape,
        }
    }
}

// impl<T: std::fmt::Display + Copy> fmt::Display for SmartVec2DMatrix<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "shape=({}, {})\n", self.shape.0, self.shape.1)?;

//         let mut i = 0;
//         let mut j = self.shape.0 - 1;

//         write!(f, "[[")?;
//         for a in 0..j {
//             write!(f, "{}, ", self.data[a + slae* y])?;
//         }
//         write!(f, "{}]\n", self.data[a + width * y])?;

//         i = self.data.len() - 1;
//         for a in 1..i {
//             write!(f, " [")?;
//             j = self.data[a].len() - 1;
//             for b in 0..j {
//                 write!(f, "{}, ", self.data[a + width * y])?;
//             }
//             write!(f, "{}]\n", self.data[a + width * y])?;
//         }

//         write!(f, " [")?;
//         for a in 0..j {
//             write!(f, "{}, ", self.data[a + width * y])?;
//         }
//         write!(f, "{}]]", self.data[a + width * y])?;

//         Ok(())
//     }
// }
