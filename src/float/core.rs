// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines the FloatCore trait
//!
//! Link to [parent module](super)
// ----------------------------------------------------------------------------
// use
use crate::{
    AD,
};
// ----------------------------------------------------------------------------
/// The FloatCore trait
///
pub trait FloatCore {
    fn nan() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}
//
// impl_float_core_for_az_float
/// Implements the FloatCore trait for
/// `AzFloat<P>` and `NumVec< AzFloat<P> >`
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_float_core_for_az_float{ ($P:ident) => {
    impl crate::float::core::FloatCore for crate::AzFloat<$P> {
        fn nan()  -> Self { Self::from( $P::NAN ) }
        fn zero() -> Self { Self::from( 0 as $P ) }
        fn one()  -> Self { Self::from( 1 as $P ) }
    }
    impl crate::float::core::FloatCore for crate::NumVec< AzFloat<$P> > {
        fn nan()  -> Self { Self::from( AzFloat::<$P>::nan() ) }
        fn zero() -> Self { Self::from( AzFloat::<$P>::zero() ) }
        fn one()  -> Self { Self::from( AzFloat::<$P>::one() ) }
    }
} }
pub(crate) use impl_float_core_for_az_float;
//
/// Implements the FloatCore trait for `AD<V>`
impl<V> FloatCore for AD<V>
where
    V : FloatCore,
{
        fn nan()  -> Self { Self::from( V::nan() ) }
        fn zero() -> Self { Self::from( V::zero() ) }
        fn one()  -> Self { Self::from( V::one() ) }
}
