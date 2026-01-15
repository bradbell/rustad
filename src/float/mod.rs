// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines the SimpleFloat trait
//!
//! Link to [parent module](super)
// ----------------------------------------------------------------------------
// use
use crate::{
    AD,
};
// ----------------------------------------------------------------------------
/// The SimpleFloat trait
///
pub trait SimpleFloat {
    fn nan() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}
//
// impl_simple_float_for_az_float
/// Implements the SimpleFloat trait for
/// `AzFloat<P>` and `NumVec< AzFloat<P> >`
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_simple_float_for_az_float{ ($P:ident) => {
    impl crate::float::SimpleFloat for crate::AzFloat<$P> {
        fn nan()  -> Self { Self::from( $P::NAN ) }
        fn zero() -> Self { Self::from( 0 as $P ) }
        fn one()  -> Self { Self::from( 1 as $P ) }
    }
    impl crate::float::SimpleFloat for crate::NumVec< AzFloat<$P> > {
        fn nan()  -> Self { Self::from( AzFloat::<$P>::nan() ) }
        fn zero() -> Self { Self::from( AzFloat::<$P>::zero() ) }
        fn one()  -> Self { Self::from( AzFloat::<$P>::one() ) }
    }
} }
pub(crate) use impl_simple_float_for_az_float;
//
/// Implements the SimpleFloat trait for `AD<V>`
impl<V> SimpleFloat for AD<V>
where
    V : SimpleFloat,
{
        fn nan()  -> Self { Self::from( V::nan() ) }
        fn zero() -> Self { Self::from( V::zero() ) }
        fn one()  -> Self { Self::from( V::one() ) }
}
