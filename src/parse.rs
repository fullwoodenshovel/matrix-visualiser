use std::io;


pub fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub enum MatEx {
    MatMul(Box<MatEx>, Box<MatEx>),
    MatAdd(Box<MatEx>, Box<MatEx>),
    Neg(Box<MatEx>),
    Mul(Box<FloatEx>, Box<MatEx>),
    Rot(Box<FloatEx>),
    New(Box<FloatEx>, Box<FloatEx>, Box<FloatEx>, Box<FloatEx>),
    Vert(Box<VecEx>, Box<VecEx>),
    Hor(Box<VecEx>, Box<VecEx>)
}

pub enum VecEx {
    VecMul(Box<MatEx>, Box<VecEx>),
    VecAdd(Box<VecEx>, Box<VecEx>),
    Neg(Box<VecEx>),
    Mul(Box<FloatEx>, Box<VecEx>),
    Rot(Box<FloatEx>),
    Left(Box<MatEx>),
    Right(Box<MatEx>),
    Top(Box<MatEx>),
    Bottom(Box<MatEx>),
    New(Box<FloatEx>, Box<FloatEx>),
    Dot(Box<VecEx>),
    Cross(Box<VecEx>),
    Pow(Box<FloatEx>, Box<VecEx>)
}

pub enum FloatEx {
    TopLeft(Box<MatEx>),
    TopRight(Box<MatEx>),
    BottomLeft(Box<MatEx>),
    BottomRight(Box<MatEx>),
    X(Box<VecEx>),
    Y(Box<VecEx>),
    Mul(Box<FloatEx>),
    Pow(Box<FloatEx>),
    Add(Box<FloatEx>),
    Neg(Box<FloatEx>),
    Literal(f32)
}

pub enum Ex {
    Mat(MatEx),
    Vec(VecEx),
    Float(FloatEx)
}

pub enum Line {
    Eval(Ex),
    SetVar(String, Ex)
}


#[derive(PartialEq, Debug)]
pub enum Token {
    LBrace,
    RBrace,
    Float(f32),
    Mat,
    Vec,
    Comma,
    Add,
    Mul,
    Neg,
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
    VarName(String)
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
            // match residual {
            //     "**" => result.push(Token::Pow),
            //     _value => {
            //         return false
            //     }
            // }
            true
        }
        
        for char in inp.chars() {
            let mut added_to_residual = false;
            let mut push = None;
            match char {
                '(' => push = Some(Token::LBrace),
                ')' => push = Some(Token::RBrace),
                ',' => push = Some(Token::Comma),
                '+' => push = Some(Token::Add),
                '*' => push = Some(Token::Mul),
                '-' => push = Some(Token::Neg),
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
                println!("{result:?}");
            } else if !added_to_residual && !residual.is_empty() {
                append_residual(&mut result, &residual);
                residual = String::new();
                println!("{result:?}");
            }
            if let Some(value) = push {
                result.push(value);
                println!("{result:?}");
            }
        }
        if !residual.is_empty() {
            append_residual(&mut result, &residual);
            residual = String::new();
            println!("{result:?}");
        }
    }
    Ok(result)
}

pub fn parse_exp() -> Option<Line> {
    let inp = input("> ");
    todo!()
}