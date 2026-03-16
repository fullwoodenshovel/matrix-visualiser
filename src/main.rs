mod mat2;
use input_handler::InputHandler;
use std::{collections::HashMap, f32};
mod parse;
use parse::{parse_exp, Ex, Line, resolve_ex};
use parse::for_each::{for_each, resolve_indexed};
use parse::visualise::{visualise, display_background, visualise_obj};
mod transform;
use transform::{Transform, get_screen_dims};
use macroquad::prelude::*;

// todo!() Add the rest of the visualisations.
// todo!() Up arrow and down arrow to speed up and slow down.
// todo!() Display the equation used and highlight the specific part of the equation being displayed, and return the result from it.
// todo!() Make tree clickable to decide what is not required to visualise.
// todo!() Add an independent feature that allows for alternative predefined visualisations, accessed using "Show" e.g.:
//  - alternate visualisations of -M having 180 degree rotation and multiplying the vectors by -1
//  - visual proof that A(BC) = (AB)C
//  - visual proof that A + B = B + A


fn conf() -> Conf {
    Conf {
        window_title: "Matrix Visualiser".to_string(),
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
    vars.entry("I".to_string()).insert_entry(parse::Obj::Mat(mat2::I));
    let mut handler = InputHandler::new().expect("Failed to initialise InputHandler");
    display_go_to_term().await;
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
            graphics(ex).await;
            display_go_to_term().await;
        }
    }
}

async fn graphics(ex: Ex) {
    'main: loop {
        next_frame().await;

        let order = 'tree: loop {
            clear_background(BLACK);

            let order = draw_tree(&ex);
            
            next_frame().await;
            if is_key_pressed(KeyCode::Right) {
                break 'tree order
            }
        };

        next_frame().await;

        let mut index = 0;
        let mut time = get_frame_time() * SPEED;
        let mut anim_done = false;
        let mut transform = Transform::new(Vec2::new(0.0, 0.0), 0.01);
        const SPEED: f32 = 1.0;

        'visualise: loop {
            clear_background(BLACK);
            transform.move_camera();
            transform.screen_dims = get_screen_dims();
            display_background(&transform);

            let vec = for_each(&ex, false, true);

            // This algorithm does the following:
            // walk the tree backwards, until reaching index of target. all these values are special cases and do not need to be displayed.
            // set target_depth to depth of this index, and display.
            // when walking backwards, if target_depth >= depth, display and set target_depth to depth.
            if index < order.len() { // This doesnt work if order isnt simply [0,1,2,...]
                let current_ex = for_each(&ex, true, false)[order[index]].0;
                let new_index = vec.iter().position(|d| d.0.pointer_eq(current_ex)).unwrap();
                let mut target_depth = if time == 0.0 {
                    vec[new_index].1 + 1
                } else {
                    vec[new_index].1
                };
                for (ex, depth) in vec.range(new_index + 1..order.len()) {
                    if target_depth >= *depth {
                        visualise_obj(ex.resolve(), &mut transform, true);
                        target_depth = *depth;
                    }
                }
            }

            if time > 0.0 {
                if !anim_done {
                    anim_done = visualise(order[index], time, &ex, &mut transform);
                    time += get_frame_time() * SPEED;
                }
                if anim_done {
                    index += 1;
                    time = 0.0;
                }
            }

            if time == 0.0 {
                visualise_obj(resolve_indexed(order[index - 1], &ex), &mut transform, false);
            }

            if is_key_pressed(KeyCode::Left) && index == 0 {
                break 'visualise
            } else if is_key_pressed(KeyCode::Left) {
                if time == 0.0 {
                    index -= 1;
                    if index == 0 {
                        break 'visualise
                    }
                } else {
                    time = 0.0;
                }
            } else if is_key_pressed(KeyCode::Right) {
                if index == order.len() {
                    loop {
                        draw_text("End of visualisation.", 50.0, 50.0, 30.0, WHITE);
                        next_frame().await;
                        if is_key_pressed(KeyCode::Left) {
                            break
                        } else if is_key_pressed(KeyCode::Right) {
                            break 'main
                        }
                    }
                } else if time > 0.0 {
                    index += 1;
                    time = 0.0;
                } else {
                    time += get_frame_time() * SPEED;
                    anim_done = false;
                }
            }

            next_frame().await;
        }
    }
}

fn get_total(ex: &Ex) -> Vec<usize> {
    let mut depths = Vec::new();

    for (_, depth) in for_each(ex, true, false) {
        while depths.len() <= depth {
            depths.push(0);
        }
        depths[depth] += 1;
    }

    depths
}

fn draw_tree(ex: &Ex) -> Vec<usize> {
    // let mouse = mouse_position();
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

    let mut order = Vec::new();
    
    for (ex, depth) in for_each(ex, true, false) {
        while indicies.len() <= depth {
            indicies.push(0);
        }

        let i = indicies[depth];
        indicies[depth] += 1;
        
        
        let x = x_offset + spacing * (totals[depth] as f32 / -2.0 + 0.5 + i as f32);
        let y = y_offset + spacing * depth as f32;
        
        if depth != 0 {
            let j = indicies[depth - 1];
            let parent_x = x_offset + spacing * (totals[depth - 1] as f32 / -2.0 + 0.5 + j as f32);
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

        match order.last() {
            Some(last) => order.push(last + 1),
            None => order.push(0),
        }
    }

    order
}

async fn display_go_to_term() {
    clear_background(BLACK);
    draw_text("Enter input in terminal", 50.0, 50.0, 30.0, WHITE);
    next_frame().await;
}

// Show Mat(1,2,-3,3) * Mat(0.5,-1,1,0.5)

/*

a = Mat(1,2,-3,3)
b = Mat(0.5,-1,1,0.5)
c = Mat(1.0,0.5,-2,0.5)
Show c*(a-b) + b

Show Mat(1.0,0.5,-2,0.5)*(Mat(1,2,-3,3)-Mat(0.5,-1,1,0.5))

*/

// [
//     (
//         Mat(MatMul(
//             Literal([1.0, 0.5, -2.0, 0.5]),
//             MatAdd(
//                 Literal([1.0, 2.0, -3.0, 3.0]),
//                 Literal([0.5, -1.0, 1.0, 0.5])
//             )
//         )),
//         0
//     ),
//     (
//         Mat(MatAdd(
//             Literal([1.0, 2.0, -3.0, 3.0]),
//             Literal([0.5, -1.0, 1.0, 0.5])
//         )),
//         1
//     ),
//     (
//         Mat(Literal([0.5, -1.0, 1.0, 0.5])),
//         2
//     ),
//     (
//         Mat(Literal([1.0, 2.0, -3.0, 3.0])),
//         2
//     ),
//     (
//         Mat(Literal([1.0, 0.5, -2.0, 0.5])),
//         1
//     )
// ]