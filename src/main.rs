mod matrix;
use matrix::smartervecmatrix::SmartVec2DMatrix;
use matrix::vecmatrix::Vec2DMatrix;
use std::time::Instant;
use ndarray::prelude::*;

fn main() {
    let a: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((10_000, 10_000));
    let b: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((10_000, 10_000));

    let now = Instant::now();

    {
        let _ = a + b;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let a: SmartVec2DMatrix<f32> = SmartVec2DMatrix::new_rand((10_000, 10_000));
    let b: SmartVec2DMatrix<f32> = SmartVec2DMatrix::new_rand((10_000, 10_000));

    let now = Instant::now();

    {
        let _ = a + b;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let a = [[1 as f32; 10_000]; 10_000];
    let b = [[2 as f32; 10_000]; 10_000];
    let mut c = [[0 as f32; 10_000]; 10_000];

    let now = Instant::now();

    {
        for i in 0..10_000 {
            for j in 0..10_000 {
                c[i][j] = a[i][j] + b[i][j];
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let a = Array::<f32, _>::zeros((10_000, 10_000).f());
    let b = Array::<f32, _>::ones((10_000, 10_000).f());

    let now = Instant::now();

    {
        let _ = a + b;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
