mod matrix;
use matrix::Vec2DMatrix;

fn main() {
    let a: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((4, 5));
    println!("{}", a);

    let a: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((10_000, 10_000));
    let b: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((10_000, 10_000));

    use std::time::Instant;
    let now = Instant::now();

    {
        let _ = a + b;
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
