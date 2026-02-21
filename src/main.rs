mod mat2;
use input_handler::InputHandler;
use std::{collections::HashMap, f32};
mod parse;
use parse::{parse_exp, Ex, Line, resolve_ex, for_each::{ExPointer, for_each}};
use macroquad::prelude::*;

// struct Transform {
//     scale: f32,
//     offset: Mat2
// }


fn conf() -> Conf {
    Conf {
        window_title: "Matrix Visualiser".to_string(),
        // Request 4x Multisampling
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut vars = HashMap::new();
    vars.entry("pi".to_string()).insert_entry(parse::Obj::Float(f32::consts::PI));
    vars.entry("tau".to_string()).insert_entry(parse::Obj::Float(f32::consts::TAU));
    vars.entry("e".to_string()).insert_entry(parse::Obj::Float(f32::consts::E));
    let mut handler = InputHandler::new().expect("Failed to initialise InputHandler");
    loop {
        let Some((line, show)) = parse_exp(&vars, &mut handler) else {continue;};

        let mut set_var = None;
        let ex = match line {
            Line::Eval(ex) => ex,
            Line::SetVar(var, ex) => {set_var = Some(var); ex},
            Line::None => continue,
        };

        let result = resolve_ex(&ex);
        
        if let Some(var) = set_var {
            vars.entry(var).insert_entry(result);
        } else {
            println!("{result:?}");
        }

        if show {
            println!("Go to window for visualisation.");
            visualise(ex).await;
        }
    }
}

async fn visualise(ex: Ex) {
    // let mut show_order = Vec::new();
    loop {
        clear_background(BLACK);
        draw_tree(&ex);
        next_frame().await;
    }
}

fn get_total(ex: &Ex) -> Vec<usize> {
    fn closure(mut depths: Vec<usize>, _: ExPointer, depth: usize) -> Vec<usize> {
        if depths.len() <= depth {
            depths.push(1);
        } else {
            depths[depth] += 1;
        }
        depths
    }
    for_each(&mut closure, Vec::new(), ex)
}

fn draw_tree(ex: &Ex) {
    let mouse = mouse_position();
    let (width, height) = (screen_width(), screen_height());
    let totals = get_total(ex);
    let spacing = *[
        (width as usize - 20) / *totals.iter().max().expect("ex shouldn't be empty"),
        (height as usize - 20) / totals.len(),
        300,
        300
    ].iter().min().unwrap() as f32;

    let polygon_lines = (spacing * 0.5).round().clamp(8.0, 128.0) as u8;

    let mut indicies = Vec::new();

    let x_offset = width / 2.0;
    let y_offset = spacing / 2.0;

    for_each(&mut |_, ex, depth| {
        let i;

        if indicies.len() <= depth {
            i = 0;
            indicies.push(1);
        } else {
            i = indicies[depth];
            indicies[depth] += 1;
        }
        
        let x = x_offset + spacing * (totals[depth] as f32 / -2.0 + 0.5 + i as f32);
        let y = y_offset + spacing * depth as f32;
        
        if depth != 0 {
            let j = indicies[depth - 1];
            let parent_x = x_offset + spacing * (totals[depth - 1] as f32 / -2.0 - 0.5 + j as f32);
            let parent_y = y_offset + spacing * (depth - 1) as f32;

            draw_line(x, y, parent_x, parent_y, (spacing / 100.0).clamp(2.0, f32::INFINITY), LIGHTGRAY);
        };

        draw_poly(x, y, polygon_lines, spacing / 3.0, 0.0, YELLOW);

        let text = &ex.to_string();
        let mut scale = (spacing / 3.0) as u16;
        let TextDimensions { width: mut text_width, mut offset_y, .. } = measure_text(text, None, scale, 1.0);

        while
            let TextDimensions { width: test_width, offset_y: test_y, .. } = measure_text(text, None, scale, 1.0) &&
            test_width > spacing / 1.7
        {
            scale -= 1; // could be optimised with binary search instead
            text_width = test_width;
            offset_y = test_y;
        };

        draw_text(text, x - text_width / 2.0, y + offset_y / 3.0, scale as f32, BLUE);

    }, (), ex)
}