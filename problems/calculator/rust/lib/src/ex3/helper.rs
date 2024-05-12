///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::ast::*;

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl From<Val> for Ex {
    fn from(value: Val) -> Self {
        Ex::Val(value)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<Fnc> for Ex {
    fn from(value: Fnc) -> Self {
        Ex::Fn(value)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<Vec<Ex>> for Ex {
    fn from(value: Vec<Ex>) -> Self {
        Ex::Mat(value)
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl From<Num> for Ex {
    fn from(value: Num) -> Self {
        Val::Num(value).into()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<&str> for Ex {
    fn from(value: &str) -> Self {
        Val::Var(value.to_owned()).into()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<bool> for Ex {
    fn from(value: bool) -> Self {
        Val::Bool(value).into()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<Tok> for Ex {
    fn from(value: Tok) -> Self {
        Val::Tok(value).into()
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl From<i32> for Ex {
    fn from(value: i32) -> Self {
        Num::Int(value).into()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<f64> for Ex {
    fn from(value: f64) -> Self {
        Num::Flt(value).into()
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl From<Op> for Bin {
    fn from(value: Op) -> Self {
        Bin::Op(value)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl From<Rl> for Bin {
    fn from(value: Rl) -> Self {
        Bin::Rl(value)
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl Add for Ex {
    type Output = Ex;

    fn add(self, rhs: Self) -> Self::Output {
        Ex::Infix(Box::new(self), Op::Add.into(), Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Sub for Ex {
    type Output = Ex;

    fn sub(self, rhs: Self) -> Self::Output {
        Ex::Infix(Box::new(self), Op::Sub.into(), Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Mul for Ex {
    type Output = Ex;

    fn mul(self, rhs: Self) -> Self::Output {
        Ex::Infix(Box::new(self), Op::Mul.into(), Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Div for Ex {
    type Output = Ex;

    fn div(self, rhs: Self) -> Self::Output {
        Ex::Infix(Box::new(self), Op::Div.into(), Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Rem for Ex {
    type Output = Ex;

    fn rem(self, rhs: Self) -> Self::Output {
        Ex::Infix(Box::new(self), Op::Mod.into(), Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Neg for Ex {
    type Output = Ex;

    fn neg(self) -> Self::Output {
        Ex::Neg(Box::new(self))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Ex {
    pub fn pow(self, rhs: Self) -> Self {
        self.c(rhs, Op::Pow.into())
    }

    pub fn c(self, rhs: Self, j: Bin) -> Self {
        Ex::Infix(Box::new(self), j, Box::new(rhs))
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

pub fn sin(e: Ex) -> Ex {
    Fnc::Sin(Box::new(e)).into()
}

pub fn cos(e: Ex) -> Ex {
    Fnc::Cos(Box::new(e)).into()
}

pub fn tan(e: Ex) -> Ex {
    Fnc::Tan(Box::new(e)).into()
}

pub fn ln(e: Ex) -> Ex {
    Fnc::Ln(Box::new(e)).into()
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl Add for Num {
    type Output = Num;

    fn add(self, rhs: Self) -> Self::Output {
        self.either_wrap(rhs, |a, b| a + b, |a, b| a + b)
    }
}

impl Sub for Num {
    type Output = Num;

    fn sub(self, rhs: Self) -> Self::Output {
        self.either_wrap(rhs, |a, b| a - b, |a, b| a - b)
    }
}

impl Mul for Num {
    type Output = Num;

    fn mul(self, rhs: Self) -> Self::Output {
        self.either_wrap(rhs, |a, b| a * b, |a, b| a * b)
    }
}

impl Div for Num {
    type Output = Num;

    fn div(self, rhs: Self) -> Self::Output {
        self.either_wrap(rhs, |a, b| a / b, |a, b| a / b)
    }
}

impl Rem for Num {
    type Output = Num;

    fn rem(self, rhs: Self) -> Self::Output {
        self.either_wrap(rhs, |a, b| a % b, |a, b| a % b)
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl Num {
    pub fn pow(self, rhs: Self) -> Num {
        self.either_wrap(
            rhs,
            |a, b| {
                if b >= 0 {
                    a.pow(b as u32)
                } else {
                    todo!()
                }
            },
            |a, b| a.powf(b),
        )
    }

    fn either_wrap<F: Fn(i32, i32) -> i32, G: Fn(f64, f64) -> f64>(
        self,
        rhs: Self,
        f: F,
        g: G,
    ) -> Num {
        match (self, rhs) {
            (Num::Int(a), Num::Int(b)) => Self::Int(f(a, b)),
            (Num::Int(a), Num::Flt(b)) => Self::Flt(g(a.into(), b)),
            (Num::Flt(a), Num::Int(b)) => Self::Flt(g(a, b.into())),
            (Num::Flt(a), Num::Flt(b)) => Self::Flt(g(a, b)),
        }
    }

    pub fn flt(self) -> f64 {
        match self {
            Num::Int(v) => v.into(),
            Num::Flt(v) => v,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
