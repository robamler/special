pub use self::specialized::*;

macro_rules! declare_float_ext {
    ($(($method_name:ident, $fn32:expr, $fn64:expr $(, $param:ident: $t:tt)*),)*) => {
        /// TODO: document
        pub trait FloatExt {
            $(
                fn $method_name(self, $($param: $t),*) -> Self;
            )*
        }
    }
}

macro_rules! bind_float_ext_methods {
    ($macro: ident) => {
        $macro! {
            (ln, libm::logf, libm::log),
            (exp, libm::expf, libm::exp),
            (sqrt, libm::sqrtf, libm::sqrt),
            (powf, libm::powf, libm::pow, exponent: Self),
            (powi, powi_f32, powi_f64, exponent: i32),
            (abs, libm::fabsf, libm::fabs),
            (sin, libm::sinf, libm::sin),
            (floor, libm::floorf, libm::floor),
            (ln_1p, libm::log1pf, libm::log1p),
            (exp_m1, libm::expm1f, libm::expm1),
            (round, libm::roundf, libm::round),
            (trunc, libm::truncf, libm::trunc),
            (fract, fract_f32, fract_f64),
        }
    }
}

bind_float_ext_methods!{ declare_float_ext }

#[cfg(feature = "std")]
mod specialized {
    use super::FloatExt;

    macro_rules! impl_float_ext {
        ($(($method_name:ident, $fn32:expr, $fn64:expr $(, $param:ident: $t:tt)*),)*) => {
            impl FloatExt for f32 {
                $(
                    #[inline(always)]
                    fn $method_name(self, $($param: $t),*) -> Self {
                        f32::$method_name(self, $($param),*)
                    }
                )*
            }
    
            impl FloatExt for f64 {
                $(
                    #[inline(always)]
                    fn $method_name(self, $($param: $t),*) -> Self {
                        f64::$method_name(self, $($param),*)
                    }
                )*
            }
        };
    }

    bind_float_ext_methods!{ impl_float_ext }
}

#[cfg(not(feature = "std"))]
mod specialized {
    use super::FloatExt;

    macro_rules! impl_float_ext {
        ($(($method_name:ident, $fn32:expr, $fn64:expr $(, $param:ident: $t:tt)*),)*) => {
            impl FloatExt for f32 {
                $(
                    #[inline(always)]
                    fn $method_name(self, $($param: $t),*) -> Self {
                        $fn32(self, $($param),*)
                    }
                )*
            }
    
            impl FloatExt for f64 {
                $(
                    #[inline(always)]
                    fn $method_name(self, $($param: $t),*) -> Self {
                        $fn64(self, $($param),*)
                    }
                )*
            }
        };
    }
    
    #[inline(always)]
    fn powi_f32(x: f32, i: i32) -> f32 {
        libm::powf(x, i as f32)
    }
    
    #[inline(always)]
    fn powi_f64(x: f64, i: i32) -> f64 {
        libm::pow(x, i as f64)
    }

    #[inline(always)]
    fn fract_f32(x: f32) -> f32 {
        x - x.trunc() // TODO: verify that this is correct for negative numbers.
    }

    #[inline(always)]
    fn fract_f64(x: f64) -> f64 {
        x - x.trunc() // TODO: verify that this is correct for negative numbers.
    }

    bind_float_ext_methods!{ impl_float_ext }
}
