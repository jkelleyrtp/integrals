/// Calculate the value of a polynomial at a given point.
///
///                     2          N
/// y  =  C  + C x + C x  +...+ C x
///        0    1     2          N
///
/// Coefficients are stored in reverse order:
///
/// coef[0] = C  , ..., coef[N] = C  .
///            N                   0
///
/// The function p1evl() assumes that coef[N] = 1.0 and is
/// omitted from the array.  Its calling arguments are
/// otherwise the same as polevl().
pub fn polyeval(x: f64, coeffs: &[f64]) -> f64 {
    /*
    This uses horner's method to evaluate the polynomial with as few instructions as possible.

    https://en.wikipedia.org/wiki/Horner%27s_method

    The first coefficient is assumed to be 1.0.
    */

    match coeffs.split_first() {
        Some((&k, coefficients)) => {
            let mut val = k;
            for &k in coefficients {
                val = k + (val * x);
            }
            val
        }
        None => 0.0,
    }
}

#[test]
fn test_poleval() {
    // let coeffs = [1.0, 2.0, 3.0, 4.0];
    // assert_eq!(poleval(&coeffs, 2.0), 14.0);
}
