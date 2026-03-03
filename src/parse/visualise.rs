use macroquad::prelude::*;

use crate::{mat2::{I, Mat2}, parse::Obj, transform::Transform};

use super::{Ex, MatEx, VecEx, FloatEx, resolve, for_each::{for_each, ExPointer}};

pub fn smooth_step(frac: f32) -> f32 {
    frac * frac * (3.0 - 2.0 * frac)
}

pub fn visualise_obj(obj: Obj, transform: &mut Transform) {
    match obj {
        Obj::Mat(mat) => display_mat_all(mat, transform, "i", "j"),
        Obj::Vec(vec) => display_vec(vec, transform, "", DARKBLUE),
        Obj::Float(float) => display_float(float, transform),
    }
}

pub fn visualise(index: usize, time: f32, ex: &Ex, transform: &mut Transform) -> bool {

    let mut anim_done = false;

    for_each(&mut |comp, ex, _| {
        if index == comp {
            anim_done = visualise_individual(time, ex, transform);
            if !anim_done {
                display_ex_label(ex, transform);
            }
        };
        comp + 1
    }, 0, ex);

    anim_done
}

pub fn display_ex_label(ex: ExPointer, transform: &Transform) {
    let text = ex.to_string();
    let w = measure_text(&text, None, 18, 1.0).width;
    draw_text(&text, transform.screen_dims.x / 2.0 - w / 2.0, 26.0, 18.0, WHITE);
}

pub fn display_background(transform: &Transform) {
    let rect = transform.rect();
    let pos = rect.point().floor().as_i64vec2() - 1;
    let size = rect.size().ceil().as_i64vec2() + 2;

    for x in pos.x..pos.x + size.x {
        let x = transform.world_to_screen(vec2(x as f32, 0.0)).x;
        draw_line(x, 0.0, x, transform.screen_dims[1], 2.0, DARKGRAY);
    }

    for y in pos.y..pos.y + size.y {
        let y = transform.world_to_screen(vec2(0.0, y as f32)).y;
        draw_line(0.0, y, transform.screen_dims[0], y, 2.0, DARKGRAY);
    }
}

pub fn visualise_individual(time: f32, ex: ExPointer, transform: &mut Transform) -> bool {
    match ex {
        ExPointer::Mat(ex) => match ex {
            MatEx::MatMul(ex, ex1) => {
                if time <= 3.0 {
                    let frac = smooth_step(time / 3.0);
                    let mult = resolve(ex) * frac + I * (1.0 - frac);
                    display_mat_all(mult, transform, "i", "j");
                    display_mat_foreground(mult * resolve(ex1), transform, "i", "j");
                } else if time <= 5.0 {
                    let frac = smooth_step((time - 3.0) / 2.0);
                    let mat1 = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat1, transform, "", "");
                    display_mat_foreground(mat1 * resolve(ex1), transform, "i", "j");
                } else {
                    return true
                }
                false
            },
            // MatEx::MatAdd(ex, ex1) => {
            //     if time <= 2.0 {
            //         let frac = smooth_step(time / 2.0);
            //         display_mat_all(resolve(ex1), transform, "i", "j");
            //         // display_vec_offset(resolve(ex1), transform, "i", "j");
            //     }
            // },
            MatEx::MatAdd(_, _) => true,
            MatEx::MatSub(ex, ex1) => true,
            MatEx::Neg(ex) => true,
            MatEx::Mul(ex, ex1) => true,
            MatEx::Div(ex, ex1) => true,
            MatEx::Rot(ex) => true,
            MatEx::New(ex, ex1, ex2, ex3) => true,
            MatEx::Vert(ex, ex1) => true,
            MatEx::Hor(ex, ex1) => true,
            MatEx::Inv(ex) => true,
            MatEx::Literal(_) => true,
        },
        ExPointer::Float(ex) => match ex {
            FloatEx::A(ex) => true,
            FloatEx::B(ex) => true,
            FloatEx::C(ex) => true,
            FloatEx::D(ex) => true,
            FloatEx::X(ex) => true,
            FloatEx::Y(ex) => true,
            FloatEx::Mul(ex, ex1) => true,
            FloatEx::Div(ex, ex1) => true,
            FloatEx::Pow(ex, ex1) => true,
            FloatEx::Add(ex, ex1) => true,
            FloatEx::Sub(ex, ex1) => true,
            FloatEx::Neg(ex) => true,
            FloatEx::Dot(ex, ex1) => true,
            FloatEx::Cross(ex, ex1) => true,
            FloatEx::Det(ex) => true,
            FloatEx::Literal(_) => true,
        },
        ExPointer::Vec(ex) => match ex {
            VecEx::VecMul(ex, ex1) => true,
            VecEx::VecAdd(ex, ex1) => true,
            VecEx::VecSub(ex, ex1) => true,
            VecEx::Neg(ex) => true,
            VecEx::Mul(ex, ex1) => true,
            VecEx::Div(ex, ex1) => true,
            VecEx::Rot(ex) => true,
            VecEx::Left(ex) => true,
            VecEx::Right(ex) => true,
            VecEx::Top(ex) => true,
            VecEx::Bottom(ex) => true,
            VecEx::New(ex, ex1) => true,
            VecEx::Literal(vec2) => true,
        },
    }
}

fn display_mat_background(mat: Mat2, transform: &Transform) {
    if mat.det() == 0.0 {
        let dir = {
            let trial = mat * vec2(1.0, 0.0);
            if trial.x.abs() < f32::EPSILON && trial.y.abs() < f32::EPSILON {
                mat * vec2(0.0, 1.0)
            } else {
                trial
            }
        };
        if dir.x.abs() < f32::EPSILON && dir.y.abs() < f32::EPSILON {
            return
        }
        transform.draw_line(dir, vec2(0.0, 0.0), 2.0, LIGHTGRAY);
    } else {
        let mut neg_x = -1.0;
        while transform.draw_line(mat * vec2(neg_x, -1.0), mat * vec2(neg_x, 1.0), 2.0, GRAY) {
            neg_x -= 1.0;
        }
        let mut pos_x = 1.0;
        while transform.draw_line(mat * vec2(pos_x, -1.0), mat * vec2(pos_x, 1.0), 2.0, GRAY) {
            pos_x += 1.0;
        }
        let mut neg_y = -1.0;
        while transform.draw_line(mat * vec2(-1.0, neg_y), mat * vec2(1.0, neg_y), 2.0, GRAY) {
            neg_y -= 1.0;
        }
        let mut pos_y = 1.0;
        while transform.draw_line(mat * vec2(-1.0, pos_y), mat * vec2(1.0, pos_y), 2.0, GRAY) {
            pos_y += 1.0;
        }

        transform.draw_line(mat * vec2(-1.0, 0.0), mat * vec2(1.0, 0.0), 2.0, LIGHTGRAY);
        transform.draw_line(mat * vec2(0.0, -1.0), mat * vec2(0.0, 1.0), 2.0, LIGHTGRAY);
    }
}

fn display_mat_foreground(mat: Mat2, transform: &mut Transform, labeli: &str, labelj: &str) {
    display_vec(mat * vec2(1.0, 0.0), transform, labeli, GOLD);
    display_vec(mat * vec2(0.0, 1.0), transform, labelj, GOLD);
}

fn display_mat_all(mat: Mat2, transform: &mut Transform, labeli: &str, labelj: &str) {
    display_mat_background(mat, transform);
    display_mat_foreground(mat, transform, labeli, labelj);
}

fn display_vec(vec: Vec2, transform: &mut Transform, label: &str, colour: Color) {
    display_vec_offset(vec, vec2(0.0, 0.0), transform, label, colour);
}

fn display_vec_offset(vec: Vec2, offset: Vec2, transform: &mut Transform, label: &str, colour: Color) {
    let normalized = vec.normalize_or(vec2(1.0, 0.0));
    let normalized = vec2(normalized.x, -normalized.y);
    let pos = transform.world_to_screen(vec) + normalized * 20.0;
    transform.draw_line_segment(vec, offset, 3.0, colour);
    draw_text(label, pos.x, pos.y, 26.0, colour);
    
    let end = transform.world_to_screen(vec + offset);
    draw_triangle(
        end,
        normalized.perp() * 10.0 - normalized * 15.0 + end,
        normalized.perp() * -10.0 - normalized * 15.0 + end,
        colour
    );

    transform.point_of_interest(vec);
    transform.point_of_interest(offset);
}

fn display_point(point: Vec2, transform: &Transform) {

}

fn display_float(float: f32, transform: &Transform) {
    display_point(vec2(float, 0.0), transform)
}

// fn triangles_to_rect(triangles: &[(Vec2, Vec2, Vec2)], rect_pos: Vec2, rect_height: f32, time: f32) -> f32 {
//     // Outputs rect_width
//     // Start by making each triangle into a rectangle of arbitrary dimensions:
//     //  Cut the triangles between the midpoints of two lines
//     //  Cut the triangles perpendicular to the previous cut, intersecting the point where the two lines intersect
//     //  Rearrange to form rectangle of dimensions base * (height / 2)
//     // Make each rectange have the same height:
//     //  https://www.themathdoctors.org/cutting-and-rearranging-a-rectangle/
//     //  https://www.themathdoctors.org/wp-content/uploads/2020/12/ADM55371-solution.png
//     // Arrange these rectangles side by side to form final rectangle
//     todo!()
// }

fn draw_line_if_in_screen(p1: Vec2, p2: Vec2, thickness: f32, colour: Color, window_rect: Rect) -> bool {
    if line_intersects_rect(p1, p2, window_rect) {
        draw_line(p1.x, p1.y, p2.x, p2.y, thickness, colour);
        true
    } else {
        false
    }
}

fn line_intersects_rect(p1: Vec2, p2: Vec2, rect: Rect) -> bool {
    // Check endpoints
    if rect.contains(p1) || rect.contains(p2) {
        return true;
    }
    
    // Check rectangle edges
    let edges = [
        (Vec2::new(rect.x, rect.y), Vec2::new(rect.x + rect.w, rect.y)),
        (Vec2::new(rect.x + rect.w, rect.y), Vec2::new(rect.x + rect.w, rect.y + rect.h)),
        (Vec2::new(rect.x + rect.w, rect.y + rect.h), Vec2::new(rect.x, rect.y + rect.h)),
        (Vec2::new(rect.x, rect.y + rect.h), Vec2::new(rect.x, rect.y)),
    ];
    
    for (e1, e2) in edges.iter() {
        if line_segments_intersect(p1, p2, *e1, *e2) {
            return true;
        }
    }
    
    false
}

fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let denominator = (b2.y - b1.y) * (a2.x - a1.x) - (b2.x - b1.x) * (a2.y - a1.y);
    
    if denominator.abs() < f32::EPSILON {
        return false;
    }

    let ua = ((b2.x - b1.x) * (a1.y - b1.y) - (b2.y - b1.y) * (a1.x - b1.x)) / denominator;
    let ub = ((a2.x - a1.x) * (a1.y - b1.y) - (a2.y - a1.y) * (a1.x - b1.x)) / denominator;
    
    (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub)
}

fn sanitise_time(time: Option<f32>, max: f32) -> f32 {
    match time {
        Some(time) => time / max,
        None => 1.0
    }
}