use libc::{c_double, c_int};

extern "C" {
    pub fn erf(x: c_double) -> c_double;
    pub fn erfc(x: c_double) -> c_double;
    pub fn tgamma(x: c_double) -> c_double;
}

#[cfg(windows)]
extern "C" {
    pub fn lgamma(x: c_double, sign: &mut c_int) -> c_double;
}

#[cfg(unix)]
extern "C" {
    pub fn lgamma_r(x: c_double, sign: &mut c_int) -> c_double;
}

#[cfg(unix)]
#[inline(always)]
pub unsafe fn lgamma(x: c_double, sign: &mut c_int) -> c_double {
    lgamma_r(x, sign)
}
