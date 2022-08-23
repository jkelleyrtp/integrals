# Advanced math for Rust

This crate provides some advanced math functions taken from cephes.

It's roughly meant to be scipy for Rust.

It's eventually intended to be part of an advanced type-safe math library, but for now, it's just a humble collection of well-tested functions.

Elliptic integrals
---------
- ellipkm1
- ellipk
- ellipe


## How this is being built

Every day I want to convert one or two functions from cephes C to Rust. There's a lot of them, but we can use the cephes library itself as a test suite and template.

Once enough functions have been transposed, then we will "Rustify" the library, adding a type-safe layer on top of our functions and throwing actually helpful errors.

The crate we're trying to hit parity with is special_fun which provides all the cephes functions via FFI to Rust.


## Why

Why not just use cephes?

I love math, physics, and Rust. This project lets me indulge in all three: learn about physics concepts, simulate the harder stuff, and demonstrate my learning in a low-level prorgramming language with a good documentation system.

Plus, there are advantages in not using FFI - WASM support being a top highlight.


## Can I use?

Sure, MIT licesned.

My goal is to provide a somewhat less generic interface than most Rust libraries. I don't care about f8/f16 or weird numbers. Most of my work is simulation and f64 is fine.

Additionally, I don't want to add too much type shenanigans over the underlying algorithms. The core functions should read roughly like C code

## Routines implemented for f64

- 2/78 methods implemented
- About 34 days to completion

- [ ] frexp
- [ ] ldexp
- [ ] lsqrt
- [ ] expm1
- [ ] expx2
- [ ] ei
- [ ] erf
- [ ] erfc
- [ ] spence
- [ ] cosm1
- [ ] sici
- [ ] shichi
- [ ] beta
- [ ] incbet
- [ ] incbi
- [ ] gamma
- [ ] rgamma
- [ ] lgam
- [ ] igam
- [ ] igamc
- [ ] igami
- [ ] psi
- [ ] fac
- [ ] j0
- [ ] j1
- [ ] jn
- [ ] jv
- [ ] y0
- [ ] y1
- [ ] yn
- [ ] yv
- [ ] i0
- [ ] i0e
- [ ] i1
- [ ] i1e
- [ ] iv
- [ ] k0
- [ ] k0e
- [ ] k1
- [ ] k1e
- [ ] kn
- [ ] ellik
- [ ] ellie
- [x] ellpk
- [x] ellpe
- [ ] ellpj
- [ ] hyperg
- [ ] onef2
- [ ] hyp2f1
- [ ] threef0
- [ ] bdtr
- [ ] bdtrc
- [ ] bdtri
- [ ] nbdtr
- [ ] nbdtrc
- [ ] nbdtri
- [ ] btdtr
- [ ] chdtr
- [ ] chdtrc
- [ ] chdtri
- [ ] fdtr
- [ ] fdtrc
- [ ] fdtri
- [ ] gdtr
- [ ] gdtrc
- [ ] ndtr
- [ ] ndtri
- [ ] pdtr
- [ ] pdtrc
- [ ] pdtri
- [ ] stdtr
- [ ] stdtri
- [ ] airy
- [ ] dawsn
- [ ] fresnl
- [ ] plancki
- [ ] struve
- [ ] zetac
- [ ] zeta

Built-in
- [ ] exp
- [ ] exp10
- [ ] exp2
- [ ] signbit
- [ ] isnan
- [ ] isfinite
- [ ] round
- [ ] floor
- [ ] ceil
- [ ] sqrt
- [ ] pow
- [ ] powi
- [ ] log
- [ ] log10
- [ ] log2
- [ ] log1p
- [ ] sinh
- [ ] cosh
- [ ] tanh
- [ ] sin
- [ ] cos
- [ ] tan
- [ ] asin
- [ ] acos
- [ ] atan
- [ ] asinh
- [ ] acosh
- [ ] atanh
- [ ] fabs
- [ ] atan2
- [ ] cbrt



```rust
pub fn round(x: f64) -> f64; /// Round to nearest or event integer valued f64.
pub fn floor(x: f64) -> f64; /// Largest integer less than or equal to x.
pub fn ceil(x: f64) -> f64; /// Smallest integer greater than or equal to x.
pub fn frexp(x: f64, expnt: &mut i32) -> f64; /// x = y * 2**expn
pub fn ldexp(x: f64, n: i32) -> f64; /// Multiply x by 2**n.
pub fn fabs(x: f64) -> f64; /// Absolute value.
pub fn signbit(x: f64) -> i32; /// Return 1 if the sign bit of x is 1, else 0.
pub fn isnan(x: f64) -> i32; /// Return 1 if x is NaN, else 0.
pub fn isfinite(x: f64) -> i32; /// Return 1 if x is finite, else 0.
pub fn cbrt(x: f64) -> f64; /// Cube root.
pub fn sqrt(x: f64) -> f64; /// Square root.
pub fn lsqrt(x: i64) -> i64; /// Integer square root.
pub fn exp(x: f64) -> f64; /// Exponential function.
pub fn exp10(x: f64) -> f64; /// Base 10 exponential function.
pub fn exp2(x: f64) -> f64; /// Base 2 exponential function.
pub fn expm1(x: f64) -> f64; /// Compute accurately exponential of squared argument.
pub fn expx2(x: f64, sign: i32) -> f64; /// Compute accurately exp(x) - 1 for x close to 0.
pub fn ei(x: f64) -> f64; /// Exponential integral.
pub fn erf(x: f64) -> f64; /// Error function.
pub fn erfc(x: f64) -> f64; /// Complementary error function.
pub fn pow(x: f64, y: f64) -> f64; /// Power function.
pub fn powi(x: f64, n: i32) -> f64; /// Integer power function.
pub fn log(x: f64) -> f64; /// Natural logarithm.
pub fn log10(x: f64) -> f64; /// Common logarithm.
pub fn log2(x: f64) -> f64; /// Base 2 logarithm.
pub fn log1p(x: f64) -> f64; /// Compute accurately log(1 + x) for x close to 0.
pub fn spence(x: f64) -> f64; /// Dilogarithm (Spence's function).
pub fn sin(x: f64) -> f64; /// Circular sine.
pub fn cos(x: f64) -> f64; /// Circular cosine.
pub fn tan(x: f64) -> f64; /// Circular tangent.
pub fn asin(x: f64) -> f64; /// Inverse circular sine.
pub fn acos(x: f64) -> f64; /// Inverse circular cosine.
pub fn atan(x: f64) -> f64; /// Inverse circular tangent.
pub fn atan2(y: f64, x: f64) -> f64; /// Quadrant-correct inverse circular tangent.
pub fn cosm1(x: f64) -> f64; /// Compute accurately cos(x) - 1 for x close to 0.
pub fn sici(x: f64, si: &mut f64, ci: &mut f64) -> f64; /// Sine and cosine integrals.
pub fn sinh(x: f64) -> f64; /// Hyperbolic sine.
pub fn cosh(x: f64) -> f64; /// Hyperbolic cosine.
pub fn tanh(x: f64) -> f64; /// Hyperbolic tangent.
pub fn asinh(x: f64) -> f64; /// Inverse hyperbolic sine.
pub fn acosh(x: f64) -> f64; /// Inverse hyperbolic cosine.
pub fn atanh(x: f64) -> f64; /// Inverse hyperbolic tangent.
pub fn shichi(x: f64, chi: &mut f64, shi: &mut f64); /// Hyperbolic sine and cosine integrals.
pub fn beta(a: f64, b: f64) -> f64; /// Beta function.
pub fn incbet(a: f64, b: f64, x: f64) -> f64; /// Regularized incomplete beta function.
pub fn incbi(a: f64, b: f64, y: f64) -> f64; /// Inverse of incomplete beta integral.
pub fn gamma(x: f64) -> f64; /// Gamma function.
pub fn rgamma(x: f64) -> f64; /// Reciprocal gamma function.
pub fn lgam(x: f64) -> f64; /// Natural logarithm of gamma function.
pub fn igam(a: f64, x: f64) -> f64; /// Regularized incomplete gamma integral.
pub fn igamc(a: f64, x: f64) -> f64; /// Complemented incomplete gamma integral.
pub fn igami(a: f64, p: f64) -> f64; /// Inverse of complemented incomplete gamma integral.
pub fn psi(x: f64) -> f64; /// Psi (digamma) function.
pub fn fac(i: i32) -> f64; /// Factorial function.
pub fn j0(x: f64) -> f64; /// Bessel function of order zero.
pub fn j1(x: f64) -> f64; /// Bessel function of order one.
pub fn jn(n: i32, x: f64) -> f64; /// Bessel function of integer order.
pub fn jv(n: f64, x: f64) -> f64; /// Bessel function of real order.
pub fn y0(x: f64) -> f64; /// Bessel function of the second kind, order zero.
pub fn y1(x: f64) -> f64; /// Bessel function of the second kind, order one.
pub fn yn(n: i32, x: f64) -> f64; /// Bessel function of the second kind, integer order.
pub fn yv(v: f64, x: f64) -> f64; /// Bessel function of the second kind, real order.
pub fn i0(x: f64) -> f64; /// Modified Bessel function of order zero.
pub fn i0e(x: f64) -> f64; /// Modified Bessel function of order zero, exponentially scaled.
pub fn i1(x: f64) -> f64; /// Modified Bessel function of order one.
pub fn i1e(x: f64) -> f64; /// Modified Bessel function of order one, exponentially scaled.
pub fn iv(v: f64, x: f64) -> f64; /// Modified Bessel function of real order.
pub fn k0(x: f64) -> f64; /// Modified Bessel function of the third kind, order zero.
pub fn k0e(x: f64) -> f64; /// exponentially scaled.
pub fn k1(x: f64) -> f64; /// Modified Bessel function of the third kind, order one.
pub fn k1e(x: f64) -> f64; /// exponentially scaled.
pub fn kn(n: i32, x: f64) -> f64; /// Modified Bessel function of the third kind, integer order.
pub fn ellik(phi: f64, m: f64) -> f64; /// Incomplete elliptic integral of the first kind.
pub fn ellie(phi: f64, m: f64) -> f64; /// Incomplete elliptic integral of the second kind.
pub fn ellpk(m1: f64) -> f64; /// Complete elliptic integral of the first kind.
pub fn ellpe(m1: f64) -> f64; /// Complete elliptic integral of the second kind.
pub fn ellpj(u: f64, m: f64, sn: &mut f64, cn: &mut f64, dn: &mut f64, phi: &mut f64) -> i32; /// Jacobian elliptic function.
pub fn hyperg(a: f64, b: f64, x: f64) -> f64; /// Confluent hypergeometric function 1F1.
pub fn onef2(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64; /// Hypergeometric function 1F2.
pub fn hyp2f1(a: f64, b: f64, c: f64, x: f64) -> f64; /// Gauss hypergeometric function 2F1.
pub fn threef0(a: f64, b: f64, c: f64, x: f64, err: &mut f64) -> f64; /// Hypergeometric function 3F0.
pub fn bdtr(k: i32, n: i32, p: f64) -> f64; /// Binomial distribution.
pub fn bdtrc(k: i32, n: i32, p: f64) -> f64; /// Complemented binomial distribution.
pub fn bdtri(k: i32, n: i32, y: f64) -> f64; /// Inverse of binomial distribution.
pub fn nbdtr(k: i32, n: i32, p: f64) -> f64; /// Negative binomial distribution.
pub fn nbdtrc(k: i32, n: i32, p: f64) -> f64; /// Complemented negative binomial distribution.
pub fn nbdtri(k: i32, n: i32, p: f64) -> f64; /// Inverse of negative binomial distribution.
pub fn btdtr(a: f64, b: f64, x: f64) -> f64; /// Beta distribution.
pub fn chdtr(df: f64, x: f64) -> f64; /// Chi-square distribution.
pub fn chdtrc(v: f64, x: f64) -> f64; /// Complemented chi-square distribution.
pub fn chdtri(df: f64, y: f64) -> f64; /// Inverse of complemented chi-square distribution.
pub fn fdtr(df1: i32, df2: i32, x: f64) -> f64; /// F distribution.
pub fn fdtrc(df1: i32, df2: i32, x: f64) -> f64; /// Complemented F distribution.
pub fn fdtri(df1: i32, df2: i32, p: f64) -> f64; /// Inverse of complemented F distribution.
pub fn gdtr(a: f64, b: f64, x: f64) -> f64; /// Gamma distribution.
pub fn gdtrc(a: f64, b: f64, x: f64) -> f64; /// Complemented gamma distribution.
pub fn ndtr(x: f64) -> f64; /// Normal distribution.
pub fn ndtri(y: f64) -> f64; /// Inverse of normal distribution.
pub fn pdtr(k: i32, m: f64) -> f64; /// Poisson distribution.
pub fn pdtrc(k: i32, m: f64) -> f64; /// Complemented Poisson distribution.
pub fn pdtri(k: i32, y: f64) -> f64; /// Inverse of Poisson distribution.
pub fn stdtr(k: i16, t: f64) -> f64; /// Student's t distribution.
pub fn stdtri(k: i32, p: f64) -> f64; /// Inverse of Student's t distribution.
pub fn airy(x: f64, ai: &mut f64, aip: &mut f64, bi: &mut f64, bip: &mut f64) -> i32; /// Airy function.
pub fn dawsn(x: f64) -> f64; /// Dawson's integral.
pub fn fresnl(x: f64, s: &mut f64, c: &mut f64); /// Fresnel integral.
pub fn plancki(lambda: f64, temperature: f64) -> f64; /// Integral of Planck's black body radiation formula.
pub fn struve(v: f64, x: f64) -> f64; /// Struve function.
pub fn zetac(x: f64) -> f64; /// Riemann zeta function.
pub fn zeta(x: f64, q: f64) -> f64; /// Riemann zeta function of two arguments.
```
