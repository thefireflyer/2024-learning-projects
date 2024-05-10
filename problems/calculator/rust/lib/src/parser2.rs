///////////////////////////////////////////////////////////////////////////////

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use crate::Expr;

use self::tokens::{tokenize, Token};

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

mod tokens {
    use std::fmt::{Display, Write};

    //---------------------------------------------------------------------------//

    #[derive(PartialEq, Debug, Clone)]
    pub enum Token {
        Sym(String),
        Num(f64),
        Op(Op),
        Rel(Rel),
        Open(char),
        Close(char),
        Pred,
        Comma,
        Leaf,
        Term,
        Group(char, char, Vec<Token>),
        Infix(Vec<Token>, Box<Token>, Vec<Token>),
        Prefix(Box<Token>, Vec<Token>),
    }

    //---------------------------------------------------------------------------//

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum Op {
        Add,
        Sub,
        Mul,
        Div,
        Pow,
        Mod,
    }

    //---------------------------------------------------------------------------//

    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub enum Rel {
        Eq,
        Le,
        Leq,
        Ge,
        Geq,
        Neg,
    }

    ///////////////////////////////////////////////////////////////////////////////

    impl From<&str> for Token {
        fn from(value: &str) -> Self {
            match value {
                "+" => Token::Op(Op::Add),
                "-" => Token::Op(Op::Sub),
                "*" => Token::Op(Op::Mul),
                "/" => Token::Op(Op::Div),
                "^" => Token::Op(Op::Pow),
                "%" => Token::Op(Op::Mod),
                "=" => Token::Rel(Rel::Eq),
                ">" => Token::Rel(Rel::Ge),
                ">=" => Token::Rel(Rel::Geq),
                "<" => Token::Rel(Rel::Le),
                "<=" => Token::Rel(Rel::Leq),
                "!" => Token::Rel(Rel::Neg),
                "(" => Token::Open('('),
                "[" => Token::Open('['),
                "{" => Token::Open('{'),
                ")" => Token::Close(')'),
                "]" => Token::Close(']'),
                "}" => Token::Close('}'),
                "|" => Token::Pred,
                "," => Token::Comma,
                "" => Token::Leaf,
                ";" => Token::Term,
                _ => {
                    if let Ok(v) = value.parse::<f64>() {
                        Token::Num(v)
                    } else {
                        Token::Sym(value.to_owned())
                    }
                }
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////

    impl Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Sym(arg0) => f.write_fmt(format_args!("{{{}}}", arg0)),
                Self::Num(arg0) => f.write_fmt(format_args!("{}", arg0)),
                Self::Op(arg0) => match arg0 {
                    Op::Add => f.write_str("+"),
                    Op::Sub => f.write_str("-"),
                    Op::Mul => f.write_str("*"),
                    Op::Div => f.write_str("/"),
                    Op::Pow => f.write_str("^"),
                    Op::Mod => f.write_str("%"),
                },
                Self::Rel(arg0) => match arg0 {
                    Rel::Eq => f.write_str("="),
                    Rel::Le => f.write_str("<"),
                    Rel::Leq => f.write_str("<="),
                    Rel::Ge => f.write_str(">"),
                    Rel::Geq => f.write_str(">="),
                    Rel::Neg => f.write_str("!"),
                },
                Self::Open(arg0) => f.write_fmt(format_args!(" `{}", arg0)),
                Self::Close(arg0) => f.write_fmt(format_args!("{}` ", arg0)),
                Self::Pred => write!(f, "|"),
                Self::Comma => write!(f, ","),
                Self::Leaf => write!(f, ""),
                Self::Term => write!(f, ";"),
                Self::Group(arg0, arg1, arg2) => {
                    f.write_char(*arg0)?;
                    for token in arg2 {
                        f.write_fmt(format_args!("{}", token))?;
                    }
                    f.write_char(*arg1)
                }
                Token::Infix(_, _, _) => todo!(),
                Token::Prefix(_, _) => todo!(),
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////

    pub fn tokenize(s: &str) -> Vec<Token> {
        let s = "(".to_owned() + s + ")";

        let mut tokens = vec![];

        let mut start = 0;

        for (end, ch) in s.chars().enumerate() {
            let token = Token::from(ch.to_string().as_str());

            match token {
                Token::Op(_)
                | Token::Rel(_)
                | Token::Open(_)
                | Token::Close(_)
                | Token::Pred
                | Token::Comma
                | Token::Term => {
                    let val = s[start..end].to_owned();
                    tokens.push(Token::from(val.trim()));
                    tokens.push(token);
                    start = end + 1;
                }
                _ => {}
            }
        }

        tokens
    }
}
///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

pub struct LRk {
    fulltext: String,
    tokens: Vec<Token>,
    pos: usize,
    parse_stack: Vec<Token>,
}

///////////////////////////////////////////////////////////////////////////////

impl LRk {
    //-----------------------------------------------------------------------//

    pub fn new(s: &str) -> Self {
        Self {
            fulltext: s.to_owned(),
            tokens: tokenize(s),
            pos: 0,
            parse_stack: vec![],
        }
    }

    //-----------------------------------------------------------------------//

    pub fn step(&mut self) {
        println!(">>> {}", self);
    }

    //-----------------------------------------------------------------------//

    fn shift(&mut self) {}

    //-----------------------------------------------------------------------//

    fn reduce(&mut self) {}

    //-----------------------------------------------------------------------//
}

///////////////////////////////////////////////////////////////////////////////

impl Display for LRk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            f.write_fmt(format_args!("{}", token))?;
        }
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////

pub fn parse(s: &str) -> Result<Expr, Vec<String>> {
    let mut parser = LRk::new(s);

    parser.step();

    Err(vec![])
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn take2parser() {
        eval(vec![
            "",
            "1",
            "1.1",
            "-1.1",
            "x",
            "1+1",
            "1+1.1",
            "1+x",
            "1-1",
            "1+1+1-1*2",
            "1.0+2.43",
            "2*(3+1*x)",
            "x*(((x^2)/x)^1)",
            "x(((x^2)/x)^1)4",
            "+ 1 2",
            "x*((6/14)^2) | x = 5 + 7",
            "[1,2]*x*((6/14)^2) | x = 5 + 7*y | y = 2",
            "f(2*((3/x)^1), y*((5/10)^1), z*((3/z)^1), w)",
        ])
    }

    fn eval(xs: Vec<&str>) {
        for x in xs {
            match parse(x) {
                Ok(mut y) => {
                    println!("{:17} ==> {}", x, y);
                    y.normalize();
                    println!("{:17} ==> {}", "|", y);
                }
                Err(ys) => {
                    println!("{:17} ==> <!> Failed to parse <!>", x);
                    for y in ys {
                        println!("{:17} --> {}", "|", y);
                    }
                }
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
