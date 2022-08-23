use crate::polyeval;

/// Complete elliptic integral of the second kind
///
/// SYNOPSIS:
///
/// double m1, y, ellpe();
///
/// y = ellpe( m1 );
///
///
///
/// DESCRIPTION:
///
/// Approximates the integral
///
///
///            pi/2
///             -
///            | |                 2
/// E(m)  =    |    sqrt( 1 - m sin t ) dt
///          | |
///           -
///            0
///
/// Where m = 1 - m1, using the approximation
///
///      P(x)  -  x log x Q(x).
///
/// Though there are no singularities, the argument m1 is used
/// rather than m for compatibility with ellpk().
///
/// E(1) = 1; E(0) = pi/2.
///
///
/// ACCURACY:
///
///                      Relative error:
/// arithmetic   domain     # trials      peak         rms
///    DEC        0, 1       13000       3.1e-17     9.4e-18
///    IEEE       0, 1       10000       2.1e-16     7.3e-17
///
///
/// ERROR MESSAGES:
///
///   message         condition      value returned
/// ellpe domain      x<0, x>1            0.0
pub fn ellipe(x: f64) -> f64 {
    let x = 1.0 - x;

    static P: &[f64] = &[
        1.53552577301013293365E-4,
        2.50888492163602060990E-3,
        8.68786816565889628429E-3,
        1.07350949056076193403E-2,
        7.77395492516787092951E-3,
        7.58395289413514708519E-3,
        1.15688436810574127319E-2,
        2.18317996015557253103E-2,
        5.68051945617860553470E-2,
        4.43147180560990850618E-1,
        1.00000000000000000299E0,
    ];
    static Q: &[f64] = &[
        3.27954898576485872656E-5,
        1.00962792679356715133E-3,
        6.50609489976927491433E-3,
        1.68862163993311317300E-2,
        2.61769742454493659583E-2,
        3.34833904888224918614E-2,
        4.27180926518931511717E-2,
        5.85936634471101055642E-2,
        9.37499997197644278445E-2,
        2.49999999999888314361E-1,
    ];

    if (x < 0.0) || (x > 1.0) {
        if x == 1.0 {
            1.0
        } else {
            0.0
        }
    } else {
        polyeval(x, P) - x.ln() * x * polyeval(x, Q)
    }
}

#[test]
fn ellpk_works() {
    use approx::assert_relative_eq;

    assert_relative_eq!(ellipe(0.0), 1.5707963267948966);
    assert_relative_eq!(ellipe(0.1), 1.5307576368977633);
    assert_relative_eq!(ellipe(0.2), 1.489035058095853);
    assert_relative_eq!(ellipe(0.3), 1.4453630644126654);
    assert_relative_eq!(ellipe(0.4), 1.3993921388974322);
    assert_relative_eq!(ellipe(0.5), 1.3506438810476755);
    assert_relative_eq!(ellipe(0.6), 1.2984280350469133);
    assert_relative_eq!(ellipe(0.7), 1.2416705679458229);
    assert_relative_eq!(ellipe(0.8), 1.1784899243278386);
    assert_relative_eq!(ellipe(0.9), 1.1047747327040733);
}
