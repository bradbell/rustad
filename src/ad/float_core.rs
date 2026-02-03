// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module implements FloatCore for AD types
//!
//! Link to [parent module](super)
//!
//!
// ---------------------------------------------------------------------------
use crate::{
    FloatCore,
    AD,
};
//
/// Implements the FloatCore trait for AD types
impl<V> FloatCore for AD<V>
where
    V : FloatCore,
{
        fn nan()  -> Self { Self::from( V::nan() ) }
        fn zero() -> Self { Self::from( V::zero() ) }
        fn one()  -> Self { Self::from( V::one() ) }
}
