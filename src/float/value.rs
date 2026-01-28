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
    // is_zero
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
    // is_one
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
    // is_nan
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
    //
    // to_src
    /// Generates rust source code that corresponds to this value.
    /// The exact form of this source is unspecified and may change.
    ///
    /// # Example
    ///
    /// # Example
    /// ```
    /// use rustad::AzFloat;
    /// use rustad::FloatValue;
    /// let two : AzFloat<f32> = 2.0.into();
    /// assert_eq!( two.to_src(), "AzFloat(2 as f32)" );
    ///
    /// use rustad::NumVec;
    /// let three : NumVec<AzFloat<f32>> = 3.0.into();
    /// let check = "NumVec::new( vec![ AzFloat(3 as f32), ] )";
    /// assert_eq!(three.to_src(), check);
    /// ```
    fn to_src(&self) -> String;
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
        fn to_src(&self)   -> String {
            if self.is_nan() {
                "AzFloat( f32::NAN as ".to_string() + stringify!($P) + ")"
            } else {
                "AzFloat(".to_string() +
                    &self.0.to_string() + " as " + stringify!($P) +
                ")"
            }
        }
    }
    impl crate::float::value::FloatValue for crate::NumVec< AzFloat<$P> > {
        // is_zero
        fn is_zero(&self)  -> bool {
            let mut all_zero = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_zero      = all_zero && primitive == (0 as $P)
            }
            all_zero
        }
        // is_one
        fn is_one(&self)  -> bool {
            let mut all_one = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_one       = all_one && primitive == (1 as $P)
            }
            all_one
        }
        // is_nan
        fn is_nan(&self)  -> bool {
            let mut all_nan = true;
            for i in 0 .. self.len() {
                let primitive = self.get(i).0;
                all_nan       = all_nan && primitive != primitive;
            }
            all_nan
        }
        // to_src
        fn to_src(&self) -> String {
            let mut src = "NumVec::new( vec![ ".to_string();
            for i in 0 .. self.len() {
                if self.get(i).is_nan() {
                    src = src + "AzFloat(f32::NAN as " + stringify!($P) + "), ";
                } else {
                    src = src + "AzFloat(" +
                        &self.get(i).to_string() + " as " + stringify!($P) +
                    "), ";
                }
            }
            src += "] )";
            src
        }
    }
} }
pub(crate) use impl_float_value_for_az_float;
