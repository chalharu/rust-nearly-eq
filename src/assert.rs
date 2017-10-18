//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

/// Asserts that two expressions are nearly(approximately) equal to each other.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, NearlyEq::eps() is the default used.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate nearly_eq;
/// # fn main() {
/// assert_nearly_eq!(1f64, 1.5f64, 0.6f64); // does not panic
/// assert_nearly_eq!(0f64, 1e-12f64); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate nearly_eq;
/// # fn main() {
/// assert_nearly_eq!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
macro_rules! assert_nearly_eq {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        #[inline(always)]
        fn nearly_eq_noeps<A: ?Sized, B, C: $crate::NearlyEq<A, B> + ?Sized>(a: &C, b: &A) -> bool {
            a.eq(b, &C::eps())
        }
        assert!(nearly_eq_noeps(a, b),
                "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`)",
                 *a, *b);
    });
    ($a:expr, $b:expr, $eps:expr) => ({
        let (a, b, eps) = (&$a, &$b, &$eps);
        #[inline(always)]
        fn nearly_eq<A: ?Sized, B, C: $crate::NearlyEq<A, B> + ?Sized>(a: &C, b: &A, c: &B) -> bool {
            a.eq(b, c)
        }
        assert!(nearly_eq(a, b, eps),
                "assertion failed: `(left == right)` (left: `{:?}` , right: `{:?}`, eps: `{:?}`)",
                 *a, *b, $eps);
    })
}

/// Asserts that two expressions are nearly(approximately) equal to each other.
///
/// You can optionally add an optional diff value. If you don't supply
///  a diff value as an argument, NearlyEq::eps() is the default used.
///
/// Unlike assert_nearly_eq!, debug_assert_nearly_eq! statements are only enabled in non optimized builds by default.
/// An optimized build will omit all debug_assert_nearly_eq! statements unless -C debug-assertions is passed to the compiler.
///
/// # Examples
///
/// ```rust
/// # #[macro_use] extern crate nearly_eq;
/// # fn main() {
/// debug_assert_nearly_eq!(1f64, 1.5f64, 0.6f64); // does not panic
/// debug_assert_nearly_eq!(0f64, 1e-12f64); // does not panic
/// # }
/// ```
/// ```should_panic
/// # #[macro_use] extern crate nearly_eq;
/// # fn main() {
/// debug_assert_nearly_eq!(1f64, 2f64); // panics
/// # }
/// ```
#[macro_export]
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.3"))]
macro_rules! debug_assert_nearly_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_nearly_eq!($($arg)*); })
}
