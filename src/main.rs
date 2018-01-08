
use std::rc::Rc;
use std::fmt;

// Symbolic functions in 1 variable (R -> R)
#[derive(Clone, Debug)]
pub enum FN {
    Var,
    Const(f32),
    Plus(Rc<FN>, Rc<FN>),
    Mult(Rc<FN>, Rc<FN>)
}

impl fmt::Display for FN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Var => write!(f, "x"),
            &Const(i) => write!(f, "{}", i),
            &Plus(ref p, ref q) => write!(f, "({}+{})", p, q),
            &Mult(ref p, ref q) => write!(f, "{}*{}", p, q)
        }
    }
}

// Helper functions to generate boxed FNs
fn var() -> Rc<FN> {
    Rc::new(Var)
}

fn cnst(i : f32) -> Rc<FN> {
    Rc::new(Const(i))
}

fn plus(x : Rc<FN>, y: Rc<FN>) -> Rc<FN> {
    Rc::new(Plus(x, y))
}

fn mult(x : Rc<FN>, y: Rc<FN>) -> Rc<FN> {
    Rc::new(Mult(x, y))
}

// Generator a polynomial from coefficients (most significant first)
fn polynomial(v : &Vec<f32>) -> Rc<FN> {
    if v.len() > 0 {
        let x0 = v[0];
        let vs = &v[1..];
        vs.iter().fold(cnst(x0), |x,y|{plus(mult(x, var()), cnst(*y))} )
    } else
    {
        cnst(0.0)
    }
}

use FN::*;

struct AD {
    v : f32, // value
    d : f32 // derivative
}

// Evaluation and forward mode differentiation
fn ad (f : &FN, x : f32) -> AD {
    match f {
        &Var => AD{ v: x, d: 1.0},
        &Const(k) => AD{v: k, d: 0.0},
        &Plus(ref v1, ref v2) => {
            let AD{v : v1v, d: v1p} = ad(&v1, x);
            let AD{v: v2v, d: v2p} = ad(&v2, x);
            AD{ v: v1v + v2v, d: v1p + v2p}
        }

        &Mult(ref v1, ref v2) => {
            let AD{v: v1v, d:v1p} = ad(&v1, x);
            let AD{v: v2v, d: v2p} = ad(&v2, x);
            AD{ v: v1v * v2v, d: v1p * v2v + v2p * v1v}
        }
    }
}


// Newton-Raphson root finding
fn nr_root(epsilon : f32, x0 : f32, f : &FN) -> f32 {
    assert!(epsilon > 0.0);
    let mut x = x0;
    let mut c = ad(f, x);
    let mut steps = 0;
    while c.v.abs() > epsilon {
        c = ad(f, x);
        x -= c.v / c.d;
        steps += 1;
        // TODO: Conditionally compile or remove
        println!("step {}; x={}; f(x)={}; f'(x)={}  ", steps,  x, c.v, c.d);

        // TODO: Implement error handling
        if steps > 1000 { panic!("Step limit reached")};
    }
    x
}

fn main() {
    let f = polynomial(&vec![2.0,5.0,3.0,-7.0]);
    let x = nr_root(1e-6, -100.0, &f);
    println!("Found root for {} at x={}", f, x);
}
