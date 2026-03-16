use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{mat2::{I, Mat2}, parse::Obj, transform::Transform};

use super::{Ex, MatEx, VecEx, FloatEx, resolve, for_each::{for_each, ExPointer}};

pub fn smooth_step(frac: f32) -> f32 {
    frac * frac * (3.0 - 2.0 * frac)
}

pub fn smoother_step(frac: f32) -> f32 {
    smooth_step(smooth_step(frac))
}

pub fn visualise_obj(obj: Obj, transform: &mut Transform, background: bool) {
    if background {
        match obj {
            Obj::Mat(mat) => {
                display_mat_foreground_with_col(mat, transform, "i", "j", DARKPURPLE);
            },
            Obj::Vec(vec) => display_vec_with_col(vec, transform, "", DARKPURPLE),
            Obj::Float(float) => display_float_with_col(float, transform, DARKPURPLE),
        }
    } else {
        match obj {
            Obj::Mat(mat) => display_mat_all(mat, transform, "i", "j"),
            Obj::Vec(vec) => display_vec(vec, transform, ""),
            Obj::Float(float) => display_float(float, transform),
        }
    }
}

pub fn visualise(index: usize, time: f32, ex: &Ex, transform: &mut Transform) -> bool {

    let indexed = for_each(ex, true, false)[index].0;
    let anim_done = visualise_individual(time, indexed, transform);
    if !anim_done {
        display_ex_label(indexed, transform);
    }

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

    let Vec2 {x, y} = transform.world_to_screen(vec2(0.0, 0.0));
    draw_line(x, 0.0, x, transform.screen_dims[1], 2.0, LIGHTGRAY);
    draw_line(0.0, y, transform.screen_dims[0], y, 2.0, LIGHTGRAY);
}

pub fn visualise_individual(time: f32, ex: ExPointer, transform: &mut Transform) -> bool {
    match ex {
        ExPointer::Mat(ex) => match ex {
            MatEx::MatMul(ex, ex1) => { // Currently flips to a different background then back again. consider changing so there is no background flip.
                if time <= 3.0 {
                    let frac = smooth_step(time / 3.0);
                    let mat1 = resolve(ex);
                    let mult = mat1 * frac + I * (1.0 - frac);
                    display_mat_background(mult, transform);
                    display_mat_foreground_with_col(mat1, transform, "i", "j", RED);
                    display_mat_foreground(mult, transform, "i", "j");
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
            MatEx::MatAdd(ex, ex1) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat1 = resolve(ex);
                    let mat2 = resolve(ex1);
                    display_mat_all(mat2 + mat1 * frac, transform, "i", "j");
                    display_mat_foreground(mat1, transform, "i", "j");
                    display_vec_offset_with_col(mat2.i(), mat1.i() * frac, transform, "",  GOLD);
                    display_vec_offset_with_col(mat2.j(), mat1.j() * frac, transform, "",  GOLD);
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let mat1 = resolve(ex);
                    let mat2 = resolve(ex1);
                    display_mat_all(mat2 + mat1, transform, "i", "j");
                    display_vec_offset_with_col(mat2.i() * (1.0 - frac), mat1.i() * (1.0 - frac), transform, "",  GOLD);
                    display_vec_offset_with_col(mat2.j() * (1.0 - frac), mat1.j() * (1.0 - frac), transform, "",  GOLD);
                    display_vec_with_col(mat1.i() * (1.0 - frac), transform, "i",  GOLD);
                    display_vec_with_col(mat1.j() * (1.0 - frac), transform, "j",  GOLD);
                } else {
                    return true;
                }
                false
            },
            MatEx::MatSub(ex, ex1) => {
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    let mat1 = resolve(ex);
                    let mat2 = resolve(ex1);

                    display_vec_offset_with_col(frac * (mat1.i() - mat2.i()), mat2.i(), transform, "", GOLD);
                    display_vec_offset_with_col(frac * (mat1.j() - mat2.j()), mat2.j(), transform, "", GOLD);
                    display_vec_with_col(mat1.i(), transform, "", GOLD);
                    display_vec_with_col(mat1.j(), transform, "", GOLD);
                    display_vec_with_col(mat2.i(), transform, "", GOLD);
                    display_vec_with_col(mat2.j(), transform, "", GOLD);
                } else if time <= 3.0 {
                    let frac = smoother_step((time - 1.0) / 2.0);
                    let mat1 = resolve(ex);
                    let mat2 = resolve(ex1);
                    let x = (1.0 - frac) * mat2;
                    display_vec_with_col((1.0 - frac) * mat1.i(), transform, "", GOLD);
                    display_vec_with_col((1.0 - frac) * mat1.j(), transform, "", GOLD);
                    display_vec_with_col((1.0 - frac) * mat2.i(), transform, "", GOLD);
                    display_vec_with_col((1.0 - frac) * mat2.j(), transform, "", GOLD); 
                    
                    display_vec_offset_with_col(mat1.i() - mat2.i(), x.i(), transform, "", GOLD);
                    display_vec_offset_with_col(mat1.j() - mat2.j(), x.j(), transform, "", GOLD);
                } else {
                    return true;
                }
                false
            },
            MatEx::Neg(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat = resolve(ex);
                    display_mat_background(mat, transform);
                    display_mat_foreground(Mat2::rotation(frac * PI) * mat, transform, "i", "j");
                } else {
                    return true;
                }
                false
            },
            MatEx::Mul(ex, ex1) => {
                let float = resolve(ex);
                if time <= 2.5 {
                    let frac = smooth_step(time / 2.5);
                    let mat = resolve(ex1);
                    display_mat_background(frac * float * I + (1.0 - frac) * I, transform);
                    display_mat_foreground(frac * float * mat + (1.0 - frac) * mat, transform, "i", "j");
                    display_float(float, transform);
                } else if time <= 3.5 {
                    let frac = smooth_step(time - 2.5);
                    let mat = resolve(ex1);
                    display_point(vec2(float, 0.0), transform, "", RED, (1.0 - frac) * 5.0);
                    display_mat_foreground(float * mat, transform, "i", "j");
                } else {
                    return true;
                }
                false
            },
            MatEx::Div(ex, ex1) => {
                let float = resolve(ex1);
                if time <= 2.5 {
                    let frac = smoother_step(time / 2.5);
                    let mat = resolve(ex);
                    display_mat_background((1.0 - frac) * float * I + frac * I, transform);
                    display_mat_foreground(frac / float * mat + (1.0 - frac) * mat, transform, "i", "j");
                    display_float(float, transform);
                } else if time <= 3.5 {
                    let frac = smooth_step(time - 2.5);
                    let mat = resolve(ex);
                    display_point(vec2(float, 0.0), transform, "", RED, (1.0 - frac) * 5.0);
                    display_mat_foreground(mat / float, transform, "i", "j");
                } else {
                    return true;
                }
                false
            },
            MatEx::Rot(ex) => {
                let angle = resolve(ex);
                let t3 = (angle.ln() * 3.0).max(1.0);
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    display_vec_offset(vec2(angle, 0.0), vec2(frac, 0.0), transform, "");
                } else if time <= 3.0 {
                    let frac = smooth_step((time - 1.0) / 2.0);
                    display_vec_offset(Vec2::from_angle(frac * PI / 2.0) * angle, vec2(1.0, 0.0), transform, "");
                } else if time <= t3 + 3.0 {
                    let frac = smoother_step((time - 3.0) / t3);
                    display_mat_all(Mat2::rotation(frac * angle), transform, "i", "j");
                    display_vec_offset(Vec2::from_angle(frac * angle + PI / 2.0) * (1.0 - frac) * angle, Vec2::from_angle(frac * angle), transform, "");
                    display_arc(vec2(0.0, 0.0), 1.0, 0.0, frac * angle, DARKBLUE, false, transform);
                } else {
                    return true;
                }
                false
            },
            MatEx::New(ex, ex1, ex2, ex3) => {
                let ix = resolve(ex);
                let jx = resolve(ex1);
                let iy = resolve(ex2);
                let jy = resolve(ex3);

                if time <= 1.5 {
                    let frac = smooth_step(time / 1.5);
                    display_arc(vec2(0.0, 0.0), iy, 0.0, frac * PI / 2.0, RED, true, transform);
                    display_arc(vec2(0.0, 0.0), jy, 0.0, frac * PI / 2.0, RED, true, transform);
                    display_point(Vec2::from_angle(frac * PI / 2.0) * iy, transform, &iy.to_string(), RED, 5.0);
                    display_point(Vec2::from_angle(frac * PI / 2.0) * jy, transform, &jy.to_string(), RED, 5.0);
                    display_float(ix, transform);
                    display_float(jx, transform);
                } else if time <= 3.5 {
                    let frac = smooth_step((time - 1.5) / 2.0);
                    display_float(ix, transform);
                    display_float(jx, transform);
                    display_point(vec2(0.0, iy), transform, &iy.to_string(), RED, 5.0);
                    display_point(vec2(0.0, jy), transform, &jy.to_string(), RED, 5.0);
                    display_arc(vec2(0.0, 0.0), iy, frac * PI / 2.0, (1.0 - frac) * PI / 2.0, RED, true, transform);
                    display_arc(vec2(0.0, 0.0), jy, frac * PI / 2.0, (1.0 - frac) * PI / 2.0, RED, true, transform);
                    display_vec_offset(vec2(frac * ix, 0.0), vec2(0.0, iy), transform, "i");
                    display_vec_offset(vec2(0.0, frac * iy), vec2(ix, 0.0), transform, "i");
                    display_vec_offset(vec2(frac * jx, 0.0), vec2(0.0, jy), transform, "j");
                    display_vec_offset(vec2(0.0, frac * jy), vec2(jx, 0.0), transform, "j");
                } else if time <= 5.5 {
                    let frac = smooth_step((time - 3.5) / 2.0);
                    display_point(vec2(ix, 0.0), transform, "", RED, 5.0 * (1.0 - frac));
                    display_point(vec2(jx, 0.0), transform, "", RED, 5.0 * (1.0 - frac));
                    display_point(vec2(0.0, iy), transform, "", RED, 5.0 * (1.0 - frac));
                    display_point(vec2(0.0, jy), transform, "", RED, 5.0 * (1.0 - frac));
                    display_vec_offset(vec2(ix, 0.0), vec2(0.0, iy), transform, "i");
                    display_vec_offset(vec2(0.0, iy), vec2(ix, 0.0), transform, "i");
                    display_vec_offset(vec2(jx, 0.0), vec2(0.0, jy), transform, "j");
                    display_vec_offset(vec2(0.0, jy), vec2(jx, 0.0), transform, "j");
                    display_vec_with_col(vec2(ix, iy) * frac, transform, "i", GOLD);
                    display_vec_with_col(vec2(jx, jy) * frac, transform, "j", GOLD);
                } else if time <= 7.0 {
                    let frac = smooth_step((time - 5.5) / 1.5);
                    display_vec_offset(vec2(ix, 0.0) * (1.0 - frac), vec2(0.0, iy), transform, "i");
                    display_vec_offset(vec2(0.0, iy) * (1.0 - frac), vec2(ix, 0.0), transform, "i");
                    display_vec_offset(vec2(jx, 0.0) * (1.0 - frac), vec2(0.0, jy), transform, "j");
                    display_vec_offset(vec2(0.0, jy) * (1.0 - frac), vec2(jx, 0.0), transform, "j");
                    display_mat_all(Mat2::new(ix, jx, iy, jy), transform, "i", "j");
                } else {
                    return true;
                }
                false
            },
            MatEx::Vert(ex, ex1) => {
                fn lerp_colours(start: Color, end: Color, frac: f32) -> Color {
                    Color::from_vec(end.to_vec() * frac + (1.0 - frac) * start.to_vec())
                }

                if time <= 1.5 {
                    let frac = smooth_step(time / 1.5);
                    let v1 = resolve(ex);
                    let v2 = resolve(ex1);

                    display_mat_background_with_col(
                        Mat2::new(v1.x, v2.x, v1.y, v2.y),
                        transform,
                        lerp_colours(BLANK, LIGHTGRAY, frac),
                        lerp_colours(BLANK, GRAY, frac)
                    );

                    display_vec_with_col(v1, transform, "i", lerp_colours(DARKBLUE, GOLD, frac));
                    display_vec_with_col(v2, transform, "j", lerp_colours(DARKBLUE, GOLD, frac));
                } else {
                    return true;
                }
                false
            },
            MatEx::Hor(ex, ex1) => true,
            MatEx::Inv(ex) => {
                if time <= 3.0 {
                    let frac = smooth_step(time / 3.0);
                    let mat = resolve(ex);
                    display_mat_all(frac * I + (1.0 - frac) * mat, transform, "i", "j");
                    display_mat_foreground((1.0 - frac) * I + frac * mat.inv(), transform, "i", "j");
                } else if time <= 5.0 {
                    let frac = smooth_step((time - 3.0) / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground((1.0 - frac) * I, transform, "i", "j");
                    display_mat_foreground(mat.inv(), transform, "i", "j");
                } else {
                    return true;
                }
                false
            },
            MatEx::Literal(_) => true,
        },
        ExPointer::Float(ex) => match ex {
            FloatEx::A(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground(mat, transform, "i", "j");
                    display_vec_offset(vec2(0.0, frac * -mat.i().y), mat.i(), transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat, transform, "i", "j");
                    display_float(mat.i().x, transform);
                    display_vec_offset(vec2(0.0, (1.0 - frac) * -mat.i().y), vec2(mat.i().x, (1.0 - frac) * mat.i().y), transform, "");
                } else {
                    return true;
                }
                false
            },
            FloatEx::B(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground(mat, transform, "i", "j");
                    display_vec_offset(vec2(0.0, frac * -mat.j().y), mat.j(), transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat, transform, "i", "j");
                    display_float(mat.j().x, transform);
                    display_vec_offset(vec2(0.0, (1.0 - frac) * -mat.j().y), vec2(mat.j().x, (1.0 - frac) * mat.j().y), transform, "");
                } else {
                    return true;
                }
                false
            },
            FloatEx::C(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground(mat, transform, "i", "j");
                    display_vec_offset(vec2(frac * -mat.i().x, 0.0), mat.i(), transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat, transform, "i", "j");
                    display_point(vec2(0.0, mat.i().y), transform, &mat.i().y.to_string(), RED, 5.0);
                    display_vec_offset(vec2((1.0 - frac) * -mat.i().x, 0.0), vec2((1.0 - frac) * mat.i().x, mat.i().y), transform, "");
                } else if time <= 5.5 {
                    let frac = smooth_step((time - 4.0) / 1.5);
                    let mat = resolve(ex);
                    display_point(Vec2::from_angle((1.0 - frac) * PI / 2.0) * mat.i().y, transform, &mat.i().y.to_string(), RED, 5.0);
                    display_arc(vec2(0.0, 0.0), mat.i().y, PI / 2.0, -frac * PI / 2.0, RED, true, transform);
                } else if time <= 7.0 {
                    let frac = smooth_step((time - 5.5) / 1.5);
                    let mat = resolve(ex);
                    display_float(mat.i().y, transform);
                    display_arc(vec2(0.0, 0.0), mat.i().y, (1.0 - frac) * PI / 2.0, -(1.0 - frac) * PI / 2.0, RED, true, transform);
                } else {
                    return true;
                }
                false
            },
            FloatEx::D(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground(mat, transform, "i", "j");
                    display_vec_offset(vec2(frac * -mat.j().x, 0.0), mat.j(), transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let mat = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat, transform, "i", "j");
                    display_point(vec2(0.0, mat.j().y), transform, &mat.j().y.to_string(), RED, 5.0);
                    display_vec_offset(vec2((1.0 - frac) * -mat.j().x, 0.0), vec2((1.0 - frac) * mat.j().x, mat.j().y), transform, "");
                } else if time <= 5.5 {
                    let frac = smooth_step((time - 4.0) / 1.5);
                    let mat = resolve(ex);
                    display_point(Vec2::from_angle((1.0 - frac) * PI / 2.0) * mat.j().y, transform, &mat.j().y.to_string(), RED, 5.0);
                    display_arc(vec2(0.0, 0.0), mat.j().y, PI / 2.0, -frac * PI / 2.0, RED, true, transform);
                } else if time <= 7.0 {
                    let frac = smooth_step((time - 5.5) / 1.5);
                    let mat = resolve(ex);
                    display_float(mat.j().y, transform);
                    display_arc(vec2(0.0, 0.0), mat.j().y, (1.0 - frac) * PI / 2.0, -(1.0 - frac) * PI / 2.0, RED, true, transform);
                } else {
                    return true;
                }
                false
            },
            FloatEx::X(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let vec = resolve(ex);
                    display_vec(vec, transform, "");
                    display_vec_offset(vec2(0.0, frac * -vec.y), vec, transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let vec = resolve(ex);
                    display_vec((1.0 - frac) * vec, transform, "");
                    display_float(vec.x, transform);
                    display_vec_offset(vec2(0.0, (1.0 - frac) * -vec.y), vec2(vec.x, (1.0 - frac) * vec.y), transform, "");
                } else {
                    return true;
                }
                false
            },
            FloatEx::Y(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let vec = resolve(ex);
                    display_vec(vec, transform, "");
                    display_vec_offset(vec2(frac * -vec.x, 0.0), vec, transform, "");
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let vec = resolve(ex);
                    display_vec((1.0 - frac) * vec, transform, "");
                    display_point(vec2(0.0, vec.y), transform, &vec.y.to_string(), RED, 5.0);
                    display_vec_offset(vec2((1.0 - frac) * -vec.x, 0.0), vec2((1.0 - frac) * vec.x, vec.y), transform, "");
                } else if time <= 5.5 {
                    let frac = smooth_step((time - 4.0) / 1.5);
                    let vec = resolve(ex);
                    display_point(Vec2::from_angle((1.0 - frac) * PI / 2.0) * vec.y, transform, &vec.y.to_string(), RED, 5.0);
                    display_arc(vec2(0.0, 0.0), vec.y, PI / 2.0, -frac * PI / 2.0, RED, true, transform);
                } else if time <= 7.0 {
                    let frac = smooth_step((time - 5.5) / 1.5);
                    let vec = resolve(ex);
                    display_float(vec.y, transform);
                    display_arc(vec2(0.0, 0.0), vec.y, (1.0 - frac) * PI / 2.0, -(1.0 - frac) * PI / 2.0, RED, true, transform);
                } else {
                    return true;
                }
                false
            },
            FloatEx::Mul(ex, ex1) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    display_mat_background((frac * f1 + 1.0 - frac) * I, transform);
                    display_float(f2 * (frac * f1 + 1.0 - frac), transform);
                    display_float(frac * f1 + 1.0 - frac, transform);
                    display_point(vec2(f1, 0.0), transform, &f1.to_string(), GOLD, 5.0);
                } else if time <= 3.0 {
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    let frac = smooth_step(time - 2.0);
                    display_float(f1 * f2, transform);
                    display_point(vec2(f1, 0.0), transform, &f1.to_string(), RED, (1.0 - frac) * 5.0);
                } else {
                    return true;
                }
                false
            },
            FloatEx::Div(ex, ex1) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    display_mat_background(((1.0 - frac) * f2 + frac) * I, transform);
                    display_float(f1 / f2 * ((1.0 - frac) * f2 + frac), transform);
                    display_float((1.0 - frac) * f2 + frac, transform);
                } else if time <= 3.0 {
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    let frac = smooth_step(time - 2.0);
                    display_float(f1 / f2, transform);
                    display_point(vec2(1.0, 0.0), transform, &f1.to_string(), RED, (1.0 - frac) * 5.0);
                } else {
                    return true;
                }
                false
            },
            FloatEx::Pow(ex, ex1) => true,
            FloatEx::Add(ex, ex1) => {
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);

                    display_vec(vec2(frac * f2, 0.0), transform, "");
                    display_float(f1, transform);
                    display_float(f2, transform);
                } else if time <= 3.0 {
                    let frac = smoother_step((time - 1.0) / 2.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    let x = frac * f1;
                    let y = -x * (x - f1) / f1;
                    
                    display_vec_offset(vec2(f2, 0.0), vec2(x, y), transform, "");
                    display_float(f1, transform);
                    display_point(vec2(f2 + x, y), transform, &(f2 + x).to_string(), RED, 5.0);
                } else if time <= 4.0 {
                    let frac = smooth_step(time - 3.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    display_vec_offset(vec2(f2 * (1.0 - frac), 0.0), vec2(f1 + f2 * frac, 0.0), transform, "");
                    display_float(f1 + f2, transform);
                    display_point(vec2(f1, 0.0), transform, &f1.to_string(), RED, 5.0 * (1.0 - frac));
                } else {
                    return true;
                }
                false
            },
            FloatEx::Sub(ex, ex1) => {
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);

                    display_vec_offset(vec2(frac * (f1 - f2), 0.0), vec2(f2, 0.0), transform, "");
                    display_float(f1, transform);
                    display_float(f2, transform);
                } else if time <= 3.0 {
                    let frac = smoother_step((time - 1.0) / 2.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    let x = (1.0 - frac) * f2;
                    let y = -x * (x - f2) / f2;
                    
                    display_vec_offset(vec2(f1 - f2, 0.0), vec2(x, y), transform, "");
                    display_point(vec2(x, y), transform, &x.to_string(), RED, 5.0);
                    display_point(vec2(x + f1 - f2, y), transform, &(x + f1 - f2).to_string(), RED, 5.0);
                } else if time <= 4.0 {
                    let frac = smooth_step(time - 3.0);
                    let f1 = resolve(ex);
                    let f2 = resolve(ex1);
                    display_vec_offset(vec2((f1 - f2) * (1.0 - frac), 0.0), vec2((f1 - f2) * frac, 0.0), transform, "");
                    display_float(f1 - f2, transform);
                    display_point(vec2(0.0, 0.0), transform, &0.0.to_string(), RED, 5.0 * (1.0 - frac));
                } else {
                    return true;
                }
                false
            },
            FloatEx::Neg(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let f = resolve(ex);
                    display_point(Vec2::from_angle(frac * PI) * f, transform, &f.to_string(), RED, 5.0);
                    display_arc(vec2(0.0, 0.0), f, 0.0, frac * PI, RED, true, transform);
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let f = resolve(ex);
                    display_float(-f, transform);
                    display_arc(vec2(0.0, 0.0), f, frac * PI, (1.0 - frac) * PI, RED, true, transform);
                } else {
                    return true;
                }
                false
            },
            FloatEx::Dot(ex, ex1) => true,
            FloatEx::Cross(ex, ex1) => true,
            FloatEx::Det(ex) => true,
            FloatEx::Literal(_) => true,
        },
        ExPointer::Vec(ex) => match ex {
            VecEx::VecMul(ex, ex1) => {
                if time <= 3.0 {
                    let frac = smooth_step(time / 3.0);
                    let mat1 = resolve(ex);
                    let mult = mat1 * frac + I * (1.0 - frac);
                    display_mat_background(mult, transform);
                    display_mat_foreground_with_col(mat1, transform, "i", "j", RED);
                    display_mat_foreground(mult, transform, "i", "j");
                    display_vec(mult * resolve(ex1), transform, "");
                } else if time <= 5.0 {
                    let frac = smooth_step((time - 3.0) / 2.0);
                    let mat1 = resolve(ex);
                    display_mat_foreground((1.0 - frac) * mat1, transform, "", "");
                    display_vec(mat1 * resolve(ex1), transform, "");
                } else {
                    return true
                }
                false
            },
            VecEx::VecAdd(ex, ex1) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let vec1 = resolve(ex);
                    let vec2 = resolve(ex1);
                    display_vec(vec2 + vec1 * frac, transform, "");
                    display_vec(vec1, transform, "");
                    display_vec_offset_with_col(vec2, vec1 * frac, transform, "",  DARKBLUE);
                } else if time <= 4.0 {
                    let frac = smooth_step((time - 2.0) / 2.0);
                    let vec1 = resolve(ex);
                    let vec2 = resolve(ex1);
                    display_vec(vec2 + vec1, transform, "");
                    display_vec_offset_with_col(vec2 * (1.0 - frac), vec1 * (1.0 - frac), transform, "",  DARKBLUE);
                    display_vec_with_col(vec1 * (1.0 - frac), transform, "i",  DARKBLUE);
                } else {
                    return true;
                }
                false
            },
            VecEx::VecSub(ex, ex1) => {
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    let v1 = resolve(ex);
                    let v2 = resolve(ex1);

                    display_vec_offset(frac * (v1 - v2), v2, transform, "");
                    display_vec(v1, transform, "");
                    display_vec(v2, transform, "");
                } else if time <= 3.0 {
                    let frac = smoother_step((time - 1.0) / 2.0);
                    let v1 = resolve(ex);
                    let v2 = resolve(ex1);
                    let x = (1.0 - frac) * v2;
                    display_vec((1.0 - frac) * v1, transform, "");
                    display_vec((1.0 - frac) * v2, transform, "");
                    
                    display_vec_offset(v1 - v2, x, transform, "");
                } else {
                    return true;
                }
                false
            },
            VecEx::Neg(ex) => {
                if time <= 2.0 {
                    let frac = smooth_step(time / 2.0);
                    let vec = resolve(ex);
                    display_vec(Mat2::rotation(frac * PI) * vec, transform, "");
                } else {
                    return true;
                }
                false
            },
            VecEx::Mul(ex, ex1) => {
                let float = resolve(ex);
                if time <= 2.5 {
                    let frac = smooth_step(time / 2.5);
                    let vec = resolve(ex1);
                    display_mat_background(frac * float * I + (1.0 - frac) * I, transform);
                    display_vec(frac * float * vec + (1.0 - frac) * vec, transform, "");
                    display_float(float, transform);
                } else if time <= 3.5 {
                    let frac = smooth_step(time - 2.5);
                    let vec = resolve(ex1);
                    display_point(vec2(float, 0.0), transform, "", RED, (1.0 - frac) * 5.0);
                    display_vec(float * vec, transform, "");
                } else {
                    return true;
                }
                false
            },
            VecEx::Div(ex, ex1) => {
                let float = resolve(ex1);
                if time <= 2.5 {
                    let frac = smoother_step(time / 2.5);
                    let vec = resolve(ex);
                    display_mat_background((1.0 - frac) * float * I + frac * I, transform);
                    display_vec(frac / float * vec + (1.0 - frac) * vec, transform, "");
                    display_float(float, transform);
                } else if time <= 3.5 {
                    let frac = smooth_step(time - 2.5);
                    let vec = resolve(ex);
                    display_point(vec2(float, 0.0), transform, "", RED, (1.0 - frac) * 5.0);
                    display_vec(vec / float, transform, "");
                } else {
                    return true;
                }
                false
            },
            VecEx::Rot(ex) => {
                let angle = resolve(ex);
                let t3 = (angle.ln() * 3.0).max(1.0);
                if time <= 1.0 {
                    let frac = smooth_step(time);
                    display_vec_offset(vec2(angle, 0.0), vec2(frac, 0.0), transform, "");
                } else if time <= 3.0 {
                    let frac = smooth_step((time - 1.0) / 2.0);
                    display_vec_offset(Vec2::from_angle(frac * PI / 2.0) * angle, vec2(1.0, 0.0), transform, "");
                } else if time <= t3 + 3.0 {
                    let frac = smoother_step((time - 3.0) / t3);
                    display_vec(Vec2::from_angle(frac * angle), transform, "");
                    display_vec_offset(Vec2::from_angle(frac * angle + PI / 2.0) * (1.0 - frac) * angle, Vec2::from_angle(frac * angle), transform, "");
                    display_arc(vec2(0.0, 0.0), 1.0, 0.0, frac * angle, DARKBLUE, false, transform);
                } else {
                    return true;
                }
                false
            },
            VecEx::Left(ex) => {
                fn lerp_colours(start: Color, end: Color, frac: f32) -> Color {
                    Color::from_vec(end.to_vec() * frac + (1.0 - frac) * start.to_vec())
                }

                if time <= 1.5 {
                    let frac = smooth_step(time / 1.5);
                    let mat = resolve(ex);

                    display_mat_background_with_col(mat, transform, lerp_colours(BLANK, LIGHTGRAY, frac), lerp_colours(GRAY, BLANK, frac));

                    display_vec_with_col(mat.i(), transform, "i", lerp_colours(GOLD, DARKBLUE, frac));
                    display_vec_with_col(mat.j(), transform, "j", lerp_colours(GOLD, BLANK, frac));
                } else {
                    return true;
                }
                false
            },
            VecEx::Right(ex) => {
                fn lerp_colours(start: Color, end: Color, frac: f32) -> Color {
                    Color::from_vec(end.to_vec() * frac + (1.0 - frac) * start.to_vec())
                }

                if time <= 1.5 {
                    let frac = smooth_step(time / 1.5);
                    let mat = resolve(ex);

                    display_mat_background_with_col(mat, transform, lerp_colours(BLANK, LIGHTGRAY, frac), lerp_colours(GRAY, BLANK, frac));

                    display_vec_with_col(mat.i(), transform, "j", lerp_colours(GOLD, BLANK, frac));
                    display_vec_with_col(mat.j(), transform, "i", lerp_colours(GOLD, DARKBLUE, frac));
                } else {
                    return true;
                }
                false
            },
            VecEx::Top(ex) => true,
            VecEx::Bottom(ex) => true,
            VecEx::New(ex, ex1) => true,
            VecEx::Literal(vec2) => true,
        },
    }
}

fn display_mat_background(mat: Mat2, transform: &Transform) {
    display_mat_background_with_col(mat, transform, LIGHTGRAY, GRAY);
}

fn display_mat_foreground(mat: Mat2, transform: &mut Transform, labeli: &str, labelj: &str) {
    display_mat_foreground_with_col(mat, transform, labeli, labelj, GOLD);
}

fn display_mat_foreground_with_col(mat: Mat2, transform: &mut Transform, labeli: &str, labelj: &str, colour: Color) {
    display_vec_with_col(mat * vec2(1.0, 0.0), transform, labeli, colour);
    display_vec_with_col(mat * vec2(0.0, 1.0), transform, labelj, colour);
}

fn display_mat_all(mat: Mat2, transform: &mut Transform, labeli: &str, labelj: &str) {
    display_mat_background(mat, transform);
    display_mat_foreground(mat, transform, labeli, labelj);
}

fn display_mat_background_with_col(mat: Mat2, transform: &Transform, axis: Color, others: Color) {
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
        transform.draw_line(dir, vec2(0.0, 0.0), 2.0, axis);
    } else {
        if mat.i().length() * transform.scale < 2.0 || mat.j().length() * transform.scale < 2.0 {
            clear_background(others);
        } else {
            let mut neg_x = -1.0;
            while transform.draw_line(mat * vec2(neg_x, -1.0), mat * vec2(neg_x, 1.0), 2.0, others) {
                neg_x -= 1.0;
            }
            let mut pos_x = 1.0;
            while transform.draw_line(mat * vec2(pos_x, -1.0), mat * vec2(pos_x, 1.0), 2.0, others) {
                pos_x += 1.0;
            }
            let mut neg_y = -1.0;
            while transform.draw_line(mat * vec2(-1.0, neg_y), mat * vec2(1.0, neg_y), 2.0, others) {
                neg_y -= 1.0;
            }
            let mut pos_y = 1.0;
            while transform.draw_line(mat * vec2(-1.0, pos_y), mat * vec2(1.0, pos_y), 2.0, others) {
                pos_y += 1.0;
            }
        }
    
        transform.draw_line(mat * vec2(-1.0, 0.0), mat * vec2(1.0, 0.0), 2.0, axis);
        transform.draw_line(mat * vec2(0.0, -1.0), mat * vec2(0.0, 1.0), 2.0, axis);
    }
}

fn display_vec(vec: Vec2, transform: &mut Transform, label: &str) {
    display_vec_with_col(vec, transform, label, DARKBLUE);
}

fn display_vec_with_col(vec: Vec2, transform: &mut Transform, label: &str, colour: Color) {
    display_vec_offset_with_col(vec, vec2(0.0, 0.0), transform, label, colour);
}

fn display_vec_offset(vec: Vec2, offset: Vec2, transform: &mut Transform, label: &str) {
    display_vec_offset_with_col(vec, offset, transform, label, DARKBLUE);
}

fn display_vec_offset_with_col(vec: Vec2, offset: Vec2, transform: &mut Transform, label: &str, colour: Color) {
    let normalized = vec.normalize_or(vec2(1.0, 0.0));
    let normalized = vec2(normalized.x, -normalized.y);
    let arrow_multiplier = (transform.scale * vec.length()).min(20.0) / 20.0;
    let p1 = transform.world_to_screen(vec + offset) - normalized * 5.0 * arrow_multiplier;
    let p2 = transform.world_to_screen(offset);
    draw_line(p1.x, p1.y, p2.x, p2.y, 3.0, colour);
    let pos = transform.world_to_screen(vec + offset) + normalized * 20.0;
    draw_text(label, pos.x, pos.y, arrow_multiplier * 26.0, colour);
    
    let end = transform.world_to_screen(vec + offset);
    draw_triangle(
        end,
        normalized.perp() * 10.0 * arrow_multiplier - normalized * 15.0 * arrow_multiplier + end,
        normalized.perp() * -10.0 * arrow_multiplier - normalized * 15.0 * arrow_multiplier + end,
        colour
    );

    transform.point_of_interest(vec);
    transform.point_of_interest(offset);
}

fn display_point(point: Vec2, transform: &mut Transform, label: &str, colour: Color, size: f32) {
    let pos = transform.world_to_screen(point);
    draw_circle(pos.x, pos.y, size, colour);
    draw_text(label, pos.x, pos.y + 15.0 + size, 26.0 * size / 5.0, colour);
    transform.point_of_interest(point);
}

fn display_float(float: f32, transform: &mut Transform) {
    display_float_with_col(float, transform, RED);
}

fn display_float_with_col(float: f32, transform: &mut Transform, colour: Color) {
    display_point(vec2(float, 0.0), transform, &float.to_string(), colour, 5.0)
}

fn display_arc(center: Vec2, radius: f32, start_angle: f32, angle: f32, colour: Color, head: bool, transform: &mut Transform) {
    transform.point_of_interest(Vec2::from_angle(start_angle) * radius);
    transform.point_of_interest(Vec2::from_angle(start_angle + angle) * radius);
    
    let center = transform.world_to_screen(center);
    let screen_radius = transform.scale * radius;
    let screen_angle;
    let screen_start_angle;
    if angle > 0.0 {
        screen_angle = -angle;
        screen_start_angle = angle + start_angle;
    } else {
        screen_angle = angle;
        screen_start_angle = start_angle;
    }
    draw_arc(center.x, center.y, 128, screen_radius, -screen_start_angle / PI * 180.0, 2.0, -screen_angle / PI * 180.0, colour);

    if head {
        let end = transform.world_to_screen(Vec2::from_angle(start_angle + angle) * radius);
        let normalized = Vec2::from_angle(start_angle + angle).perp();
        let normalized = vec2(normalized.x, -normalized.y);
        let arrow_multiplier = (screen_radius * angle).clamp(-20.0, 20.0) / 20.0;
    
        draw_triangle(
            end,
            normalized.perp() * 10.0 * arrow_multiplier - normalized * 15.0 * arrow_multiplier + end,
            normalized.perp() * -10.0 * arrow_multiplier - normalized * 15.0 * arrow_multiplier + end,
            colour
        );
    }
}