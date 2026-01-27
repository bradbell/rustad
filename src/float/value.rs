// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines the FloatValue trait
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust src/float.
//! This enables src/float to be directly included as part of a Dll library.
//!
// ----------------------------------------------------------------------------
// use
use crate::{
    FloatCore,
};
// ----------------------------------------------------------------------------
/// The FloatValue trait
///
pub trait FloatValue : FloatCore {
    //
    /// True if all the components of self are zero.
    ///
    /// # Example
    /// ```
    /// use rustad::AzFloat;
    /// use rustad::FloatValue;
    /// let zero : AzFloat<f32> = 0.0.into();
    /// let one  : AzFloat<f32> = 1.0.into();
    /// assert!( zero.is_zero() );
    /// assert!( ! one.is_zero() );
    /// ```
    fn is_zero(&self) -> bool;
    //
    /// True if all the components of self are one.
    ///
    /// # Example
    /// ```
    /// use rustad::AzFloat;
    /// use rustad::NumVec;
    /// use rustad::FloatValue;
    /// type S               = AzFloat<f64>;
    ///
    /// let value : NumVec<S> = NumVec::new(vec![ S::from(1.0), S::from(0.0) ]);
    /// assert!( ! value.is_one() );
    ///
    /// let value : NumVec<S> = NumVec::new(vec![ S::from(1.0), S::from(1.0) ]);
    /// assert!( value.is_one() );
    /// ```
    fn is_one(&self) -> bool;
    //
    /// True if all the components of self are nan.
    ///
    /// # Example
    /// ```
    /// use rustad::AzFloat;
    /// use rustad::NumVec;
    /// use rustad::FloatValue;
    /// type S               = AzFloat<f64>;
    /// let nan_s             = S::from(f32::NAN);
    ///
    /// let value : NumVec<S> = NumVec::new(vec![ nan_s, S::from(0.0) ]);
    /// assert!( ! value.is_nan() );
    ///
    /// let value : NumVec<S> = NumVec::new(vec![ nan_s, nan_s ]);
    /// assert!( value.is_nan() );
    /// ```
    fn is_nan(&self) -> bool;
}
//
// impl_float_value_for_az_float
/// Implements the FloatValue trait for value types
/// `AzFloat<P>` and `NumVec< AzFloat<P> >`
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_float_value_for_az_float{ ($P:ident) => {
    impl crate::float::value::FloatValue for crate::AzFloat<$P> {
        fn is_zero(&self)  -> bool { self.0 == ( 0 as $P ) }
        fn is_one(&self)   -> bool { self.0 == ( 1 as $P ) }
        fn is_nan(&self)   -> bool { self.0 != self.0 }
    }
    impl crate::float::value::FloatValue for crate::NumVec< AzFloat<$P> > {
        fn is_zero(&self)  -> bool {
            let mut all_zero = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_zero      = all_zero && primitive == (0 as $P)
            }
            all_zero
        }
        fn is_one(&self)  -> bool {
            let mut all_one = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_one       = all_one && primitive == (1 as $P)
            }
            all_one
        }
        fn is_nan(&self)  -> bool {
            let mut all_nan = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_nan       = all_nan && primitive != primitive;
            }
            all_nan
        }
    }
} }
pub(crate) use impl_float_value_for_az_float;
