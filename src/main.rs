mod matrix;
use matrix::Vec2DMatrix;

fn main() {
    let a: Vec2DMatrix<f32> = Vec2DMatrix::new_rand((4, 5));
    println!("{}", a);
}
