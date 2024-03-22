////////////////////////////////////////////////////////////////////////////////

/**
 * # Calculator library #######################################################
 *
 * Exposes an expression API for solving and simplifing equations.
 * Computer Algebra System (CAS).
 *
 * ## Actors ##################################################################
 *
 * - Root
 *   - A placeholder container for all other data
 *
 * - Document
 *   - A high level container for other data
 *
 * - Expression
 *   - Primary data type
 *   - Examples
 *     - y=5x+2
 *     - f(x)=x^2-lg(n)
 *
 * - Expression elements
 *   - Constant
 *     - 1
 *     - pi
 *   - Fraction
 *     - 4/5
 *     - 3(2/3)
 *   - Relationship
 *     - x=2
 *     - y<2x
 *   - Variable
 *     - x
 *     - y
 *   - Function
 *     - f(x)
 *     - f(x,y)
 *     - sin(x)
 *   - Operator
 *     - int (x) dx
 *     - (d/dx) 3x^2
 *     - x + 2y
 *     - 5 - f(x)
 *     - 3 * x/2
 *
 * ## Actions #################################################################
 *
 * TODO
 *
 * ## Exposed API #############################################################
 *
 * TODO
 *
 * ## Example Usage ###########################################################
 *
 * TODO
 *
 * ## Used in #################################################################
 *
 * TODO
 *
 * ## License #################################################################
 *
 * AGPLv3+
 *
 * ############################################################################
 */

////////////////////////////////////////////////////////////////////////////////

struct _Root;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct Document {
    contents: Vec<Expr>,
}

//---------------------------------------------------------------------------//

impl Document {
    fn new(contents: Vec<Expr>) -> Self {
        Self { contents }
    }

    fn normalize(&mut self) {
        for expr in &mut self.contents {
            expr.normalize();
        }
    }
}

//---------------------------------------------------------------------------//

impl<'a> From<&'a str> for Document {
    fn from(value: &str) -> Self {
        todo!()
    }
}

//---------------------------------------------------------------------------//

impl<'a> From<Vec<&'a str>> for Document {
    fn from(value: Vec<&str>) -> Self {
        todo!()
    }
}

//---------------------------------------------------------------------------//

impl Default for Document {
    fn default() -> Self {
        Self {
            contents: Default::default(),
        }
    }
}

//---------------------------------------------------------------------------//

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Document:\n")?;
        for expr in &self.contents {
            f.write_fmt(format_args!("> {}\n", expr))?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    // '1
    Int(i32),
    Flt(f64),
    // '1
    Var(&'static str),
    // '1'2'3
    Rel(Box<Expr>, RelType, Box<Expr>),
    // '1(..'2)
    Fn(&'static str, Vec<Expr>),
    // '1'2'3
    Op(Box<Expr>, OpType, Box<Expr>),
    // '1='2
    Where(Box<Expr>, &'static str, Box<Expr>),
}

////////////////////////////////////////////////////////////////////////////////

impl Expr {
    fn unit(co: Expr, n: Expr, d: Expr, p: Expr) -> Self {
        Self::op(
            co,
            OpType::Mul,
            Self::op(
                Self::Op(Box::new(n), OpType::Div, Box::new(d)),
                OpType::Pow,
                p,
            ),
        )
    }

    fn rel(a: Expr, r: RelType, b: Expr) -> Self {
        Self::Rel(Box::new(a), r, Box::new(b))
    }

    fn op(a: Expr, o: OpType, b: Expr) -> Self {
        Self::Op(Box::new(a), o, Box::new(b))
    }

    fn normalize(&mut self) {
        match self {
            Expr::Int(_) => {}
            Expr::Flt(_) => {}
            Expr::Var(_) => {}
            Expr::Rel(a, _, b) => {
                a.normalize();
                b.normalize();
            }
            Expr::Fn(_, args) => {
                for arg in args {
                    arg.normalize();
                }
            }
            Expr::Op(a, op, b) => {
                a.normalize();
                b.normalize();

                match (a.as_ref(), op, b.as_ref()) {
                    // --- float constants ------------------------------------
                    (Expr::Flt(a), OpType::Add, Expr::Flt(b)) => *self = (a + b).into(),
                    (Expr::Flt(a), OpType::Sub, Expr::Flt(b)) => *self = (a - b).into(),
                    (Expr::Flt(a), OpType::Mul, Expr::Flt(b)) => *self = (a * b).into(),
                    (Expr::Flt(a), OpType::Div, Expr::Flt(b)) => *self = (a / b).into(),
                    (Expr::Flt(a), OpType::Pow, Expr::Flt(b)) => *self = (a.powf(*b)).into(),

                    // --- integer constants (preserve integer type) ----------
                    (Expr::Int(a), OpType::Add, Expr::Int(b)) => *self = (a + b).into(),
                    (Expr::Int(a), OpType::Sub, Expr::Int(b)) => *self = (a - b).into(),
                    (Expr::Int(a), OpType::Mul, Expr::Int(b)) => *self = (a * b).into(),
                    (Expr::Int(a), OpType::Div, Expr::Int(b)) if a % b == 0 => {
                        *self = (a / b).into()
                    }
                    (Expr::Int(a), OpType::Pow, Expr::Int(b)) if b > &0 => {
                        *self = (a.pow(*b as u32)).into()
                    }
                    (Expr::Int(a), OpType::Pow, Expr::Int(b)) => {
                        *self = Self::op(1.into(), OpType::Div, a.pow(b.abs() as u32).into())
                    }

                    // --- special multiple -----------------------------------
                    (a, OpType::Mul, b) if a == b => {
                        *self = Self::op(a.clone(), OpType::Pow, 2.into())
                    }
                    // int
                    (a, OpType::Mul, Expr::Int(b)) if b == &1 => *self = a.clone(),
                    (Expr::Int(a), OpType::Mul, b) if a == &1 => *self = b.clone(),
                    (_, OpType::Mul, Expr::Int(b)) if b == &0 => *self = 0.into(),
                    (Expr::Int(a), OpType::Mul, _) if a == &0 => *self = 0.into(),
                    // float
                    (a, OpType::Mul, Expr::Flt(b)) if b == &1.0 => *self = a.clone(),
                    (Expr::Flt(a), OpType::Mul, b) if a == &1.0 => *self = b.clone(),
                    (_, OpType::Mul, Expr::Flt(b)) if b == &0.0 => *self = 0.into(),
                    (Expr::Flt(a), OpType::Mul, _) if a == &0.0 => *self = 0.into(),

                    // --- special division -----------------------------------
                    (a, OpType::Div, b) if a == b => *self = 1.into(),
                    // int
                    (a, OpType::Div, Expr::Int(b)) if b == &1 => *self = a.clone(),
                    (Expr::Int(a), OpType::Mul, _) if a == &0 => *self = 0.into(),
                    // float
                    (a, OpType::Div, Expr::Flt(b)) if b == &1.0 => *self = a.clone(),
                    (Expr::Flt(a), OpType::Mul, _) if a == &0.0 => *self = 0.into(),

                    // --- special power --------------------------------------
                    (a, OpType::Pow, Expr::Int(b)) if b == &1 => {
                        *self = a.clone();
                    }
                    (a, OpType::Pow, Expr::Flt(b)) if b == &1.0 => {
                        *self = a.clone();
                    }
                    (_, OpType::Pow, Expr::Int(b)) if b == &0 => {
                        *self = 1.into();
                    }
                    (_, OpType::Pow, Expr::Flt(b)) if b == &0.0 => {
                        *self = 1.into();
                    }

                    // --- nested ops cases -----------------------------------
                    (Expr::Op(a, OpType::Pow, b), OpType::Pow, c) => {
                        *self = Expr::op(
                            *a.clone(),
                            OpType::Pow,
                            Expr::op(*b.clone(), OpType::Mul, c.clone()),
                        );
                        self.normalize();
                    }

                    // --- factoring cases ------------------------------------
                    // (Expr::Int(a), OpType::Div, Expr::Int(b)) if a / b > 1 => {
                    //     // handle case where n is a multiple of d
                    //     // (d*m)/d -> m OR (n*m)/d -> m(n/d)
                    //     let c = a / b;
                    //     let n = a / c;
                    //     let frac = Expr::op(n.into(), OpType::Div, (*b).into());

                    //     *self = Expr::op(c.into(), OpType::Mul, frac);

                    //     self.normalize();
                    // }
                    (Expr::Int(a), OpType::Div, Expr::Int(b)) if b % a == 0 => {
                        // handle case where d is a constant multiple of n
                        // n/(n*m) -> 1/m
                        let d = b / a;

                        *self = Expr::op(1.into(), OpType::Div, d.into());
                    }
                    (Expr::Int(a), OpType::Div, Expr::Int(b)) if b > &1 => {
                        // Form: n/d
                        // Known constraints:
                        // - n != d
                        // - n % d != 0
                        // - d % n != 0
                        // - 1 < n < d

                        let mut m = 2;
                        let mut gm = 2;

                        // find (n/m)/(d/m) cases
                        // iterate up to the current denominator
                        while &m < b {
                            m += 1;
                            // check if we've found a new smallest denominator
                            //(in other words, the largest factor)
                            if a % &m == 0 && b % &m == 0 {
                                // update tracker
                                gm = m;
                            }
                        }

                        if a % &gm == 0 && b % &gm == 0 && &gm < b {
                            // n and d are both divisible by a common factor less than d
                            // this gives us our lowest common denominator
                            let n = a / gm;
                            let d = b / gm;

                            *self = Self::op(n.into(), OpType::Div, d.into());
                        }
                    }
                    (a, OpType::Div, c) => match (a, c) {
                        (Expr::Op(a, OpType::Mul, b), c) if *(*a) == *c => *self = *b.clone(),
                        (Expr::Op(a, OpType::Mul, b), c) if *(*b) == *c => *self = *a.clone(),
                        (a, Expr::Op(b, OpType::Mul, c)) if *a == *(*b) => {
                            *self = Self::op(1.into(), OpType::Div, *c.clone())
                        }
                        (a, Expr::Op(b, OpType::Mul, c)) if *a == *(*c) => {
                            *self = Self::op(1.into(), OpType::Div, *b.clone())
                        }
                        (Expr::Op(a, OpType::Mul, b), Expr::Op(c, OpType::Mul, d))
                            if *(*a) == *(*c) =>
                        {
                            *self = Self::op(*b.clone(), OpType::Div, *d.clone())
                        }
                        (Expr::Op(a, OpType::Mul, b), Expr::Op(c, OpType::Mul, d))
                            if *(*a) == *(*d) =>
                        {
                            *self = Self::op(*b.clone(), OpType::Div, *c.clone())
                        }
                        (Expr::Op(a, OpType::Mul, b), Expr::Op(c, OpType::Mul, d))
                            if *(*b) == *(*c) =>
                        {
                            *self = Self::op(*a.clone(), OpType::Div, *d.clone())
                        }
                        (Expr::Op(a, OpType::Mul, b), Expr::Op(c, OpType::Mul, d))
                            if *(*b) == *(*d) =>
                        {
                            *self = Self::op(*a.clone(), OpType::Div, *c.clone())
                        }
                        (_, _) => {}
                    },
                    (Expr::Op(a, OpType::Div, b), OpType::Mul, c) => {
                        *self = Expr::op(
                            Expr::Op(a.clone(), OpType::Mul, Box::new(c.clone())),
                            OpType::Div,
                            *b.clone(),
                        );

                        self.normalize();
                    }
                    (c, OpType::Mul, Expr::Op(a, OpType::Div, b)) => {
                        *self = Expr::op(
                            Expr::Op(Box::new(c.clone()), OpType::Mul, a.clone()),
                            OpType::Div,
                            *b.clone(),
                        );

                        self.normalize();
                    }
                    (Expr::Op(a, OpType::Mul, b), OpType::Mul, Expr::Int(c)) => {
                        match (*a.clone(), *b.clone()) {
                            (Expr::Int(a), Expr::Int(b)) => {
                                *self = Self::op((a * c).into(), OpType::Mul, (b * c).into())
                            }
                            (Expr::Int(a), b) => *self = Self::op((a * c).into(), OpType::Mul, b),
                            (a, Expr::Int(b)) => *self = Self::op(a, OpType::Mul, (b * c).into()),
                            (_, _) => {}
                        }

                        self.normalize();
                    }
                    (Expr::Int(c), OpType::Mul, Expr::Op(a, OpType::Mul, b)) => {
                        match (*a.clone(), *b.clone()) {
                            (Expr::Int(a), Expr::Int(b)) => {
                                *self = Self::op((a * c).into(), OpType::Mul, (b * c).into())
                            }
                            (Expr::Int(a), b) => *self = Self::op((a * c).into(), OpType::Mul, b),
                            (a, Expr::Int(b)) => *self = Self::op(a, OpType::Mul, (b * c).into()),
                            (_, _) => {}
                        }

                        self.normalize();
                    }
                    (_, OpType::Mul, _) => {}
                    (Expr::Op(a, OpType::Div, b), OpType::Pow, c) => {
                        *self = Expr::op(
                            Expr::Op(a.clone(), OpType::Pow, Box::new(c.clone())),
                            OpType::Div,
                            Expr::Op(b.clone(), OpType::Pow, Box::new(c.clone())),
                        );

                        self.normalize();
                    }
                    (Expr::Op(a, OpType::Mul, b), OpType::Pow, c) => {
                        *self = Expr::op(
                            Expr::Op(a.clone(), OpType::Pow, Box::new(c.clone())),
                            OpType::Mul,
                            Expr::Op(b.clone(), OpType::Pow, Box::new(c.clone())),
                        );

                        self.normalize();
                    }
                    (_, OpType::Pow, _) => {}
                    (_, OpType::Add, _) => {}
                    (_, OpType::Sub, _) => {}
                }
            }
            Expr::Where(a, sym, b) => {
                b.normalize();
                a.and_where(&sym, b);
                a.normalize();
                *self = *a.clone();
            }
        }
    }

    fn mul_norm(&mut self) {
        todo!()
    }

    fn pow_norm(&mut self) {
        todo!()
    }

    fn collect_norm(&mut self) {
        todo!()
    }

    fn and_where(&mut self, given: &'static str, value: &Expr) {
        match self {
            Expr::Int(_) => {}
            Expr::Flt(_) => {}
            Expr::Var(sym) if *sym == given => *self = value.clone(),
            Expr::Var(_) => {}
            Expr::Rel(a, _, b) => {
                a.and_where(given, value);
                b.and_where(given, value);
            }
            Expr::Fn(_, args) => {
                for arg in args {
                    arg.and_where(given, value);
                }
            }
            Expr::Op(a, _, b) => {
                a.and_where(given, value);
                b.and_where(given, value);
            }
            Expr::Where(a, _, b) => {
                a.and_where(given, value);
                b.and_where(given, value);
            }
        }
    }
}

//---------------------------------------------------------------------------//

impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Self::Flt(value)
    }
}

impl From<&'static str> for Expr {
    fn from(value: &'static str) -> Self {
        Self::Var(value)
    }
}

//---------------------------------------------------------------------------//

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(a) => f.write_fmt(format_args!("{}", a)),
            Expr::Flt(a) => f.write_fmt(format_args!("{}", a)),
            Expr::Var(a) => f.write_str(a),
            Expr::Rel(a, b, c) => f.write_fmt(format_args!("{} {} {}", a, b, c)),
            Expr::Fn(a, args) => {
                f.write_str(a)?;
                f.write_str("(")?;
                let mut args = args.into_iter();
                f.write_fmt(format_args!("{}", args.next().unwrap()))?;
                for arg in args {
                    f.write_fmt(format_args!(", {}", arg))?;
                }
                f.write_str(")")?;
                Ok(())
            }
            Expr::Op(a, o, b) if o == &OpType::Mul => {
                let a = match a.as_ref() {
                    Expr::Int(a) => format!("{}", a),
                    Expr::Flt(a) => format!("{}", a),
                    Expr::Var(a) => format!("{}", a),
                    _ => format!("({})", a),
                };
                let b = match b.as_ref() {
                    Expr::Int(b) => format!("{}", b),
                    Expr::Flt(b) => format!("{}", b),
                    Expr::Var(b) => format!("{}", b),
                    _ => format!("({})", b),
                };
                f.write_fmt(format_args!("{}*{}", a, b))
            }
            Expr::Op(a, o, b) if o == &OpType::Div => {
                let a = match a.as_ref() {
                    Expr::Int(a) => format!("{}", a),
                    Expr::Flt(a) => format!("{}", a),
                    Expr::Var(a) => format!("{}", a),
                    _ => format!("({})", a),
                };
                let b = match b.as_ref() {
                    Expr::Int(b) => format!("{}", b),
                    Expr::Flt(b) => format!("{}", b),
                    Expr::Var(b) => format!("{}", b),
                    _ => format!("({})", b),
                };
                f.write_fmt(format_args!("{}/{}", a, b))
            }
            Expr::Op(a, o, b) if o == &OpType::Pow => {
                let a = match a.as_ref() {
                    Expr::Int(a) => format!("{}", a),
                    Expr::Flt(a) => format!("{}", a),
                    Expr::Var(a) => format!("{}", a),
                    _ => format!("({})", a),
                };
                let b = match b.as_ref() {
                    Expr::Int(b) => format!("{}", b),
                    Expr::Flt(b) => format!("{}", b),
                    Expr::Var(b) => format!("{}", b),
                    _ => format!("({})", b),
                };
                f.write_fmt(format_args!("{}^{}", a, b))
            }
            Expr::Op(a, o, b) => f.write_fmt(format_args!("{} {} {}", a, o, b)),
            Expr::Where(a, b, c) => f.write_fmt(format_args!("{} | {} = {}", a, b, c)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RelType {
    Eq,
    L,
    G,
    Leq,
    Geq,
    Neq,
}

//---------------------------------------------------------------------------//

impl std::fmt::Display for RelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelType::Eq => f.write_str("="),
            RelType::L => f.write_str(">"),
            RelType::G => f.write_str("<"),
            RelType::Leq => f.write_str(">="),
            RelType::Geq => f.write_str("<="),
            RelType::Neq => f.write_str("!="),
        }
    }
}

//---------------------------------------------------------------------------//

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum OpType {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

//---------------------------------------------------------------------------//

impl std::fmt::Display for OpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpType::Add => f.write_str("+"),
            OpType::Sub => f.write_str("-"),
            OpType::Mul => f.write_str("*"),
            OpType::Div => f.write_str("/"),
            OpType::Pow => f.write_str("^"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{Document, Expr, OpType};

    #[test]
    fn document() {
        let mut doc = Document::new(vec![
            3.into(),
            (-4).into(),
            1.2.into(),
            (-30.5).into(),
            "x".into(),
            Expr::unit(1.into(), 1.into(), 1.into(), 1.into()),
            Expr::unit(1.into(), "x".into(), 2.into(), 1.into()),
            Expr::unit(1.into(), 4.into(), 2.into(), 1.into()),
            Expr::unit(1.into(), 3.into(), 2.into(), 1.into()),
            Expr::unit(
                Expr::op("x".into(), OpType::Mul, 3.into()),
                6.into(),
                Expr::op(2.into(), OpType::Add, 5.into()),
                2.into(),
            ),
            Expr::unit(0.into(), 4.into(), 2.into(), 1.into()),
            Expr::unit(2.into(), "x".into(), "x".into(), 1.into()),
            Expr::unit("x".into(), "x".into(), 1.into(), 1.into()),
            Expr::unit("x".into(), 0.into(), 25.into(), 1.into()),
            Expr::unit("x".into(), 57.into(), 25.into(), 0.into()),
            Expr::unit("x".into(), 14.into(), 6.into(), 2.into()),
            Expr::unit(
                "x".into(),
                Expr::op("x".into(), OpType::Pow, 2.into()),
                "x".into(),
                1.into(),
            ),
            Expr::op(
                Expr::unit(
                    1.into(),
                    Expr::op("x".into(), OpType::Pow, 2.into()),
                    1.into(),
                    2.into(),
                ),
                OpType::Pow,
                2.into(),
            ),
            Expr::Fn(
                "sin",
                vec![Expr::unit(2.into(), 3.into(), "x".into(), 1.into())],
            ),
            Expr::Fn(
                "f",
                vec![
                    Expr::unit(2.into(), 3.into(), "x".into(), 1.into()),
                    Expr::unit("y".into(), 5.into(), 10.into(), 1.into()),
                    Expr::unit("z".into(), 3.into(), "z".into(), 1.into()),
                ],
            ),
            Expr::rel(
                Expr::Fn("f", vec!["x".into(), "y".into(), "z".into()]),
                crate::RelType::Eq,
                Expr::unit("x".into(), "y".into(), "z".into(), 2.into()),
            ),
            Expr::unit("x".into(), 6.into(), 14.into(), 2.into()),
            Expr::op(7.into(), OpType::Div, 14.into()),
            Expr::Where(
                Box::new(Expr::unit("x".into(), 6.into(), 14.into(), 2.into())),
                "x",
                Box::new(Expr::op(5.into(), OpType::Add, 7.into())),
            ),
        ]);

        println!("{}\n", doc);
        doc.normalize();
        println!("{}", doc);
    }
}

////////////////////////////////////////////////////////////////////////////////
