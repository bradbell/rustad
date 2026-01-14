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
// impl_simple_float_from_function
/// Implements one SimpleFloat function
///
/// * Self : From must map from the type of E to this type.
/// * F : is the function that is implemented.
/// * E : a expression equal to the value we a returning.
macro_rules! simple_float_function{ ($F:ident, $E:expr) => {
    fn $F() -> Self {
        Self::from( $E )
    }
} }
pub(crate) use simple_float_function;
//
// impl_simple_float_for_az_float
/// Implements the SimpleFloat trait for
/// `AzFloat<P>` and `NumVec< AzFloat<P> >`
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_simple_float_for_az_float{ ($P:ident) => {
    impl crate::simple_float::SimpleFloat for crate::AzFloat<$P> {
    crate::simple_float::simple_float_function!(nan, $P::NAN );
    crate::simple_float::simple_float_function!(zero, (0 as $P) );
    crate::simple_float::simple_float_function!(one, (1 as $P) );
    }
    impl crate::simple_float::SimpleFloat for crate::NumVec< AzFloat<$P> > {
    crate::simple_float::simple_float_function!(nan, AzFloat::<$P>::nan() );
    crate::simple_float::simple_float_function!(zero, AzFloat::<$P>::zero() );
    crate::simple_float::simple_float_function!(one, AzFloat::<$P>::one() );
    }
} }
pub(crate) use impl_simple_float_for_az_float;
//
/// Implements the SimpleFloat trait for `AD<V>`
impl<V> SimpleFloat for AD<V>
where
    V : SimpleFloat,
{
    simple_float_function!(nan, V::nan() );
    simple_float_function!(zero, V::zero() );
    simple_float_function!(one, V::one() );
}
