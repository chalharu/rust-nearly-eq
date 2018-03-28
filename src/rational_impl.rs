//! # Licensing
//! This Source Code is subject to the terms of the Mozilla Public License
//! version 2.0 (the "License"). You can obtain a copy of the License at
//! [http://mozilla.org/MPL/2.0/](http://mozilla.org/MPL/2.0/).

use num_rational::Ratio;
use num_integer::Integer;
use num_traits::identities::Zero;
use NearlyEq;

#[cfg_attr(feature = "docs", stable(feature = "rational", since = "0.2.1"))]
impl<A: Integer + Clone> NearlyEq<Ratio<A>, Ratio<A>> for Ratio<A> {
    fn eps() -> Ratio<A> {
        Ratio::zero()
    }

    fn eq(&self, other: &Ratio<A>, eps: &Ratio<A>) -> bool {
        let diff = if *self > *other {
            self.clone() - other.clone()
        } else {
            other.clone() - self.clone()
        };

        if *self == *other {
            true
        } else {
            diff < *eps
        }
    }
}
