//! Trait for nearly equality comparisons.
//!
//! # Overview
//! Implementing the `NearlyEq` traits, Can asserts that the two expressions are nearly(approximately) equal to each other.
//!
//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.
//!
//! # Examples
//!
//! ```rust
//! # #[macro_use] extern crate nearly_eq;
//! # fn main() {
//! assert_nearly_eq!(1f64, 1.5f64, 0.6f64); // does not panic
//! assert_nearly_eq!(0f64, 1e-12f64); // does not panic
//! # }
//! ```
//! ```should_panic
//! # #[macro_use] extern crate nearly_eq;
//! # fn main() {
//! assert_nearly_eq!(1f64, 2f64); // panics
//! # }
//! ```

#![cfg_attr(feature = "docs", feature(staged_api))]
#![cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
#![cfg_attr(feature = "i128", feature(i128_type))]

#[cfg(feature = "num-complex")]
extern crate num_complex;

#[cfg(feature = "num-rational")]
extern crate num_rational;

#[cfg(feature = "num-integer")]
extern crate num_integer;

#[cfg(feature = "num-traits")]
extern crate num_traits;

#[cfg(feature = "ndarray")]
extern crate ndarray;

#[macro_use]
mod assert;

#[cfg(feature = "num-complex")]
mod complex_impl;

#[cfg(feature = "num-traits")]
#[cfg(feature = "num-rational")]
#[cfg(feature = "num-integer")]
mod rational_impl;

#[cfg(feature = "ndarray")]
mod ndarray_impl;

use std::rc::{Rc, Weak};

use std::sync::Arc;

/// Trait for nearly equality comparisons.
#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
pub trait NearlyEq<Rhs: ?Sized = Self, Diff: ?Sized = Self> {
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn eps() -> Diff;

    /// This method tests for self and other values to be nearly equal.
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn eq(&self, other: &Rhs, eps: &Diff) -> bool;

    /// This method tests for not nearly equal.
    #[inline]
    #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
    fn ne(&self, other: &Rhs, eps: &Diff) -> bool {
        !self.eq(other, eps)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl NearlyEq for f32 {
    fn eps() -> f32 {
        1e-6
    }

    fn eq(&self, other: &f32, eps: &f32) -> bool {
        let diff = (*self - *other).abs();

        if *self == *other {
            true
        } else {
            diff < *eps
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl NearlyEq for f64 {
    fn eps() -> f64 {
        1e-11
    }

    fn eq(&self, other: &f64, eps: &f64) -> bool {
        let diff = (*self - *other).abs();

        if *self == *other {
            true
        } else {
            diff < *eps
        }
    }
}

macro_rules! itype_impls {
    ($($T:ty)+) => {
        $(
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.0"))]
            impl NearlyEq for $T {
                fn eps() -> $T {
                    0
                }

                fn eq(&self, other: &$T, eps: &$T) -> bool {
                    let diff = (*self - *other).abs();

                    if *self == *other {
                        true
                    } else {
                        diff < *eps
                    }
                }
            }
        )+
    }
}

itype_impls! { i8 i16 i32 i64 }

#[cfg(feature = "i128")]
itype_impls! { i128 }

macro_rules! utype_impls {
    ($($T:ty)+) => {
        $(
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.0"))]
            impl NearlyEq for $T {
                fn eps() -> $T {
                    0
                }

                fn eq(&self, other: &$T, eps: &$T) -> bool {
                    let diff = if *self > *other { *self - *other } else { *other - *self };

                    if *self == *other {
                        true
                    } else {
                        diff < *eps
                    }
                }
            }
        )+
    }
}

utype_impls! { u8 u16 u32 u64 }

#[cfg(feature = "i128")]
utype_impls! { u128 }

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<[A], B> for [C] {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &[A], eps: &B) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self[i].ne(&other[i], eps) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<Vec<A>, B> for Vec<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Vec<A>, eps: &B) -> bool {
        if self.len() != other.len() {
            false
        } else {
            for i in 0..self.len() {
                if self[i].ne(&other[i], eps) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
impl<'a, A: ?Sized, B, C: NearlyEq<A, B> + ?Sized> NearlyEq<A, B> for &'a C {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &A, eps: &B) -> bool {
        (**self).eq(&other, eps)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.0"))]
impl<'a, A: ?Sized, B, C: NearlyEq<A, B> + ?Sized> NearlyEq<A, B> for &'a mut C {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &A, eps: &B) -> bool {
        (**self).eq(&other, eps)
    }
}

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
            #[cfg_attr(feature = "docs", stable(feature = "default", since = "0.1.0"))]
            impl<A, B, C: NearlyEq<A, B>> NearlyEq<[A; $N], B> for [C; $N] {
                fn eps() -> B {
                    C::eps()
                }

                fn eq(&self, other: &[A; $N], eps: &B) -> bool {
                    for i in 0..$N {
                        if self[i].ne(&other[i], eps) {
                            return false;
                        }
                    }
                    true
                }
            }
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.1"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<Option<A>, B> for Option<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Option<A>, eps: &B) -> bool {
            match (self, other) {
                (&None, &None) => return true,
                (&None, _) | (_, &None) => return false,
                (&Some(ref x), &Some(ref y)) => x.eq(y, eps),
            }
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.2"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<Rc<A>, B> for Rc<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Rc<A>, eps: &B) -> bool {
        self.as_ref().eq(other, eps)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.2"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<Arc<A>, B> for Arc<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Arc<A>, eps: &B) -> bool {
        self.as_ref().eq(other, eps)
    }
}

#[cfg_attr(feature = "docs", stable(feature = "default", since = "0.2.2"))]
impl<A, B, C: NearlyEq<A, B>> NearlyEq<Weak<A>, B> for Weak<C> {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &Weak<A>, eps: &B) -> bool {
        self.upgrade().eq(&other.upgrade(), eps)
    }
}

