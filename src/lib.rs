//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! http://mozilla.org/MPL/2.0/.

#[macro_use]
mod assert;

pub trait NearlyEq<Rhs: ?Sized = Self, Diff: ?Sized = Self> {
    fn eps() -> Diff;

    fn eq(&self, other: &Rhs, eps: &Diff) -> bool;

    #[inline]
    fn ne(&self, other: &Rhs, eps: &Diff) -> bool {
        !self.eq(other, eps)
    }
}

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

impl<'a, A: ?Sized, B, C: NearlyEq<A, B> + ?Sized> NearlyEq<A, B> for &'a C {
    fn eps() -> B {
        C::eps()
    }

    fn eq(&self, other: &A, eps: &B) -> bool {
        (*self).eq(&other, eps)
    }
}

macro_rules! array_impls {
    ($($N:expr)+) => {
        $(
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
