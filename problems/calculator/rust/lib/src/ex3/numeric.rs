///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

use std::f64::consts::PI;

use super::ast::*;

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl Ex {
    /// Numeric reduction step
    ///
    /// - Collect adj numbers
    /// - Distribute non-adj numbers
    pub fn numeric(&mut self) {
        match self {
            Ex::Val(_) => {}
            Ex::Neg(ex) => {
                ex.numeric();
                match ex.as_mut() {
                    Ex::Neg(ex) => *self = *ex.to_owned(),
                    Ex::Mat(exs) => *self = Ex::Mat(exs.iter().map(|ex| -ex.to_owned()).collect()),
                    Ex::Infix(a, j, b) => {
                        let a = -(*a.to_owned());
                        let b = -(*b.to_owned());
                        *self = a.c(b, j.to_owned());
                    }
                    _ => {}
                }
            }
            Ex::Infix(a, j, b) => {
                a.numeric();
                b.numeric();
                match j {
                    Bin::Op(op) => match (a.as_ref(), &op, b.as_ref()) {
                        (Ex::Val(Val::Num(a)), _, Ex::Val(Val::Num(b))) => match op {
                            Op::Add => *self = Ex::from(a.to_owned() + b.to_owned()),
                            Op::Sub => *self = Ex::from(a.to_owned() - b.to_owned()),
                            Op::Mul => *self = Ex::from(a.to_owned() * b.to_owned()),
                            Op::Pow => *self = Ex::from(a.to_owned().pow(b.to_owned())),
                            Op::Mod => *self = Ex::from(a.to_owned() % b.to_owned()),
                            // Op::Div => *self = Ex::from(a.to_owned() / b.to_owned()),
                            Op::Div => {} // TODO: fix!!
                        },
                        (Ex::Infix(_, _, _), _, _) | (_, _, Ex::Infix(_, _, _)) => {} //TODO: is this correct???
                        _ => {}
                    },
                    Bin::Rl(rl) => {}
                }
            }
            Ex::Fn(f) => match f {
                Fnc::Ln(e) => {
                    e.numeric();
                    match e.as_mut() {
                        Ex::Val(v) => match v {
                            Val::Num(v) => *self = Ex::from(v.flt().ln()),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                Fnc::Sin(e) => {
                    e.numeric();
                    match e.as_mut() {
                        Ex::Val(v) => match v {
                            Val::Num(v) => *self = Ex::from(v.flt().sin()),
                            Val::Tok(Tok::Pi) => *self = Ex::from(PI.sin()),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                Fnc::Cos(e) => {
                    e.numeric();
                    match e.as_mut() {
                        Ex::Val(v) => match v {
                            Val::Num(v) => *self = Ex::from(v.flt().cos()),
                            Val::Tok(Tok::Pi) => *self = Ex::from(PI.cos()),
                            _ => {}
                        },
                        _ => {}
                    }
                }
                Fnc::Tan(e) => {
                    e.numeric();
                    match e.as_mut() {
                        Ex::Val(v) => match v {
                            Val::Num(v) => *self = Ex::from(v.flt().tan()),
                            Val::Tok(Tok::Pi) => *self = Ex::from(PI.tan()),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            },
            Ex::Mat(_) => todo!(),
            Ex::Invalid => {}
        }
    }

    /// Confluent reduction step
    ///
    /// - Only complete for a small set of expressions
    pub fn confluent(&mut self) {
        todo!()
    }

    pub fn gradient_descent(&mut self) {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

// pub struct NumericCalculator {
//     current: Ex,
//     last: Ex,
// }

// ///////////////////////////////////////////////////////////////////////////////

// impl<'a> From<Ex> for NumericCalculator {
//     fn from(value: Ex) -> Self {
//         Self {
//             current: value,
//             last: Ex::Invalid,
//         }
//     }
// }

// ///////////////////////////////////////////////////////////////////////////////

// impl<'a> Iterator for NumericCalculator {
//     type Item = Ex;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.current.rec_step();
//         if self.current != self.last {
//             self.last = self.current.clone();
//             Some(self.current.clone())
//         } else {
//             None
//         }
//     }
// }

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
