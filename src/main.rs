mod mat2;
use mat2::Mat2;
use std::{collections::HashMap, f32, io};
mod parse;
use parse::{Ex, MatEx, VecEx, FloatEx, parse_exp, input, tokenise, get_result};

struct Transform {
    scale: f32,
    offset: Mat2
}

#[macroquad::main("Matrix Visualiser")]
async fn main() {
    loop {
        let mut vars = HashMap::new();
        vars.entry("pi".to_string()).insert_entry(parse::Obj::Float(f32::consts::PI));
        vars.entry("tau".to_string()).insert_entry(parse::Obj::Float(f32::consts::TAU));
        vars.entry("e".to_string()).insert_entry(parse::Obj::Float(f32::consts::E));
        let Some(line) = parse_exp(&mut vars) else {continue;};
        let Some(result) = get_result(&mut vars, line) else {continue;};
        println!("{result:?}");
    }
}