use std::{collections::{HashMap, VecDeque}, fmt::{Debug, Pointer}, io::{self, Write}};

use macroquad::math::Vec2;

use crate::mat2::Mat2;


pub fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

#[derive(Clone, Debug)]
pub enum MatEx {
    MatMul(Box<MatEx>, Box<MatEx>),
    MatAdd(Box<MatEx>, Box<MatEx>),
    MatSub(Box<MatEx>, Box<MatEx>),
    Neg(Box<MatEx>),
    Mul(Box<FloatEx>, Box<MatEx>),
    Div(Box<MatEx>, Box<FloatEx>),
    Rot(Box<FloatEx>),
    New(Box<FloatEx>, Box<FloatEx>, Box<FloatEx>, Box<FloatEx>),
    Vert(Box<VecEx>, Box<VecEx>),
    Hor(Box<VecEx>, Box<VecEx>),
    Inv(Box<MatEx>),
    Literal(Mat2)
}

#[derive(Clone, Debug)]
pub enum VecEx {
    VecMul(Box<MatEx>, Box<VecEx>),
    VecAdd(Box<VecEx>, Box<VecEx>),
    VecSub(Box<VecEx>, Box<VecEx>),
    Neg(Box<VecEx>),
    Mul(Box<FloatEx>, Box<VecEx>),
    Div(Box<VecEx>, Box<FloatEx>),
    Rot(Box<FloatEx>),
    Left(Box<MatEx>),
    Right(Box<MatEx>),
    Top(Box<MatEx>),
    Bottom(Box<MatEx>),
    New(Box<FloatEx>, Box<FloatEx>),
    Literal(Vec2)
}

#[derive(Clone, Debug)]
pub enum FloatEx {
    TopLeft(Box<MatEx>),
    TopRight(Box<MatEx>),
    BottomLeft(Box<MatEx>),
    BottomRight(Box<MatEx>),
    X(Box<VecEx>),
    Y(Box<VecEx>),
    Mul(Box<FloatEx>, Box<FloatEx>),
    Div(Box<FloatEx>, Box<FloatEx>),
    Pow(Box<FloatEx>, Box<FloatEx>),
    Add(Box<FloatEx>, Box<FloatEx>),
    Sub(Box<FloatEx>, Box<FloatEx>),
    Neg(Box<FloatEx>),
    Dot(Box<VecEx>, Box<VecEx>),
    Cross(Box<VecEx>, Box<VecEx>),
    Literal(f32)
}

pub enum Obj {
    Mat(Mat2),
    Vec(Vec2),
    Float(f32)
}

impl Debug for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obj::Mat(mat2) => mat2.fmt(f),
            Obj::Vec(vec2) => vec2.fmt(f),
            Obj::Float(float) => float.fmt(f),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Ex {
    Mat(MatEx),
    Vec(VecEx),
    Float(FloatEx)
}

trait ExTrait: Clone {
    type Into;
    fn concrete(ex: Ex) -> Option<Self> where Self: Sized;
    fn concrete_err(ex: Ex) -> Result<Self, String> where Self: Sized {
        match Self::concrete(ex) {
            Some(value) => Ok(value),
            None => Err("Received wrong type in function use.".to_string())
        }
    }
    fn resolve(ex: Self) -> Self::Into;
}

impl ExTrait for MatEx {
    type Into = Mat2;
    fn concrete(ex: Ex) -> Option<Self> {
        match ex {
            Ex::Mat(value) => Some(value),
            _ => None
        }
    }
    fn resolve(ex: Self) -> Self::Into {
        resolve_mat(ex)
    }
}
impl ExTrait for VecEx {
    type Into = Vec2;
    fn concrete(ex: Ex) -> Option<Self> {
        match ex {
            Ex::Vec(value) => Some(value),
            _ => None
        }
    }
    fn resolve(ex: Self) -> Self::Into {
        resolve_vec(ex)
    }
}
impl ExTrait for FloatEx {
    type Into = f32;
    fn concrete(ex: Ex) -> Option<Self> {
        match ex {
            Ex::Float(value) => Some(value),
            _ => None
        }
    }
    fn resolve(ex: Self) -> Self::Into {
        resolve_float(ex)
    }
}

impl Ex {
    fn get_type(&self) -> &'static str {
        match self {
            Ex::Mat(_) => "Matrix",
            Ex::Vec(_) => "Vector",
            Ex::Float(_) => "Real",
        }
    }
}

#[derive(Debug)]
pub enum Line {
    Eval(Ex),
    SetVar(String, Ex)
}


#[derive(PartialEq, Debug, Clone, Default)]
pub enum Token {
    LBrace,
    RBrace,
    Float(f32),
    Mat,
    Vec,
    Comma,
    Add,
    Neg,
    Mul,
    Div,
    Pow,
    DotX,
    DotY,
    DotA,
    DotB,
    DotC,
    DotD,
    Hor,
    Vert,
    RotMat,
    RotVec,
    Cross,
    Eq,
    VarName(String),
    #[default]
    Eof
}

fn make_exp(lhs: Ex, rhs: Ex, op: Token) -> Option<Ex> {
    let result = match lhs {
        Ex::Mat(lhs) => match rhs {
            Ex::Mat(rhs) => match op {
                Token::Add => Ex::Mat(MatEx::MatAdd(Box::new(lhs), Box::new(rhs))),
                Token::Neg => Ex::Mat(MatEx::MatSub(Box::new(lhs), Box::new(rhs))),
                Token::Mul => Ex::Mat(MatEx::MatMul(Box::new(lhs), Box::new(rhs))),
                Token::Div => Ex::Mat(MatEx::MatMul(Box::new(lhs), Box::new(MatEx::Inv(Box::new(rhs))))),
                _ => return None
            },
            Ex::Vec(rhs) => match op {
                Token::Mul => Ex::Vec(VecEx::VecMul(Box::new(lhs), Box::new(rhs))),
                _ => return None
            },
            Ex::Float(rhs) => match op {
                Token::Mul => Ex::Mat(MatEx::Mul(Box::new(rhs), Box::new(lhs))),
                Token::Div => Ex::Mat(MatEx::Div(Box::new(lhs), Box::new(rhs))),
                // Token::Pow => todo!(), // Raising matricies to float powers
                _ => return None
            },
        },
        Ex::Vec(lhs) => match rhs {
            Ex::Mat(_rhs) => return None,
            Ex::Vec(rhs) => match op {
                Token::Add => Ex::Vec(VecEx::VecAdd(Box::new(lhs), Box::new(rhs))),
                Token::Neg => Ex::Vec(VecEx::VecSub(Box::new(lhs), Box::new(rhs))),
                Token::Mul => Ex::Float(FloatEx::Dot(Box::new(lhs), Box::new(rhs))),
                Token::Cross => Ex::Float(FloatEx::Cross(Box::new(lhs), Box::new(rhs))),
                _ => return None
            },
            Ex::Float(rhs) => match op {
                Token::Mul => Ex::Vec(VecEx::Mul(Box::new(rhs), Box::new(lhs))),
                Token::Div => Ex::Vec(VecEx::Div(Box::new(lhs), Box::new(rhs))),
                _ => return None
            },
        },
        Ex::Float(lhs) => match rhs {
            Ex::Mat(rhs) => match op {
                Token::Mul => Ex::Mat(MatEx::Mul(Box::new(lhs), Box::new(rhs))),
                Token::Div => Ex::Mat(MatEx::Mul(Box::new(lhs), Box::new(MatEx::Inv(Box::new(rhs))))),
                _ => return None
            },
            Ex::Vec(rhs) => match op {
                Token::Mul => Ex::Vec(VecEx::Mul(Box::new(lhs), Box::new(rhs))),
                _ => return None
            },
            Ex::Float(rhs) => match op {
                Token::Add => Ex::Float(FloatEx::Add(Box::new(lhs), Box::new(rhs))),
                Token::Neg => Ex::Float(FloatEx::Sub(Box::new(lhs), Box::new(rhs))),
                Token::Mul => Ex::Float(FloatEx::Mul(Box::new(lhs), Box::new(rhs))),
                Token::Div => Ex::Float(FloatEx::Div(Box::new(lhs), Box::new(rhs))),
                Token::Pow => Ex::Float(FloatEx::Pow(Box::new(lhs), Box::new(rhs))),
                _ => return None
            },
        },
    };
    Some(result)
}

pub fn tokenise(inp: &str) -> Result<Vec<Token>, String> {
    if !inp.is_ascii() {
        return Err("Expression consists of non-ascii characters.".to_string())
    }
    
    let mut result = Vec::new();
    let inp = inp.trim().split(" ");

    for inp in inp {
        let mut residual = String::new();
        
        fn append_residual(result: &mut Vec<Token>, residual: &str) {
            match residual {
                ".x" => result.push(Token::DotX),
                ".y" => result.push(Token::DotY),
                ".a" => result.push(Token::DotA),
                ".b" => result.push(Token::DotB),
                ".c" => result.push(Token::DotC),
                ".d" => result.push(Token::DotD),
                "Hor" => result.push(Token::Hor),
                "Vert" => result.push(Token::Vert),
                "Mat" => result.push(Token::Mat),
                "Vec" => result.push(Token::Vec),
                "RotMat" => result.push(Token::RotMat),
                "RotVec" => result.push(Token::RotVec),
                value => {
                    match value.parse() {
                        Ok(float) => result.push(Token::Float(float)),
                        Err(_) => result.push(Token::VarName(value.to_string())),
                    }
                }
            }
        }

        fn append_residual_first(result: &mut Vec<Token>, residual: &str) -> bool {
            false
            // match residual {
            //     "**" => result.push(Token::Pow),
            //     _value => {
            //         return false
            //     }
            // }
            // true
        }
        
        for char in inp.chars() {
            let mut added_to_residual = false;
            let mut push = None;
            match char {
                '(' => push = Some(Token::LBrace),
                ')' => push = Some(Token::RBrace),
                ',' => push = Some(Token::Comma),
                '+' => push = Some(Token::Add),
                '-' => push = Some(Token::Neg),
                '*' => push = Some(Token::Mul),
                '/' => push = Some(Token::Div),
                'X' => push = Some(Token::Cross),
                '=' => push = Some(Token::Eq),
                '^' => push = Some(Token::Pow),
                r => {
                    residual.push(r);
                    added_to_residual = true;
                }
            }

            if append_residual_first(&mut result, &residual) {
                residual = String::new();
            } else if !added_to_residual && !residual.is_empty() {
                append_residual(&mut result, &residual);
                residual = String::new();
            }
            if let Some(value) = push {
                result.push(value);
            }
        }
        if !residual.is_empty() {
            append_residual(&mut result, &residual);
            residual = String::new();
        }
    }
    Ok(result)
}


struct Lexer<T: Default> {
    data: VecDeque<T>,
    default: T
}

impl<T: Default + Debug> Lexer<T> {
    fn new(data: VecDeque<T>) -> Self {
        Self {
            data,
            default: T::default()
        }
    }

    fn peek(&self) -> &T {
        if self.data.is_empty() {
            return &self.default
        }
        &self.data[0]
    }

    fn next(&mut self) -> T {
        // self.data.pop_front().expect("Called next on empty lexer. If this is intentional, uncomment following code:")
        self.data.pop_front().unwrap_or_default()
    }
}

pub fn make_tree(vars: &HashMap<String, Obj>, tokens: Vec<Token>) -> Result<Line, String> {
    if tokens.is_empty() {
        Err("Did not enter anything".to_string())
    } else if tokens.len() >= 2 && let Token::VarName(name) = tokens[0].clone() && let Token::Eq = tokens[1] {
        let mut tokens: VecDeque<Token> = tokens.into();
        tokens.pop_front();
        tokens.pop_front();
        Ok(Line::SetVar(name, pratt_parse(vars, &mut Lexer::new(tokens), 0)?))
    } else {
        let mut tokens = Lexer::new(tokens.into());
        Ok(Line::Eval(pratt_parse(vars, &mut tokens, 0)?))
    }
}

fn end_of_ex(token: &Token) -> bool {
    matches!(token, Token::Comma | Token::RBrace | Token::Eof)
}

fn binding_power(token: &Token) -> Option<(u8, u8)> {
    let result = match token {
        Token::Add => (1, 2),
        Token::Neg => (3, 4),
        Token::Mul => (5, 6),
        Token::Div => (5, 6), // todo!() make sure this is the correct way round. this means "1 / 2 / 3" should work
        Token::Cross => (7, 8),
        Token::Pow => (9, 10),
        _ => return None
    };
    Some(result)
}

/// This supports functions with at least one argument
fn parse_func<const N: usize, T: ExTrait>(vars: &HashMap<String, Obj>, lexer: &mut Lexer<Token>) -> Result<[T; N], String> {
    let token = lexer.next();

    if Token::LBrace != token {
        return Err("You must have an open bracket before function use.".to_string())
    }

    let mut result = std::array::repeat(None);

    for n in result[..N-1].iter_mut() {
        *n = Some(T::concrete_err(pratt_parse(vars, lexer, 0)?)?);

        let token = lexer.next();

        if Token::Comma != token {
            return Err("You must have a comma after each argument in a function.".to_string())
        }
    }
    
    result[N - 1] = Some(T::concrete_err(pratt_parse(vars, lexer, 0)?)?);

    let token = lexer.next();

    if Token::RBrace != token {
        return Err("You must have a comma after each argument in a function.".to_string())
    }

    Ok(result.map(|d| d.unwrap()))
}

fn parse_func_boxed<const N: usize, T: ExTrait>(vars: &HashMap<String, Obj>, lexer: &mut Lexer<Token>) -> Result<[Box<T>; N], String> {
    Ok(parse_func(vars, lexer)?.map(|d| Box::new(d)))
}

fn pratt_parse(vars: &HashMap<String, Obj>, lexer: &mut Lexer<Token>, min_bp: u8) -> Result<Ex, String> {
    let mut lhs = match lexer.next() { // todo!() Test "3 +"
        Token::Float(float) => Ex::Float(FloatEx::Literal(float)),
        Token::VarName(name) => match vars.get(&name) {
            None => return Err(format!("Variable `{name}` does not exist.")),
            Some(Obj::Float(float)) => Ex::Float(FloatEx::Literal(*float)),
            Some(Obj::Mat(mat)) => Ex::Mat(MatEx::Literal(*mat)),
            Some(Obj::Vec(vec)) => Ex::Vec(VecEx::Literal(*vec)),
        },
        Token::Hor => {
            let [a, b] = parse_func_boxed(vars, lexer)?;
            Ex::Mat(MatEx::Hor(a, b))
        },
        Token::Vert => {
            let [a, b] = parse_func_boxed(vars, lexer)?;
            Ex::Mat(MatEx::Vert(a, b))
        },
        Token::LBrace => {
            let result = pratt_parse(vars, lexer, 0)?;

            let token = lexer.next();

            if Token::RBrace != token {
                return Err("You are missing a close bracket".to_string())
            }

            result
        },
        Token::Mat => {
            let [a, b, c, d] = parse_func_boxed(vars, lexer)?;
            Ex::Mat(MatEx::New(a, b, c, d))
        },
        Token::Neg => {
            match pratt_parse(vars, lexer, 4)? {
                Ex::Mat(value) => Ex::Mat(MatEx::Neg(Box::new(value))),
                Ex::Vec(value) => Ex::Vec(VecEx::Neg(Box::new(value))),
                Ex::Float(value) => Ex::Float(FloatEx::Neg(Box::new(value))),
            }
        },
        Token::RotMat => {
            let [a] = parse_func_boxed(vars, lexer)?;
            Ex::Mat(MatEx::Rot(a))
        },
        Token::RotVec => {
            let [a] = parse_func_boxed(vars, lexer)?;
            Ex::Vec(VecEx::Rot(a))
        },
        Token::Vec => {
            let [a, b] = parse_func_boxed(vars, lexer)?;
            Ex::Vec(VecEx::New(a, b))
        },
        other => return Err(format!("Unexpected token `{other:?}`."))
    };

    loop {
        let op = lexer.peek();

        if end_of_ex(op) {
            break;
        }
        let Some((l_bp, r_bp)) = binding_power(op) else {
            return Err(format!("Expected operation, got token `{op:?}`."))
        };
        
        if l_bp < min_bp {
            break;
        }

        let op = lexer.next();

        let rhs = pratt_parse(vars, lexer, r_bp)?;
        let lhs_type = lhs.get_type();
        let rhs_type = rhs.get_type();
        let str_op = format!("{op:?}");
        lhs = match make_exp(lhs, rhs, op) {
            Some(lhs) => lhs,
            None => return Err(format!("You cannot perform the operation {} on a {} and a {}", str_op, lhs_type, rhs_type))
        }
    }
    Ok(lhs)

}

pub fn parse_exp(vars: &mut HashMap<String, Obj>) -> Option<Line> {
    let tokenised = tokenise(&input("> "));
    match tokenised {
        Err(err) => eprintln!("{err}"),
        Ok(tokens) => match make_tree(vars, tokens) {
            Ok(tree) => return Some(tree),
            Err(err) => eprintln!("{err}")
        }
    };
    
    None
}

pub fn get_result(vars: &mut HashMap<String, Obj>, line: Line) -> Option<Obj> {
    let mut set_var = None;
    let ex = match line {
        Line::Eval(ex) => ex,
        Line::SetVar(var, ex) => {set_var = Some(var); ex},
    };

    let result = resolve_ex(ex);

    if let Some(var) = set_var {
        vars.entry(var).insert_entry(result);
        return None
    };

    Some(result)
}

pub fn resolve_ex(ex: Ex) -> Obj {
    match ex {
        Ex::Float(ex) => Obj::Float(resolve_float(ex)),
        Ex::Mat(ex) => Obj::Mat(resolve_mat(ex)),
        Ex::Vec(ex) => Obj::Vec(resolve_vec(ex)),
    }
}

fn resolve<T: ExTrait>(ex: Box<T>) -> T::Into {
    T::resolve(*ex)
}

fn resolve_float(ex: FloatEx) -> f32 {
    match ex {
        FloatEx::TopLeft(ex) => resolve(ex).a(),
        FloatEx::TopRight(ex) => resolve(ex).b(),
        FloatEx::BottomLeft(ex) => resolve(ex).c(),
        FloatEx::BottomRight(ex) => resolve(ex).c(),
        FloatEx::X(ex) => resolve(ex).x,
        FloatEx::Y(ex) => resolve(ex).y,
        FloatEx::Mul(ex, ex1) => resolve(ex) * resolve(ex1),
        FloatEx::Div(ex, ex1) => resolve(ex) / resolve(ex1),
        FloatEx::Pow(ex, ex1) => resolve(ex).powf(resolve(ex1)),
        FloatEx::Add(ex, ex1) => resolve(ex) + resolve(ex1),
        FloatEx::Sub(ex, ex1) => resolve(ex) - resolve(ex1),
        FloatEx::Neg(ex) => -resolve(ex),
        FloatEx::Dot(ex, ex1) => resolve(ex).dot(resolve(ex1)),
        FloatEx::Cross(ex, ex1) => {let a = resolve(ex); let b = resolve(ex1); a.x * b.y - a.y * b.x},
        FloatEx::Literal(float) => float,
    }
}

fn resolve_mat(ex: MatEx) -> Mat2 {
    match ex {
        MatEx::MatMul(ex, ex1) => resolve(ex) * resolve(ex1),
        MatEx::MatAdd(ex, ex1) => resolve(ex) + resolve(ex1),
        MatEx::MatSub(ex, ex1) => resolve(ex) - resolve(ex1),
        MatEx::Neg(ex) => - resolve(ex),
        MatEx::Mul(ex, ex1) => resolve(ex) * resolve(ex1),
        MatEx::Div(ex, ex1) => resolve(ex) / resolve(ex1),
        MatEx::Rot(ex) => Mat2::rotation(resolve(ex)),
        MatEx::New(ex, ex1, ex2, ex3) => Mat2::new(resolve(ex), resolve(ex1), resolve(ex2), resolve(ex3)),
        MatEx::Vert(ex, ex1) => {let a = resolve(ex); let b = resolve(ex1); Mat2::new(a.x, b.x, a.y, b.y)},
        MatEx::Hor(ex, ex1) => {let a = resolve(ex); let b = resolve(ex1); Mat2::new(a.x, a.y, b.x, b.y)},
        MatEx::Inv(ex) => resolve(ex).inv(),
        MatEx::Literal(mat) => mat,
    }
}

fn resolve_vec(ex: VecEx) -> Vec2 {
    match ex {
        VecEx::VecMul(ex, ex1) => resolve(ex) * resolve(ex1),
        VecEx::VecAdd(ex, ex1) => resolve(ex) + resolve(ex1),
        VecEx::VecSub(ex, ex1) => resolve(ex) - resolve(ex1),
        VecEx::Neg(ex) => -resolve(ex),
        VecEx::Mul(ex, ex1) => resolve(ex) * resolve(ex1),
        VecEx::Div(ex, ex1) => resolve(ex) / resolve(ex1),
        VecEx::Rot(ex) => Vec2::from_angle(resolve(ex)),
        VecEx::Left(ex) => {let mat = resolve(ex); Vec2::new(mat.a(), mat.c())},
        VecEx::Right(ex) => {let mat = resolve(ex); Vec2::new(mat.b(), mat.d())},
        VecEx::Top(ex) => {let mat = resolve(ex); Vec2::new(mat.a(), mat.b())},
        VecEx::Bottom(ex) => {let mat = resolve(ex); Vec2::new(mat.c(), mat.d())},
        VecEx::New(ex, ex1) => Vec2::new(resolve(ex), resolve(ex1)),
        VecEx::Literal(vec) => vec,
    }
}