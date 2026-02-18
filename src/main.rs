mod mat2;
use input_handler::InputHandler;
use std::{collections::HashMap, f32};
mod parse;
use parse::{parse_exp, get_result};

// struct Transform {
//     scale: f32,
//     offset: Mat2
// }

#[macroquad::main("Matrix Visualiser")]
async fn main() {
    let mut vars = HashMap::new();
    vars.entry("pi".to_string()).insert_entry(parse::Obj::Float(f32::consts::PI));
    vars.entry("tau".to_string()).insert_entry(parse::Obj::Float(f32::consts::TAU));
    vars.entry("e".to_string()).insert_entry(parse::Obj::Float(f32::consts::E));
    let mut handler = InputHandler::new().expect("Failed to initialise InputHandler");
    loop {
        let Some(line) = parse_exp(&vars, &mut handler) else {continue;};
        let Some(result) = get_result(&mut vars, line) else {continue;};
        println!("{result:?}");
    }
}