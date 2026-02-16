use std::{collections::{HashMap, VecDeque}, io::{self, Write}};

use macroquad::math::Vec2;

use crate::mat2::Mat2;


pub fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input
}

#[derive(Clone)]
pub enum MatEx {
    MatMul(Box<MatEx>, Box<MatEx>),
    MatAdd(Box<MatEx>, Box<MatEx>),
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

#[derive(Clone)]
pub enum VecEx {
    VecMul(Box<MatEx>, Box<VecEx>),
    VecAdd(Box<VecEx>, Box<VecEx>),
    Neg(Box<VecEx>),
    Mul(Box<FloatEx>, Box<VecEx>),
    Div(Box<VecEx>, Box<FloatEx>),
    Rot(Box<FloatEx>),
    Left(Box<MatEx>),
    Right(Box<MatEx>),
    Top(Box<MatEx>),
    Bottom(Box<MatEx>),
    New(Box<FloatEx>, Box<FloatEx>),
    Pow(Box<FloatEx>, Box<VecEx>),
    Literal(Vec2)
}

#[derive(Clone)]
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

#[derive(Clone)]
pub enum Ex {
    Mat(MatEx),
    Vec(VecEx),
    Float(FloatEx)
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

pub enum Line {
    Eval(Ex),
    SetVar(String, Ex)
}


#[derive(PartialEq, Debug, Clone)]
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
    EOF
}

fn make_exp(lhs: Ex, rhs: Ex, op: Token) -> Option<Ex> {
    let result = match lhs {
        Ex::Mat(lhs) => match rhs {
            Ex::Mat(rhs) => match op {
                Token::Add => Ex::Mat(MatEx::MatAdd(Box::new(lhs), Box::new(rhs))),
                Token::Neg => Ex::Mat(MatEx::MatAdd(Box::new(lhs), Box::new(MatEx::Neg(Box::new(rhs))))),
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
                // Token::Pow => todo!(), // Raising matricies to float powers
                _ => return None
            },
        },
        Ex::Vec(lhs) => match rhs {
            Ex::Mat(_rhs) => return None,
            Ex::Vec(rhs) => match op {
                Token::Add => Ex::Vec(VecEx::VecAdd(Box::new(lhs), Box::new(rhs))),
                Token::Neg => Ex::Vec(VecEx::VecAdd(Box::new(lhs), Box::new(VecEx::Neg(Box::new(rhs))))),
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
                Token::Neg => Ex::Float(FloatEx::Add(Box::new(lhs), Box::new(FloatEx::Neg(Box::new(rhs))))),
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
        println!("{inp}");
        
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

enum Node {
    Token(Token),
    Ex(Ex)
}

impl Default for Node {
    fn default() -> Self {
        Self::Token(Token::EOF)
    }
}

struct Lexer<T: Default> {
    data: VecDeque<T>,
    default: T
}

impl<T: Default> Lexer<T> {
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
        let mut tokens: VecDeque<Node> = tokens.into_iter().map(Node::Token).collect();
        tokens.pop_front();
        tokens.pop_front();
        Ok(Line::SetVar(name, pratt_parse(vars, &mut Lexer::new(tokens), 0)?))
    } else {
        let mut tokens = Lexer::new(tokens.into_iter().map(Node::Token).collect());
        Ok(Line::Eval(pratt_parse(vars, &mut tokens, 0)?))
    }
}

fn end_of_ex(token: &Token) -> bool {
    matches!(token, Token::Comma | Token::RBrace | Token::EOF)
}

fn binding_power(token: &Token) -> Option<(u8, u8)> {
    let result = match token {
        Token::Add => (1, 2),
        Token::Mul => (5, 6),
        Token::Neg => (3, 4),
        Token::Pow => (9, 10),
        Token::Cross => (7, 8),
        _ => return None
    };
    Some(result)
}

/// This supports functions with at least one argument
fn parse_func<const N: usize>(vars: &HashMap<String, Obj>, lexer: &mut Lexer<Node>) -> Result<[Ex; N], String> {
    let Node::Token(token) = lexer.next() else {
        panic!("Expression reached from left side when initialising function.")
    };

    if Token::LBrace != token {
        return Err("You must have an open bracket before function use.".to_string())
    }

    let mut result = std::array::repeat(None);

    for n in result[..N-1].iter_mut() {
        *n = Some(pratt_parse(vars, lexer, 0)?);

        let Node::Token(token) = lexer.next() else {
            panic!("Expression reached from left side when parsing next argument in function.")
        };

        if Token::Comma != token {
            return Err("You must have a comma after each argument in a function.".to_string())
        }
    }
    
    result[N - 1] = Some(pratt_parse(vars, lexer, 0)?);

    let Node::Token(token) = lexer.next() else {
        panic!("Expression reached from left side when parsing next argument in function.")
    };

    if Token::RBrace != token {
        return Err("You must have a comma after each argument in a function.".to_string())
    }

    Ok(result.map(|d| d.unwrap()))
}

fn pratt_parse(vars: &HashMap<String, Obj>, lexer: &mut Lexer<Node>, min_bp: u8) -> Result<Ex, String> {
    let mut lhs = match lexer.next() { // todo!() Test "3 +"
        Node::Token(Token::Float(float)) => Ex::Float(FloatEx::Literal(float)),
        Node::Token(Token::VarName(name)) => match vars.get(&name) {
            None => return Err(format!("Variable `{name}` does not exist.")),
            Some(Obj::Float(float)) => Ex::Float(FloatEx::Literal(*float)),
            Some(Obj::Mat(mat)) => Ex::Mat(MatEx::Literal(*mat)),
            Some(Obj::Vec(vec)) => Ex::Vec(VecEx::Literal(*vec)),
        },
        Node::Ex(ex) => ex,
        Node::Token(Token::Hor) => todo!(),
        Node::Token(Token::Vert) => todo!(),
        Node::Token(Token::LBrace) => todo!(),
        Node::Token(Token::Mat) => todo!(),
        Node::Token(Token::Neg) => todo!(),
        Node::Token(Token::RotMat) => todo!(),
        Node::Token(Token::RotVec) => todo!(),
        Node::Token(Token::Vec) => todo!(),
        Node::Token(other) => return Err(format!("Unexpected token `{other:?}`."))
    };

    loop {
        let op = lexer.peek();
        let Node::Token(op) = op else {
            return Err("Expected operation, got value.".to_string());
        };

        if end_of_ex(op) {
            break;
        }
        let Some((l_bp, r_bp)) = binding_power(op) else {
            return Err(format!("Expected operation, got token `{op:?}."))
        };
        
        let Node::Token(op) = lexer.next() else {
            unreachable!("Already checked that lexer.next returns a non-ending token.")
        };

        if l_bp < min_bp {
            break;
        }

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
    match tokenise(&input("> ")) {
        Err(err) => eprintln!("{err}"),
        Ok(tokens) => match make_tree(vars, tokens) {
            Ok(tree) => return Some(tree),
            Err(err) => eprintln!("{err}")
        }
    };
    
    None
}