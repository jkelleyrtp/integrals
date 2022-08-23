#![allow(clippy::excessive_precision)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::approx_constant)]

pub(crate) mod constants;
pub(crate) mod ellipe;
pub(crate) mod ellipk;

pub(crate) mod polyeval;

pub use ellipe::ellipe;
pub use ellipk::{ellpk, ellpkm1};
pub use polyeval::polyeval;
