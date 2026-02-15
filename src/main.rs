mod mat2;
use mat2::Mat2;
use std::io;
mod parse;
use parse::{Ex, MatEx, VecEx, FloatEx, parse_exp, input, tokenise};

struct Transform {
    scale: f32,
    offset: Mat2
}

#[macroquad::main("Matrix Visualiser")]
async fn main() {
    let result = tokenise(&input("> "));
    println!("{result:?}");
}