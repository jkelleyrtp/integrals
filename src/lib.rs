#![allow(clippy::excessive_precision)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::approx_constant)]

pub(crate) mod constants;

// mod signals;
// mod fft {
//     mod fast_fourier_transform;
// }

pub mod elliptic {
    /// Jacobian elliptic functions
    mod jacobian;
    pub use jacobian::ellipj;

    /// Complete elliptic integral of the first kind.
    mod first_kind_complete;
    pub use first_kind_complete::*;

    /// Complete elliptic integral of the first kind around m = 1
    mod second_kind_complete;
    pub use second_kind_complete::ellipe;

    // Incomplete elliptic integral of the first kind
    mod first_kind_incomplete;
    // pub use first_kind_incomplete::ellipkinc;

    // Complete elliptic integral of the second kind
    mod second_kind_incomplete;
    // pub use second_kind_complete::ellipe;

    // ellipeinc(phi, m[, out])
    // Incomplete elliptic integral of the second kind

    // elliprc(x, y[, out])
    // Degenerate symmetric elliptic integral.

    // elliprd(x, y, z[, out])
    // Symmetric elliptic integral of the second kind.

    // elliprf(x, y, z[, out])
    // Completely-symmetric elliptic integral of the first kind.

    // elliprg(x, y, z[, out])
    // Completely-symmetric elliptic integral of the second kind.

    // elliprj(x, y, z, p[, out])
    // Symmetric elliptic integral of the third kind.
}

pub(crate) mod polyeval;

pub use polyeval::polyeval;
