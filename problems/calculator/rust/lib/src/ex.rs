///////////////////////////////////////////////////////////////////////////////
//! === Take 4 ================================================================
//!
//! Goal: higher level of abstract and loose p-code
//!
///////////////////////////////////////////////////////////////////////////////

use std::collections::{HashMap, HashSet};

///////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Clone)]
enum Ex {
    Value(Val),
    Infix(Box<Ex>, Co, Box<Ex>),
    Prefix(Co, Vec<Ex>),
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Val {
    Num(Num),
    Var(String),
    Bool(bool),
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Num {
    Int(i32),
    Flt(f64),
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Co {
    Op(Op),
    Rl(Rl),
    Fc(Fc),
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Rl {
    Eq,
    Le,
    Ge,
    Nt,
}

//---------------------------------------------------------------------------//

#[derive(PartialEq, Clone)]
enum Fc {}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// Expression
/// |   Value           | A fundamental primitive value. Cannot be simplified.
/// |   |   Number      |
/// |   |   Variable    |
/// |   |   Boolean     |
/// |   Infix           | A basic structural element. May be simplified.
/// |   |   Operator    |
/// |   |   Relation    |
/// |   |   Function    |
/// |   Prefix          | A basic structural element. May be simplified.
/// |   |   Operator    |
/// |   |   Relation    |
/// |   |   Function    |
///
///-------------------------------------------------------------------------///
///
/// a,b     :number
/// x,y,z   :variable
/// f,g,h   :function
/// m,n,w   :stable block
/// <>      :operation
/// <?>     :relation
///
/// --- Stable forms -------------------------------------------------------///
///
/// - operators on primitives
///
/// :: a
///
/// :: x
///
/// :: x + a
///
/// :: x - a
///
/// :: ax
///
/// :: x/a
///
/// :: x^a
///
/// :: x<>a
///
/// - functions on primitives
///
/// :: f(a)
///
/// :: f(x)
///
/// - relations on primitives
///
/// :: x <?> a
///
/// :: a <?> b
///
/// - operators on compounds
///
/// :: x^a+x^b
///
/// :: x^a<>y^b
///
/// - relations on compounds
///
/// :: y <?> x<>a
///
/// :: y <?> f(a)
///
/// :: y <?> f(x)
///
///---- Mutations ----------------------------------------------------------///
///
/// :: a        :>>: a<>b
///
/// :: x        :>>: x<>b
///
/// :: x        :>>: x<>y
///
/// :: m        :>>: m<>n
///
/// :: m<?>n    :>>: m<>w<?>n<>w
///
/// :: f(m)     :>>: f(m)<>n
///
///-------------------------------------------------------------------------///
///
/// When are infix expressions stable?
///
/// - Left side needs to be stable
/// - Right side needs to be stable
/// - If fraction a/b
///     - a is not divisible by b
///     - b is not divisible by a
///     - b is the lowest common denominator of a
///     - (optional) a < b
/// - If sum a+b
///     - If a and b are numbers
///         - a+b
///     - If a == b
///         - 2a (unstable)
///     - If a and b share factors
///         - i.e. 2x+6x => 8x      (unstable)
///         - i.e. ab+ac => a(b+c)  (unstable)
///             - counter example: x^2 + x =//=> x(x+1)
///                 - the common factor is a unknown variable
/// - If product ab
///     -
///
///
///////////////////////////////////////////////////////////////////////////////

fn simplify(value: Ex) -> Ex {
    match value {
        Ex::Value(_) => value,         // cannot be simplified
        Ex::Infix(_, _, _) => todo!(), // can be simplified
        Ex::Prefix(_, _) => todo!(),   // can be simplified
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl Ex {
    fn is_unit(&self) -> bool {
        match self {
            Ex::Value(_) => true,
            _ => false,
        }
    }

    fn is_num(&self) -> bool {
        match self {
            Ex::Value(Val::Num(_)) => true,
            _ => false,
        }
    }

    fn is_var(&self) -> bool {
        match self {
            Ex::Value(Val::Var(_)) => true,
            _ => false,
        }
    }

    fn is_stable(&self) -> bool {
        match self {
            Ex::Value(_) => true,
            Ex::Infix(a, _, b) => {
                a.is_stable() && b.is_stable() && !a.is_factor(&b) && !b.is_factor(&a)
            } // limited
            Ex::Prefix(_, a) => a.iter().all(|a| a.is_stable()), // very limited
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Ex {
    fn is_factor(&self, other: &Self) -> bool {
        match (self, other) {
            (_, _) if self == other => true,
            (Ex::Value(a), Ex::Value(b)) => a.is_factor(b),
            (Ex::Value(_), Ex::Infix(a, _, b)) => a.is_factor(self) || b.is_factor(self),
            (Ex::Value(_), Ex::Prefix(_, _)) => todo!(),
            (Ex::Infix(a, _, b), Ex::Value(_)) => a.is_factor(other) || b.is_factor(other),
            (Ex::Infix(a, _, b), Ex::Infix(c, _, d)) => {
                a.is_factor(c) || a.is_factor(d) || b.is_factor(c) || b.is_factor(d)
            }
            (Ex::Infix(_, _, _), Ex::Prefix(_, _)) => todo!(),
            (Ex::Prefix(_, _), Ex::Value(_)) => todo!(),
            (Ex::Prefix(_, _), Ex::Infix(_, _, _)) => todo!(),
            (Ex::Prefix(_, _), Ex::Prefix(_, _)) => todo!(),
        }
    }
}

//---------------------------------------------------------------------------//

impl Val {
    fn is_factor(&self, other: &Self) -> bool {
        match (self, other) {
            (Val::Num(a), Val::Num(b)) => a.is_factor(b),
            (Val::Var(a), Val::Var(b)) if a == b => true,
            _ => false, // mostly nonsensical combos
        }
    }
}

//---------------------------------------------------------------------------//

impl Num {
    fn is_factor(&self, other: &Self) -> bool {
        match (self, other) {
            (Num::Int(a), Num::Int(b)) => a % b == 0,
            (Num::Int(_), Num::Flt(_)) => true, // floats should always be
            (Num::Flt(_), Num::Int(_)) => true, // evaluated
            (Num::Flt(_), Num::Flt(_)) => true,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

struct Mutator {
    visited: HashSet<Ex>,
    current: Vec<Ex>,
    front: Vec<Ex>,
    paths: HashMap<Ex, Vec<Ex>>,
    index: usize,
}

//---------------------------------------------------------------------------//

impl Iterator for Mutator {
    type Item = Ex;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

impl From<i32> for Num {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Num {
    fn from(value: f64) -> Self {
        Self::Flt(value)
    }
}

//---------------------------------------------------------------------------//

impl From<&str> for Val {
    fn from(value: &str) -> Self {
        Self::Var(value.to_owned())
    }
}

impl From<Num> for Val {
    fn from(value: Num) -> Self {
        Self::Num(value)
    }
}

//---------------------------------------------------------------------------//

impl From<Val> for Ex {
    fn from(value: Val) -> Self {
        Self::Value(value)
    }
}

impl From<(Ex, Co, Ex)> for Ex {
    fn from((a, co, b): (Ex, Co, Ex)) -> Self {
        Self::Infix(Box::new(a), co, Box::new(b))
    }
}

impl From<(Co, Vec<Ex>)> for Ex {
    fn from((co, a): (Co, Vec<Ex>)) -> Self {
        Self::Prefix(co, a)
    }
}

//---------------------------------------------------------------------------//

impl From<Op> for Co {
    fn from(value: Op) -> Self {
        Self::Op(value)
    }
}

impl From<Rl> for Co {
    fn from(value: Rl) -> Self {
        Self::Rl(value)
    }
}

impl From<Fc> for Co {
    fn from(value: Fc) -> Self {
        Self::Fc(value)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl std::fmt::Display for Ex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render_plain())
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Ex {
    fn render_plain(&self) -> String {
        match self {
            Ex::Value(value) => value.render_plain(),
            Ex::Infix(a, co, b) => match (a.as_ref(), co, b.as_ref()) {
                (Ex::Value(a), co, Ex::Value(b)) => {
                    a.render_plain() + &co.render_plain() + &b.render_plain()
                }
                (Ex::Value(a), co, b) => {
                    a.render_plain() + &co.render_plain() + "(" + &b.render_plain() + ")"
                }
                (a, co, Ex::Value(b)) => {
                    "(".to_owned()
                        + &a.render_plain()
                        + ")"
                        + &co.render_plain()
                        + &b.render_plain()
                }
                _ => {
                    "(".to_owned()
                        + &a.render_plain()
                        + ")"
                        + &co.render_plain()
                        + "("
                        + &b.render_plain()
                        + ")"
                }
            },
            Ex::Prefix(co, a) => {
                co.render_plain()
                    + "("
                    + &a.iter()
                        .fold("".to_owned(), |acc, a| acc + &a.render_plain())
                    + ")"
            }
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Val {
    fn render_plain(&self) -> String {
        match self {
            Val::Num(Num::Int(a)) => a.to_string(),
            Val::Num(Num::Flt(a)) => a.to_string(),
            Val::Var(x) => x.to_owned(),
            Val::Bool(a) => a.to_string(),
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Co {
    fn render_plain(&self) -> String {
        match self {
            Co::Op(op) => op.render_plain(),
            Co::Rl(rl) => rl.render_plain(),
            Co::Fc(fc) => fc.render_plain(),
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Op {
    fn render_plain(&self) -> String {
        match self {
            Op::Add => "+".to_owned(),
            Op::Sub => "-".to_owned(),
            Op::Mul => "*".to_owned(),
            Op::Div => "/".to_owned(),
            Op::Pow => "^".to_owned(),
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Rl {
    fn render_plain(&self) -> String {
        match self {
            Rl::Eq => "=".to_owned(),
            Rl::Le => "<".to_owned(),
            Rl::Ge => ">".to_owned(),
            Rl::Nt => "!".to_owned(),
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Fc {
    fn render_plain(&self) -> String {
        match self {
            _ => "".to_owned(),
        }
    }

    //-----------------------------------------------------------------------//

    fn render_latex(&self) -> String {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

trait ExS {
    fn render_plain(&self) -> String;
    fn render_latex(&self) -> String;

    fn parse(val: &str) -> Result<Ex, String>;

    fn eval(&self) -> Result<f64, Ex>;

    fn add(&self, other: &Self) -> Ex;
    fn sub(&self, other: &Self) -> Ex;
    fn mul(&self, other: &Self) -> Ex;
    fn div(&self, other: &Self) -> Ex;
    fn pow(&self, other: &Self) -> Ex;

    fn rem(&self, other: &Self) -> Ex;

    fn eq(&self, other: &Self) -> Result<bool, Ex>;
    fn apprx(&self, other: &Self) -> Result<bool, Ex>;
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    #[test]
    fn ex() {}
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
