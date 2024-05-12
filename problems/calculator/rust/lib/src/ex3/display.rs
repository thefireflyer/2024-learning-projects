///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

use std::fmt::Write;

use super::ast::*;

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Ex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ex::Val(val) => f.write_fmt(format_args!("{:?}", val)),
            Ex::Infix(a, co, b) => match (a.as_ref(), co, b.as_ref()) {
                (Ex::Val(a), co, Ex::Val(b)) => f.write_fmt(format_args!("{:?}{:?}{:?}", a, co, b)),
                (Ex::Val(a), co, b) => f.write_fmt(format_args!("{:?}{:?}({:?})", a, co, b)),
                (a, co, Ex::Val(b)) => f.write_fmt(format_args!("({:?}){:?}{:?}", a, co, b)),
                _ => f.write_fmt(format_args!("({:?}) {:?} ({:?})", a, co, b)),
            },
            Ex::Fn(fnc) => f.write_fmt(format_args!("{:?}", fnc)),
            Ex::Neg(a) => match a.as_ref() {
                Ex::Val(a) => f.write_fmt(format_args!("-{:?}", a)),
                _ => f.write_fmt(format_args!("-({:?})", a)),
            },
            Ex::Mat(a) => f.write_fmt(format_args!("{:?}", a)),
            Ex::Invalid => f.write_str("invalid"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Num(a) => f.write_fmt(format_args!("{:?}", a)),
            Val::Var(a) => f.write_fmt(format_args!("{:?}", a)),
            Val::Bool(a) => f.write_fmt(format_args!("{:?}", a)),
            Val::Tok(a) => f.write_fmt(format_args!("{:?}", a)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Int(a) => f.write_fmt(format_args!("{}", a)),
            Num::Flt(a) => f.write_fmt(format_args!("{:.00001}", a)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Bin::Op(a) => f.write_fmt(format_args!("{:?}", a)),
            Bin::Rl(a) => f.write_fmt(format_args!("{:?}", a)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => f.write_char('+'),
            Op::Sub => f.write_char('-'),
            Op::Mul => f.write_char('*'),
            Op::Div => f.write_char('/'),
            Op::Pow => f.write_char('^'),
            Op::Mod => f.write_char('%'),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Rl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rl::Eqq => f.write_str("="),
            Rl::Ltt => f.write_str("<"),
            Rl::Leq => f.write_str("<="),
            Rl::Gtt => f.write_str(">"),
            Rl::Geq => f.write_str(">="),
            Rl::Neq => f.write_str("!"),
            Rl::Where => f.write_char('|'),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Fnc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fnc::Ln(a) => f.write_fmt(format_args!("ln({:?})", a)),
            Fnc::Sin(a) => f.write_fmt(format_args!("sin({:?})", a)),
            Fnc::Cos(a) => f.write_fmt(format_args!("cos({:?})", a)),
            Fnc::Tan(a) => f.write_fmt(format_args!("tan({:?})", a)),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Debug for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tok::E => f.write_str("e"),
            Tok::Pi => f.write_str("pi"),
            Tok::I => f.write_str("i"),
            Tok::Inf => f.write_str("inf"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
