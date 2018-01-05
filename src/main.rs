// // Symbolic functions in 1 variable (R -> R)
pub enum FN<'a> {
    Var,
    Const(f32),
    Plus(&'a FN<'a>, &'a FN<'a>),
    Mult(&'a FN<'a>, &'a FN<'a>)
}


use FN::*;

fn sq<'a> (x : &'a FN) -> FN<'a> {
    Mult(&x,&x)
}

// fn foldl1<'a, a> (f : &Fn(&a,&a) -> a, v : &Vec<&'a a>) -> &'a a {
//     let mut acc = v[0];
//     for w in v[1..].iter() {
//         acc = &f(&acc, w);
//     }
//     acc
// }

// fn sum<'a>(v: &'a Vec<&'a FN>) -> FN<'a> {
//     v.iter().fold(Const(0.0), |x : &FN,y : &FN| {Plus(&x,&y)})
// }

// fn poly<'a>(coeffs : Vec<f32>) -> FN<'a> {
//     if (coeffs.len() == 0) {
//         Const(0.0)
//     }
//     else {
//         let mut acc=Const(coeffs[0]);
//         for c in coeffs[1..].iter() {
//             let cnst : &'a FN = &Const(*c);
//             let m = Mult(&acc, &cnst);
//             acc= m;
//         }
//         acc
//     }
// }


struct AD {
    v : f32, // value
    d : f32 // derivative
}

// Forward mode derivation
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


// Newton-Raphson
fn nr_root(epsilon : f32, x0 : f32, f : &FN) -> f32 {
    let mut x = x0;
    let mut c = ad(f, x);
    let mut steps = 0;
    while (c.v.abs() > epsilon){
        c = ad(f, x);
        x -= c.v / c.d;
        steps += 1;
        println!("step {}; x={}; f(x)={}; f'(x)={}  ", steps,  x, c.v, c.d);
        if steps > 1000 { panic!("Step limit reached")};
    }
    x
}

fn main() {
    let v = &Var;
    let x2 = sq(v);
    let f = Mult( &x2, &Const(3.0));
    let x = nr_root(0.0000001, 100.0, &f);
    println!("root at: {}", x);

}
