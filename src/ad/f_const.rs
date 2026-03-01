// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module implements FConst for AD types
//!
//! Link to [parent module](super)
//!
//!
// ---------------------------------------------------------------------------
use crate::{
    FConst,
    AD,
};
/// Implements the FConst trait for AD types
impl<V> FConst for AD<V>
where
    V : Clone + FConst
{
    fn pi()            -> AD<V> { AD::<V>::from( V::pi() ) }
    fn nan()           -> AD<V> { AD::<V>::from( V::nan() ) }
    fn one()           -> AD<V> { AD::<V>::from( V::one() ) }
    fn zero()          -> AD<V> { AD::<V>::from( V::zero() ) }
    fn epsilon()       -> AD<V> { AD::<V>::from( V::epsilon() ) }
    fn min_positive()  -> AD<V> { AD::<V>::from( V::min_positive() ) }
}
