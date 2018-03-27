//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! [http://mozilla.org/MPL/2.0/](http://mozilla.org/MPL/2.0/).

use ndarray::{ArrayBase, Axis, Data, Dimension};
use NearlyEq;

#[cfg_attr(feature = "docs", stable(feature = "ndarray", since = "0.2.0"))]
impl<A: Data, B, C: Data, D: Dimension> NearlyEq<ArrayBase<A, D>, B> for ArrayBase<C, D>
where
    C::Elem: NearlyEq<A::Elem, B> + Sized,
{
    fn eps() -> B {
        C::Elem::eps()
    }

    fn eq(&self, other: &ArrayBase<A, D>, eps: &B) -> bool {
        if self.ndim() != other.ndim() {
            return false;
        }
        for n in 0..self.ndim() {
            if self.len_of(Axis(n)) != other.len_of(Axis(n)) {
                return false;
            }
        }

        let mut own = self.iter();
        let mut other = other.iter();

        loop {
            match (own.next(), other.next()) {
                (None, None) => return true,
                (None, _) | (_, None) => return false,
                (Some(x), Some(y)) => if !x.eq(y, eps) {
                    return false;
                },
            }
        }
    }
}
