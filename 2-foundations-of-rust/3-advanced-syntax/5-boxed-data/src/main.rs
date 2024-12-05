/// Below you find a small start of a data type modelling the abstract syntax tree for an expression,
/// and a small evaluator function.
///
/// Please extend this evaluator in the following ways:
///
/// - Add support for multiplication and division
///
/// - We have added the form "Summation(Vec<Expr>)", representing the sum of a list of expressions.
/// Question: why can we get away with Vec<Expr> enough in that case, instead of Box<Vec<Expr>> ?
/// Answer: Because Vec already holds it's content on the heap
///
/// - EXTRA: Since division can fail, the function eval needs to return an Option<i64>, where None indicates that a division by
///   zero has occurred. Can you change the code so that that errors are propagated correctly? (hint: use the ? syntax).

#[derive(PartialEq, Debug)]
enum Expr {
    Const(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var,
    Summation(Vec<Expr>),
    Sigma(i64, i64, Box<Expr>),
}

// These are convenience functions, so you don't have to type "Box::new" as often
// when building test-data types
fn add(x: Expr, y: Expr) -> Expr {
    Expr::Add(Box::new(x), Box::new(y))
}

fn sub(x: Expr, y: Expr) -> Expr {
    Expr::Sub(Box::new(x), Box::new(y))
}

fn mul(x: Expr, y: Expr) -> Expr {
    Expr::Mul(Box::new(x), Box::new(y))
}

fn div(x: Expr, y: Expr) -> Expr {
    Expr::Div(Box::new(x), Box::new(y))
}

fn sigma(from: i64, to: i64, expr: Expr) -> Expr {
    Expr::Sigma(from, to, Box::new(expr))
}

fn eval(expr: &Expr, var: i64) -> Option<i64> {
    // this should return an Option<i64>
    let result = match expr {
        Expr::Const(k) => *k,
        Expr::Var => var,
        Expr::Add(lhs, rhs) => eval(lhs, var)? + eval(rhs, var)?,
        Expr::Sub(lhs, rhs) => eval(lhs, var)? - eval(rhs, var)?,
        Expr::Mul(lhs, rhs) => eval(lhs, var)? * eval(rhs, var)?,
        Expr::Div(lhs, rhs) => {
            let lhs = eval(lhs, var)?;
            let rhs = eval(rhs, var)?;

            if rhs == 0 {
                return None;
            }

            lhs / rhs
        }
        Expr::Summation(exprs) => {
            let mut acc = 0;
            for e in exprs {
                acc += eval(e, var)?;
            }
            acc
        }
        Expr::Sigma(from, to, expr) => {
            let mut sum = 0;

            for x in *from..=*to {
                sum += eval(expr, x)?;
            }

            sum
        }
    };

    Some(result)
}

fn main() {
    let test = |expr| {
        let value = rand::random::<i8>() as i64;
        if let Some(res) = eval(&expr, value) {
            println!("{:?} with Var = {} ==> {}", &expr, value, res);
        } else {
            println!("Error: Division by zero encountered");
        }
    };

    use Expr::*;

    test(Const(5));
    test(Var);
    test(sub(Var, Const(5)));
    test(sub(Var, Var));
    test(add(sub(Var, Const(5)), Const(5)));
    test(Summation(vec![Var, Const(1)]));
    test(mul(Var, Const(5)));
    test(div(Var, Const(5)));
    test(div(Const(60), Var));
    test(sigma(1, 5, Var));
    test(sigma(1, 5, mul(Var, Var)));
    test(sigma(1, 5, mul(Var, mul(Var, Var))));
}

#[cfg(test)]
mod test {
    use super::*;
    use Expr::*;

    #[test]
    fn test_cases() {
        let x = 42;
        assert_eq!(eval(&Const(5), x), Some(5));
        assert_eq!(eval(&Var, x), Some(42));
        assert_eq!(eval(&sub(Var, Const(5)), x), Some(37));
        assert_eq!(eval(&sub(Var, Var), x), Some(0));
        assert_eq!(eval(&add(sub(Var, Const(5)), Const(5)), x), Some(42));
        assert_eq!(eval(&Summation(vec![Var, Const(1)]), x), Some(43));
        assert_eq!(eval(&mul(Var, Const(3)), x), Some(42 * 3));
        assert_eq!(eval(&div(Var, Const(3)), x), Some(42 / 3));
        assert_eq!(eval(&sigma(1, 5, Var), x), Some(15));
        assert_eq!(eval(&sigma(1, 5, mul(Var, Var)), x), Some(55));
        assert_eq!(eval(&sigma(1, 5, mul(Var, mul(Var, Var))), x), Some(225));
    }
}

// If you have time left and want to code more Rust: you can extend this exercise endlessly; one idea would be adding a Sigma(from,to,expr)
// constructor to Expr which computes the equivalent of (in LaTeX notation) \sum_{Var = from}^{to} expr; i.e. Sigma(Const(1), Const(5), Var) should be
// equivalent to Summation(vec![Const(1), Const(2), Const(3), Const(4), Const(5)]).
