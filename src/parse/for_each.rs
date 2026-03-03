use std::fmt::Display;

use super::{FloatEx, VecEx, MatEx, Ex, Obj, ExTrait};


#[derive(Copy, Clone)]
pub enum ExPointer<'a> {
    Mat(&'a MatEx),
    Float(&'a FloatEx),
    Vec(&'a VecEx),
}

impl<'a> Display for ExPointer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            ExPointer::Mat(ex) => match ex {
                MatEx::MatMul(_, _) => "M * M",
                MatEx::MatAdd(_, _) => "M + M",
                MatEx::MatSub(_, _) => "M - M",
                MatEx::Neg(_) => "- M",
                MatEx::Mul(_, _) => "F * M",
                MatEx::Div(_, _) => "M / F",
                MatEx::Rot(_) => "Rot-M",
                MatEx::New(_, _, _, _) => "New-M",
                MatEx::Vert(_, _) => "Vert-M",
                MatEx::Hor(_, _) => "Hor-M",
                MatEx::Inv(_) => "Inv-M",
                MatEx::Literal(mat) => &format!(
                    "[{} {}; {} {}]",
                    format_min_chars(mat.a()),
                    format_min_chars(mat.b()),
                    format_min_chars(mat.c()),
                    format_min_chars(mat.d())
                ),
            },
            ExPointer::Float(ex) => match ex {
                FloatEx::A(_) => "M.a",
                FloatEx::B(_) => "M.b",
                FloatEx::C(_) => "M.c",
                FloatEx::D(_) => "M.d",
                FloatEx::X(_) => "V.x",
                FloatEx::Y(_) => "V.y",
                FloatEx::Mul(_, _) => "F * F",
                FloatEx::Div(_, _) => "F / F",
                FloatEx::Pow(_, _) => "F ^ F",
                FloatEx::Add(_, _) => "F + F",
                FloatEx::Sub(_, _) => "F - F",
                FloatEx::Neg(_) => "- F",
                FloatEx::Dot(_, _) => "V * V",
                FloatEx::Cross(_, _) => "V x V",
                FloatEx::Det(_) => "Det",
                FloatEx::Literal(float) => &format_min_chars(*float),
            },
            ExPointer::Vec(ex) => match ex {
                VecEx::VecMul(_, _) => "M * V",
                VecEx::VecAdd(_, _) => "V + V",
                VecEx::VecSub(_, _) => "V - V",
                VecEx::Neg(_) => "- V",
                VecEx::Mul(_, _) => "F * V",
                VecEx::Div(_, _) => "V / F",
                VecEx::Rot(_) => "Rot-V",
                VecEx::Left(_) => "Left",
                VecEx::Right(_) => "Right",
                VecEx::Top(_) => "Top",
                VecEx::Bottom(_) => "Bottom",
                VecEx::New(_, _) => "New-V",
                VecEx::Literal(vec) => &format!("({} {})",
                    format_min_chars(vec.x),
                    format_min_chars(vec.y)
                ),
            },
        };
        write!(f, "{}", result)
    }
}

pub fn for_each<F: FnMut(T, ExPointer, usize) -> T, T>(f: &mut F, t: T, ex: &Ex) -> T {
    match ex {
        Ex::Mat(ex) => for_each_mat(f, t, ex, 0) ,
        Ex::Vec(ex) => for_each_vec(f, t, ex, 0) ,
        Ex::Float(ex) => for_each_float(f, t, ex, 0) ,
    }
}

pub fn resolve_indexed(index: usize, ex: &Ex) -> Obj {
    for_each(&mut |(mut curr, mut result), pointer, _| {
        if curr == index {
            result = Some(
                match pointer {
                    ExPointer::Mat(ex) => Obj::Mat(ExTrait::resolve(ex)),
                    ExPointer::Float(ex) => Obj::Float(ExTrait::resolve(ex)),
                    ExPointer::Vec(ex) => Obj::Vec(ExTrait::resolve(ex)),
                }
            )
        }
        curr += 1;
        (curr, result)
    }, (0, None), ex).1.unwrap()
}

pub fn for_each_mat<'a, F: FnMut(T, ExPointer, usize) -> T, T>(f: &mut F, mut t: T, ex: &'a MatEx, depth: usize) -> T {
    let next_depth = depth + 1;
    match ex {
        MatEx::MatMul(ex, ex1) |
        MatEx::MatAdd(ex, ex1) |
        MatEx::MatSub(ex, ex1) => {
            t = for_each_mat(f, t, ex, next_depth);
            t = for_each_mat(f, t, ex1, next_depth);
        },
        MatEx::Mul(ex, ex1) => {
            t = for_each_float(f, t, ex, next_depth);
            t = for_each_mat(f, t, ex1, next_depth);
        }
        MatEx::Neg(ex) |
        MatEx::Inv(ex) => { t = for_each_mat(f, t, ex, next_depth) },
        MatEx::Div(ex, ex1) => {
            t = for_each_mat(f, t, ex, next_depth);
            t = for_each_float(f, t, ex1, next_depth);
        },
        MatEx::Rot(ex) => { t = for_each_float(f, t, ex, next_depth) },
        MatEx::New(ex, ex1, ex2, ex3) => {
            t = for_each_float(f, t, ex, next_depth);
            t = for_each_float(f, t, ex1, next_depth);
            t = for_each_float(f, t, ex2, next_depth);
            t = for_each_float(f, t, ex3, next_depth);
        },
        MatEx::Vert(ex, ex1) |
        MatEx::Hor(ex, ex1) => {
            t = for_each_vec(f, t, ex, next_depth);
            t = for_each_vec(f, t, ex1, next_depth);
        },
        MatEx::Literal(_) => (),
    };
    f(t, ExPointer::<'a>::Mat(ex), depth)
}

pub fn for_each_float<'a, F: FnMut(T, ExPointer, usize) -> T, T>(f: &mut F, mut t: T, ex: &'a FloatEx, depth: usize) -> T {
    let next_depth = depth + 1;
    match ex {
        FloatEx::A(ex) |
        FloatEx::B(ex) |
        FloatEx::C(ex) |
        FloatEx::D(ex) => { t = for_each_mat(f, t, ex, next_depth) },
        FloatEx::X(ex) |
        FloatEx::Y(ex) => { t = for_each_vec(f, t, ex, next_depth) },
        FloatEx::Mul(ex, ex1) |
        FloatEx::Div(ex, ex1) |
        FloatEx::Pow(ex, ex1) |
        FloatEx::Add(ex, ex1) |
        FloatEx::Sub(ex, ex1) => {
            t = for_each_float(f, t, ex, next_depth);
            t = for_each_float(f, t, ex1, next_depth);
        },
        FloatEx::Neg(ex) => { t = for_each_float(f, t, ex, next_depth) },
        FloatEx::Dot(ex, ex1) |
        FloatEx::Cross(ex, ex1) => {
            t = for_each_vec(f, t, ex, next_depth);
            t = for_each_vec(f, t, ex1, next_depth);
        },
        FloatEx::Det(ex) => { t = for_each_mat(f, t, ex, next_depth) }
        FloatEx::Literal(_) => (),
    };
    t = f(t, ExPointer::<'a>::Float(ex), depth);
    t
}

pub fn for_each_vec<'a, F: FnMut(T, ExPointer, usize) -> T, T>(f: &mut F, mut t: T, ex: &'a VecEx, depth: usize) -> T {
    let next_depth = depth + 1;
    match ex {
        VecEx::VecMul(ex, ex1) => {
            t = for_each_mat(f, t, ex, next_depth);
            t = for_each_vec(f, t, ex1, next_depth);
        },
        VecEx::VecAdd(ex, ex1) |
        VecEx::VecSub(ex, ex1) => {
            t = for_each_vec(f, t, ex, next_depth);
            t = for_each_vec(f, t, ex1, next_depth);
        },
        VecEx::Neg(ex) => { t = for_each_vec(f, t, ex, next_depth) },
        VecEx::Mul(ex, ex1) |
        VecEx::Div(ex1, ex) => {
            t = for_each_float(f, t, ex, next_depth);
            t = for_each_vec(f, t, ex1, next_depth);
        },
        VecEx::Rot(ex) => { t = for_each_float(f, t, ex, next_depth) },
        VecEx::Left(ex) |
        VecEx::Right(ex) |
        VecEx::Top(ex) |
        VecEx::Bottom(ex) => { t = for_each_mat(f, t, ex, next_depth) },
        VecEx::New(ex, ex1) => {
            t = for_each_float(f, t, ex, next_depth);
            t = for_each_float(f, t, ex1, next_depth);
        },
        VecEx::Literal(_) => (),
    };
    t = f(t, ExPointer::<'a>::Vec(ex), depth);
    t
}

fn format_min_chars(x: f32) -> String {
    if x == 0.0 {
        return "0".to_string();
    }

    let abs = x.abs();
    let exp = abs.log10().floor();
    let scale = 10f32.powf(1.0 - exp); // for 2 significant figures
    let rounded = (x * scale).round() / scale;
    let mut fixed = if x < 10.0 {
        format!("{:.7}", rounded)
    } else {
        format!("{:.0}", x)
    };

    // Fixed notation (trim trailing zeros and dot)
    while fixed.contains('.') && fixed.ends_with('0') {
        fixed.pop();
    }
    if fixed.ends_with('.') {
        fixed.pop();
    }

    // Scientific notation with 1 digit after decimal (2 s.f.)
    let sci = format!("{:.1e}", rounded);

    if sci.len() < fixed.len() {
        sci
    } else {
        fixed
    }
}