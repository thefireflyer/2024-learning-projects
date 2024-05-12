///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

use pest::{iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;

use super::{ast::*, helper::*};

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

#[derive(Parser)]
#[grammar = "ex3/grammar.pest"]
pub struct ExParser;

///////////////////////////////////////////////////////////////////////////////

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
        .op(Op::infix(r#where, Left))
        .op(
            Op::infix(eq, Left) | Op::infix(neq, Left)
            | Op::infix(ltt, Left) | Op::infix(leq, Left)
            | Op::infix(gtt, Left) | Op::infix(geq, Left)
        )
        .op(Op::infix(add, Left) | Op::infix(sub, Left))
        .op(Op::infix(mul, Left) | Op::infix(div, Left) | Op::infix(r#mod, Left))
        .op(Op::infix(pow, Right))
        .op(Op::postfix(fac))
        .op(Op::prefix(neg))
    };
}

///////////////////////////////////////////////////////////////////////////////

impl<'a> From<Pairs<'a, Rule>> for Ex {
    fn from(value: Pairs<Rule>) -> Self {
        PRATT_PARSER
            .map_primary(|primary| match primary.as_rule() {
                Rule::tok => todo!(),
                Rule::int => Ex::from(primary.as_str().parse::<i32>().unwrap()),
                Rule::flt => Ex::from(primary.as_str().parse::<f64>().unwrap()),
                Rule::var => Ex::from(primary.as_str()),
                Rule::sin => sin(Ex::from(primary.into_inner())),
                Rule::cos => cos(Ex::from(primary.into_inner())),
                Rule::tan => tan(Ex::from(primary.into_inner())),
                Rule::ln => ln(Ex::from(primary.into_inner())),
                Rule::expr => Ex::from(primary.into_inner()),
                Rule::matrix => Ex::Mat(
                    primary
                        .into_inner()
                        .map(|x| Ex::from(x.into_inner()))
                        .collect(),
                ),
                Rule::e => Ex::from(Tok::E),
                Rule::i => Ex::from(Tok::I),
                Rule::pi => Ex::from(Tok::Pi),
                Rule::inf => Ex::from(Tok::Inf),
                _ => unreachable!(),
            })
            .map_prefix(|op, rhs| match op.as_rule() {
                Rule::neg => Ex::Neg(Box::new(rhs)),
                _ => unreachable!(),
            })
            .map_postfix(|_lhs, op| match op.as_rule() {
                Rule::fac => todo!(),
                _ => unreachable!(),
            })
            .map_infix(|lhs, op, rhs| match op.as_rule() {
                Rule::add => lhs + rhs,
                Rule::sub => lhs - rhs,
                Rule::mul => lhs * rhs,
                Rule::div => lhs / rhs,
                Rule::pow => lhs.pow(rhs),
                Rule::r#mod => lhs % rhs,

                Rule::eq => lhs.c(rhs, Rl::Eqq.into()),
                Rule::neq => lhs.c(rhs, Rl::Neq.into()),
                Rule::ltt => lhs.c(rhs, Rl::Ltt.into()),
                Rule::leq => lhs.c(rhs, Rl::Leq.into()),
                Rule::gtt => lhs.c(rhs, Rl::Gtt.into()),
                Rule::geq => lhs.c(rhs, Rl::Geq.into()),

                Rule::r#where => lhs.c(rhs, Rl::Where.into()),
                _ => unreachable!(),
            })
            .parse(value)
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
