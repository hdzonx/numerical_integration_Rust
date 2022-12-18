use ndarray::prelude::*;
/*
The Bézier curve represent a important type of curve used in
computer graphics. It is a curve that is defined by a set of points.
The nth-order Bézier curve is given by the following formula:
x = ∑Bi,n(ξ) Pxi
y = ∑Bi,n(ξ) Pyi
-1 ≤ ξ ≤ 1
Bi,n(ξ) is the Bernstein polynomial of order n.
Pxi and Pyi are the control points.
*/

//function that return a factorial of a value
pub fn factorial(value: f64) -> f64 {
    let fact: f64;
    if value == 0.0 {
        fact = 1.0;
    } else {
        fact = value * factorial(value - 1.0);
    }
    fact
}

/*Bernstein polynomial of order n and i in the range -1 ≤ ξ ≤ 1.
In the function, n is the order of the Bernstein polynomial, i is the
index. t == ξ and -1 ≤ ξ ≤ 1."""
*/
pub fn Bernstein(n: f64, i: f64, t: f64) -> f64 {
    let berns: f64;
    berns = (factorial(n) / (factorial(i) * factorial(n - i)))
        * (((1.0 + t) / 2.0).powf(i))
        * ((1.0 - t) / 2.0).powf(n - i);
    berns
}

/*
get the Bernstein function of order n in
the range -1 ≤ ξ ≤ 1, for index i
 */
pub fn get_Bernstein_function(n: f64, i: f64) -> Vec<f64> {
    let steps = 1.0 / 50.0;
    let array_of_coords = Array::range(0., 10., steps);

    let mut vec_of_bers_val = Vec::<f64>::new();

    for t in array_of_coords {
        let y = Bernstein(n, i, t);
        vec_of_bers_val.push(y);
    }
    vec_of_bers_val
}
