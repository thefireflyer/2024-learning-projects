////////////////////////////////////////////////////////////////////////////////

use std::fmt::{Display, Write};

use crate::Expr;

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone)]
enum Token {
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
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Rel {
    Eq,
    Le,
    Leq,
    Ge,
    Geq,
    Neg,
}

////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////

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
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug, Clone)]
enum Log {
    Received(String),
    Read(Token),
    UnknownToken(String, usize),
    UnexpectedToken(String, usize, String),
    MismatchedDelimiter(char, usize),
    Tokenized(Vec<Token>),
    Conversion(String),
}

////////////////////////////////////////////////////////////////////////////////

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Log::Tokenized(tokens) => {
                for token in tokens {
                    f.write_fmt(format_args!("{}", token))?;
                }
                Ok(())
            }
            _ => f.write_fmt(format_args!("{:?}", self)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn tokenize(s: &str, log: &mut Vec<Log>) -> Vec<Token> {
    let mut chunks = vec![];

    let mut start = 0;

    for (end, ch) in s.chars().enumerate() {
        let token = Token::from(ch.to_string().as_str());

        // log.push(Log::Read(token.clone()));

        match token {
            Token::Op(_)
            | Token::Rel(_)
            | Token::Open(_)
            | Token::Close(_)
            | Token::Pred
            | Token::Comma
            | Token::Term => {
                let val = s[start..end].to_owned();
                chunks.push(Token::from(val.trim()));
                chunks.push(token);
                start = end + 1;
            }
            _ => {}
        }
    }

    log.push(Log::Tokenized(chunks.clone()));

    chunks
}

////////////////////////////////////////////////////////////////////////////////

fn parse(s: &str) -> Result<Expr, Vec<Log>> {
    let s = s.trim();
    let s = "(".to_owned() + s + ")";

    let mut log: Vec<Log> = vec![Log::Received(s.to_owned())];

    let mut tokens = tokenize(&s, &mut log);

    // println!();

    fn group_by<S: Fn(&Token) -> bool, E: Fn(&Token) -> bool>(
        tokens: &mut Vec<Token>,
        log: &mut Vec<Log>,
        start: S,
        end: E,
        start_ch: char,
        end_ch: char,
    ) {
        while let Some(end) = tokens.iter().position(&end) {
            if let Some(start) = tokens[..end].iter().rposition(&start) {
                let valid = &tokens[start + 1..end];
                // println!(">> {:?}", valid);
                *tokens = [
                    tokens[..start].to_vec(),
                    vec![Token::Group(start_ch, end_ch, valid.to_vec())],
                    tokens[end + 1..].to_vec(),
                ]
                .concat();
            } else {
                println!("erm");
            }
        }
    }

    group_by(
        &mut tokens,
        &mut log,
        |x| x == &Token::Open('('),
        |x| x == &Token::Close(')'),
        '(',
        ')',
    );

    log.push(Log::Tokenized(tokens.clone()));

    fn rec_filtered_map(
        tokens: &mut Vec<Token>,
        log: &mut Vec<Log>,
        filter: &(dyn Fn(&Token) -> bool),
        map: &(dyn Fn(&mut Token)),
    ) {
        for token in tokens {
            match token {
                Token::Group(st, en, inner) => rec_filtered_map(inner, log, filter, map),
                _ => {
                    if filter(&token) {
                        map(token)
                    }
                }
            }
        }
    }

    fn rec_group_by(
        tokens: &mut Vec<Token>,
        log: &mut Vec<Log>,
        start: &(dyn Fn(&Token) -> bool),
        end: &(dyn Fn(&Token) -> bool),
        start_ch: char,
        end_ch: char,
    ) {
        for token in tokens.iter_mut() {
            match token {
                Token::Group(st, en, inner) => {
                    rec_group_by(inner, log, start, end, start_ch, end_ch)
                }
                _ => {
                    if start(&token) {
                        group_by(tokens, log, start, end, start_ch, end_ch);
                        break;
                    }
                }
            }
        }
    }

    rec_group_by(
        &mut tokens,
        &mut log,
        &|x| x == &Token::Open('['),
        &|x| x == &Token::Close(']'),
        '[',
        ']',
    );

    // log.push(Log::Tokenized(tokens.clone()));

    rec_group_by(
        &mut tokens,
        &mut log,
        &|x| x == &Token::Open('{'),
        &|x| x == &Token::Close('}'),
        '{',
        '}',
    );

    // log.push(Log::Tokenized(tokens.clone()));

    fn seperate_by<P: Fn(&Token) -> bool>(
        tokens: &mut Vec<Token>,
        log: &mut Vec<Log>,
        predicate: &P,
    ) {
        if let Some(first) = tokens.iter().position(predicate) {
            let valid = &tokens[..first];
            // println!(">> {:?}", valid);

            if first + 1 < tokens.len() {
                let mut rest = tokens[first + 1..].to_vec();

                seperate_by(&mut rest, log, predicate);
                *tokens = [
                    vec![Token::Group('(', ')', valid.to_vec())],
                    tokens[first..first + 1].to_vec(),
                    vec![Token::Group('(', ')', rest)],
                ]
                .concat();
                // print!("-> ");
                // for token in tokens.iter() {
                // print!("{}", token);
                // }
                // println!();
            }
        }
    }

    fn rec_seperate_by(
        tokens: &mut Vec<Token>,
        log: &mut Vec<Log>,
        predicate: &(dyn Fn(&Token) -> bool),
    ) {
        for token in tokens.iter_mut() {
            match token {
                Token::Group(st, en, inner) => rec_seperate_by(inner, log, predicate),
                _ => {
                    if predicate(&token) {
                        seperate_by(tokens, log, &predicate);
                        break;
                    }
                }
            }
        }
    }

    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Term);
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Pred);
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| match x {
        Token::Rel(rel) => true,
        _ => false,
    });
    // log.push(Log::Tokenized(tokens.clone()));

    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Comma);
    // log.push(Log::Tokenized(tokens.clone()));

    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Add));
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Sub));
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Mod));
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Mul));
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Div));
    // log.push(Log::Tokenized(tokens.clone()));
    rec_seperate_by(&mut tokens, &mut log, &|x| x == &Token::Op(Op::Pow));
    log.push(Log::Tokenized(tokens.clone()));

    fn single(token: &Token, log: &mut Vec<Log>) -> Result<Expr, String> {
        match token {
            Token::Sym(x) => Ok(Expr::Var(x.to_owned())),
            Token::Num(x) => Ok(Expr::Flt(*x)),
            Token::Op(_) => Err("Unexpected operator".to_owned()),
            Token::Rel(_) => Err("Unexpected relation".to_owned()),
            Token::Open(_) => Err("Unexpected open".to_owned()),
            Token::Close(_) => Err("Unexpected close".to_owned()),
            Token::Pred => Err("Unexpected predicate".to_owned()),
            Token::Comma => Err("Unexpected comma".to_owned()),
            Token::Leaf => Ok(Expr::Leaf),
            Token::Term => todo!(),
            Token::Group(_, _, inner) => rec_construct(&inner, log),
        }
    }

    fn duo(t0: &Token, t1: &Token, log: &mut Vec<Log>) -> Result<Expr, String> {
        match (t0, t1) {
            (Token::Op(Op::Sub), b) => {
                // unary negative
                todo!()
            }
            (Token::Op(Op::Add), b) => {
                // unary positive
                todo!()
            }
            (Token::Rel(Rel::Neg), b) => {
                // unary negate
                todo!()
            }
            (Token::Sym(s), Token::Group('(', ')', inner)) if inner.contains(&Token::Comma) => {
                // this is a function
                Ok(Expr::Fn(
                    s.to_owned(),
                    inner
                        .into_iter()
                        .filter_map(|token| {
                            if token != &Token::Comma {
                                Some(single(token, log).expect("Error in function argument"))
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            }
            (Token::Sym(s), b) | (b, Token::Sym(s)) => {
                // implicit multiplication
                Ok(Expr::op(
                    Expr::Var(s.to_string()),
                    crate::OpType::Mul,
                    single(b, log)?,
                ))
            }
            (Token::Num(x), b) | (b, Token::Num(x)) => {
                // implicit multiplication
                Ok(Expr::op(Expr::Flt(*x), crate::OpType::Mul, single(b, log)?))
            }
            (Token::Group(_, _, _), Token::Group(_, _, _)) => {
                // implicit multiplication
                Ok(Expr::op(
                    single(t0, log)?,
                    crate::OpType::Mul,
                    single(t1, log)?,
                ))
            }
            (a, Token::Leaf) | (Token::Leaf, a) => {
                // ignore leafs
                single(a, log)
            }
            _ => Err("Invalid syntax pair".to_owned()),
        }
    }

    fn rec_construct(tokens: &[Token], log: &mut Vec<Log>) -> Result<Expr, String> {
        match (tokens.get(0), tokens.get(1), tokens.get(2)) {
            (None, None, None) => Ok(Expr::Leaf),
            (Some(token), None, None) => single(token, log),
            (Some(t0), Some(t1), None) => duo(t0, t1, log),
            (Some(t0), Some(t1), Some(t2)) => match (t0, t1, t2) {
                //-----------------------------------------------------------//
                (a, Token::Leaf, Token::Leaf)
                | (Token::Leaf, Token::Leaf, a)
                | (Token::Leaf, a, Token::Leaf) => single(a, log),
                (a, b, Token::Leaf) | (a, Token::Leaf, b) | (Token::Leaf, a, b) => duo(a, b, log),
                //-----------------------------------------------------------//
                (_, Token::Group(_, _, _), _) => {
                    // implicit multiplication
                    Ok(Expr::op(
                        single(t0, log)?,
                        crate::OpType::Mul,
                        duo(t1, t2, log)?,
                    ))
                }
                //-----------------------------------------------------------//
                (a, Token::Op(Op::Add), b) => Ok(Expr::op(
                    single(a, log)?,
                    crate::OpType::Add,
                    single(b, log)?,
                )),
                (a, Token::Op(Op::Sub), b) => Ok(Expr::op(
                    single(a, log)?,
                    crate::OpType::Sub,
                    single(b, log)?,
                )),
                (a, Token::Op(Op::Mul), b) => Ok(Expr::op(
                    single(a, log)?,
                    crate::OpType::Mul,
                    single(b, log)?,
                )),
                (a, Token::Op(Op::Div), b) => Ok(Expr::op(
                    single(a, log)?,
                    crate::OpType::Div,
                    single(b, log)?,
                )),
                (a, Token::Op(Op::Pow), b) => Ok(Expr::op(
                    single(a, log)?,
                    crate::OpType::Pow,
                    single(b, log)?,
                )),
                //-----------------------------------------------------------//
                (a, Token::Rel(Rel::Eq), b) => Ok(Expr::rel(
                    single(a, log)?,
                    crate::RelType::Eq,
                    single(b, log)?,
                )),
                (a, Token::Rel(Rel::Ge), b) => Ok(Expr::rel(
                    single(a, log)?,
                    crate::RelType::G,
                    single(b, log)?,
                )),
                (a, Token::Rel(Rel::Geq), b) => Ok(Expr::rel(
                    single(a, log)?,
                    crate::RelType::Geq,
                    single(b, log)?,
                )),
                (a, Token::Rel(Rel::Le), b) => Ok(Expr::rel(
                    single(a, log)?,
                    crate::RelType::L,
                    single(b, log)?,
                )),
                (a, Token::Rel(Rel::Leq), b) => Ok(Expr::rel(
                    single(a, log)?,
                    crate::RelType::Leq,
                    single(b, log)?,
                )),
                //-----------------------------------------------------------//
                (a, Token::Pred, b) => {
                    todo!()
                }
                //-----------------------------------------------------------//
                _ => Err(format!("Invalid syntax trio: {:?}", tokens)),
            },
            _ => panic!("????"),
        }
    }

    match rec_construct(&tokens, &mut log) {
        Ok(res) => Ok(res),
        Err(err) => {
            log.push(Log::Conversion(err));
            Err(log)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn start() {
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
            "ln(0)",
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
                    println!("{:17} --> {}", "|", y);
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

////////////////////////////////////////////////////////////////////////////////
