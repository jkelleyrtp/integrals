#![allow(clippy::excessive_precision)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::approx_constant)]
use crate::{constants::MACHEP, polyeval};

/// Complete elliptic integral of the first kind
///
/// SYNOPSIS:
///
/// double m1, y, ellpk();
///
/// y = ellpk( m1 );
///
///
///
/// DESCRIPTION:
///
/// Approximates the integral
///
///            pi/2
///             -
///            | |
///            |           dt
/// K(m)  =    |    ------------------
///            |                   2
///          | |    sqrt( 1 - m sin t )
///           -
///            0
///
/// where m = 1 - m1, using the approximation
///
///     P(x)  -  log x Q(x).
///
/// The argument m1 is used rather than m so that the logarithmic
/// singularity at m = 1 will be shifted to the origin; this
/// preserves maximum accuracy.
///
/// K(0) = pi/2.
///
/// ACCURACY:
///
///                      Relative error:
/// arithmetic   domain     # trials      peak         rms
///    DEC        0,1        16000       3.5e-17     1.1e-17
///    IEEE       0,1        30000       2.5e-16     6.8e-17
///
/// ERROR MESSAGES:
///
///   message         condition      value returned
/// ellpk domain       x<0, x>1           0.0
pub fn ellpkm1(x: f64) -> f64 {
    static P: &[f64] = &[
        1.37982864606273237150E-4,
        2.28025724005875567385E-3,
        7.97404013220415179367E-3,
        9.85821379021226008714E-3,
        6.87489687449949877925E-3,
        6.18901033637687613229E-3,
        8.79078273952743772254E-3,
        1.49380448916805252718E-2,
        3.08851465246711995998E-2,
        9.65735902811690126535E-2,
        1.38629436111989062502E0,
    ];

    static Q: &[f64] = &[
        2.94078955048598507511E-5,
        9.14184723865917226571E-4,
        5.94058303753167793257E-3,
        1.54850516649762399335E-2,
        2.39089602715924892727E-2,
        3.01204715227604046988E-2,
        3.73774314173823228969E-2,
        4.88280347570998239232E-2,
        7.03124996963957469739E-2,
        1.24999999999870820058E-1,
        4.99999999999999999821E-1,
    ];

    const C1: f64 = 1.3862943611198906188E0;

    if (x < 0.0) || (x > 1.0) {
        0.0
    } else if x > MACHEP {
        polyeval(x, P) - x.ln() * polyeval(x, Q)
    } else if x == 0.0 {
        f64::MAX
    } else {
        C1 - 0.5 * x.ln()
    }
}

/// Complete elliptic integral of the first kind
pub fn ellpk(x: f64) -> f64 {
    ellpkm1(1.0 - x)
}

#[test]
fn ellpk_works() {
    use approx::assert_relative_eq;

    // Pulled from cephes
    assert_relative_eq!(ellpkm1(0.0), 1.7976931348623157e308);
    assert_relative_eq!(ellpkm1(0.1), 2.5780921133481733);
    assert_relative_eq!(ellpkm1(0.2), 2.2572053268208534);
    assert_relative_eq!(ellpkm1(0.3), 2.075363135292469);
    assert_relative_eq!(ellpkm1(0.4), 1.9495677498060258);
    assert_relative_eq!(ellpkm1(0.5), 1.8540746773013719);
    assert_relative_eq!(ellpkm1(0.6), 1.7775193714912534);
    assert_relative_eq!(ellpkm1(0.7), 1.713889448178791);
    assert_relative_eq!(ellpkm1(0.8), 1.659623598610528);
    assert_relative_eq!(ellpkm1(0.9), 1.6124413487202192);
    assert_relative_eq!(ellpkm1(1.0), std::f64::consts::PI / 2.0);

    // Pulled from cephes/scipy
    assert_relative_eq!(ellpk(0.0), 1.5707963267948966);
    assert_relative_eq!(ellpk(0.1), 1.6124413487202192);
    assert_relative_eq!(ellpk(0.2), 1.659623598610528);
    assert_relative_eq!(ellpk(0.3), 1.713889448178791);
    assert_relative_eq!(ellpk(0.4), 1.7775193714912534);
    assert_relative_eq!(ellpk(0.5), 1.8540746773013719);
    assert_relative_eq!(ellpk(0.6), 1.9495677498060258);
    assert_relative_eq!(ellpk(0.7), 2.075363135292469);
    assert_relative_eq!(ellpk(0.8), 2.257205326820854);
    assert_relative_eq!(ellpk(0.9), 2.5780921133481733);
}
