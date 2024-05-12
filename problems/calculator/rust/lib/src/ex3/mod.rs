///////////////////////////////////////////////////////////////////////////////
//!
//!
//!
///////////////////////////////////////////////////////////////////////////////

pub mod ast;
pub mod display;
pub mod helper;
pub mod numeric;
pub mod parser;

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

pub const TERMINAL_WIDTH: usize = 80;

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::vec;

    use pest::Parser;

    use crate::ex3::{
        ast::Ex,
        parser::{ExParser, Rule},
        TERMINAL_WIDTH,
    };

    #[test]
    fn ex3_test_parser() {
        eval(vec![
            "1",
            "1.1",
            "-1",
            "-1.1",
            "1+1",
            "1-1",
            "1-1.1",
            "-(-1.1)",
            "1+1+1-1*2",
            "-(1+1+1-1*2)",
            "1.0+2.43",
            "(2*5)^2/1+3",
        ]);

        eval(vec![
            "sin(0)",
            "cos(0)",
            "tan(0)",
            "sin(pi)",
            "cos(pi)",
            "tan(pi)",
            "sin(pi/2)",
            "cos(pi/2)",
            "tan(pi/2)",
            "ln(0)",
            "ln(1)",
            "ln(e^3)",
        ]);

        eval(vec![
            "x",
            "-x",
            "x+y+0.5-1",
            "x+x",
            "x*x",
            "x/x",
            "x/(x^2)",
            "sin(x)",
            "cos(x)",
            "tan(x)",
            "ln(e^x)",
            "2*(3+1*x)",
            "x*(((x^2)/x)^1)",
            "x*((6/14)^2) | x = 5 + 7",
        ]);
    }

    fn eval(xs: Vec<&str>) {
        println!("\n{:->TERMINAL_WIDTH$}", "");
        let width = xs
            .iter()
            .map(|x| {
                if x.len() > TERMINAL_WIDTH / 3 {
                    0
                } else {
                    x.len()
                }
            })
            .max()
            .unwrap_or(0);

        for (_, x) in xs.into_iter().enumerate() {
            match ExParser::parse(Rule::program, x) {
                Ok(mut pairs) => {
                    // println!("{:-<80}\n", "");
                    let y = Ex::from(pairs.next().unwrap().into_inner());
                    // let numeric = y.numeric_reducer();

                    let mut y1 = y.clone();
                    y1.numeric();

                    if x.len() > width {
                        println!("{:->TERMINAL_WIDTH$}", "");
                        println!("{:?}\n", x);
                        println!("{:?}\n", y);
                        println!("{:->TERMINAL_WIDTH$}", "");
                    } else {
                        println!("{:width$} ==> {:?}", x, y);
                        // for step in numeric {
                        //     println!("{:width$} --> {:?}", "", step);
                        // }
                        println!("{:width$} --> {:?}", "", y1);
                    }
                    // println!("\n{:?}\n", pairs);
                    // println!("\n{:#?}\n", pairs.peek().unwrap().into_inner());
                    // println!("\n{:?}\n", Ex::from(pairs.next().unwrap().into_inner()));
                }
                Err(e) => {
                    println!("{:20} ==> <!> Failed to parse <!>\n\n{:#?}\n\n", x, e);
                }
            }
        }
        println!("{:->TERMINAL_WIDTH$}\n", "");
    }
}

///////////////////////////////////////////////////////////////////////////////
//---------------------------------------------------------------------------//
///////////////////////////////////////////////////////////////////////////////
