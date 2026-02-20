// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines the FloatValue trait and related functions.
//!
//! Link to [parent module](super)
// ----------------------------------------------------------------------------
// use
use std::ops::{
    Add,
    Mul,
    Sub,
    Div,
};
//
use crate::{
    FloatCore,
    NumCmp,
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
// impl_float_value_from_primitive
/// Implements the FloatValue trait for value types
/// `AzFloat<P>` and `NumVec< AzFloat<P> >`
///
/// * P : is a primitive type; i.e., f32 or f64;
macro_rules! impl_float_value_from_primitive{ ($P:ident) => {
    impl crate::float_value::FloatValue for crate::AzFloat<$P> {
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
    impl crate::float_value::FloatValue for crate::NumVec< AzFloat<$P> > {
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
pub(crate) use impl_float_value_from_primitive;
// ----------------------------------------------------------------------------
// check_nearly_eq
/// Check if two values are nearly equal.
///
/// * Syntax :
/// ```text
///     flag = check_nearly_eq(x, y, arg_vec)
/// ```
///
/// * V : see [doc_generic_v](crate::doc_generic_v)
///
/// * x :
///   one of the values that we are comparing.
///
/// * y :
///   the other value that we are comparing.
///
/// * arg_vec :
///   is an [arg_vec](crate::doc_arg_vec) with the following possible keys:
///
///     * factor :
///       is a string representation of an f32 value that multiplies
///       the minimum positive value and machine epsilon; see below.
///       The default factor is 100.
///
///     * assert :
///       must be true or false. If it is true, check_nearly_eq
///       will panic with an error message when the comparison (flag) is false.
///       The default value for assert is true.
///
/// * min_positive :
///   use use the notaiton [min_positive](FloatCore::min_positive) below.
///
/// * epsilon :
///   use use the notaiton [epsilon](FloatCore::epsilon) below.
///
/// * flag :
///   the return value is true if either of the following conditions hold:
///   ```text
///   |x| + |y|             < factor * min_positive
///   |x - y| / (|x| + |y|) < factor * epsilon
///   ```
///   Note that for a [NumVec](crate::NumVec) type,
///   one of the conditions must hold for each elements of the vector.
///
/// # AzFloat Example :
/// ```
/// use rustad::{
///     AzFloat,
///     FloatCore,
///     check_nearly_eq,
/// };
/// type V = AzFloat<f32>;
/// //
/// //
/// let one_v           = V::from(1);
/// let epsilon_v   : V = FloatCore::epsilon();
/// let near_one_v      = one_v + V::from(10) * epsilon_v;
/// let x               = V::from( 1e-20 );
/// let y               = x * near_one_v;
/// let mut arg_vec     = vec![ ["assert", "false"] ];
/// assert!( check_nearly_eq::<V>(&x, &y, &arg_vec) );
/// arg_vec.push( ["factor", "2"] );
/// assert!( ! check_nearly_eq::<V>( &x, &y, &arg_vec ) );
/// ```
///
/// # NumVec Example :
/// ```
/// use rustad::{
///     AzFloat,
///     NumVec,
///     FloatCore,
///     check_nearly_eq,
/// };
/// type S = AzFloat<f32>;
/// type V = NumVec<S>;
/// //
/// let arg_vec = vec![ ["assert", "false"] ];
/// //
/// let one_v       : V = FloatCore::one();
/// let epsilon_v   : V = FloatCore::epsilon();
/// let near_one_v      = &one_v + &( &V::from(10f32) * &epsilon_v );
/// //
/// let x  = V::new( vec![ S::from(1e-20) ,  S::from(1e+20) ] );
/// let y  = &x * &near_one_v;
/// assert!( check_nearly_eq::<V>(&x, &y, &arg_vec) );
/// //
/// let y  = V::new( vec![ S::from(1.01e-20) ,  S::from(1e+20) ] );
/// assert!( ! check_nearly_eq::<V>(&x, &y, &arg_vec) );
/// ```
///
pub fn check_nearly_eq<V>(x : &V, y : &V, arg_vec : &Vec< [&str; 2] >) -> bool
where
    V  : FloatCore + FloatValue + From<f32> + std::fmt::Debug,
    for<'a> &'a V : NumCmp<&'a V, Output = V> ,
    for<'a> &'a V : Add<&'a V, Output=V> ,
    for<'a> &'a V : Mul<&'a V, Output=V> ,
    for<'a> &'a V : Sub<&'a V, Output=V> ,
    for<'a> &'a V : Div<&'a V, Output=V> ,
{   //
    // factor, assert
    let mut factor  = V::from(100f32);
    let mut assert  = true;
    for arg in arg_vec {
        match arg[0] {
            "factor" => {
                let result = arg[1].parse::<f32>();
                if result.is_err() { panic!(
                    "check_nearly_eq arg_vec: can't convert factor to f32"
                ); }
                factor = V::from( result.unwrap() );
            },
            "assert" => {
                match arg[1] {
                    "true"  => { assert = true; }
                    "false" => { assert = false; }
                    _ => { panic!(
                        "check_nearly_eq arg_vec: assert is not true of false"
                    ); }
                }
            },
            _ => panic!( "check_nearly_eq arg_vec: invalid key" ),
        }
    }
    //
    //
    // sum_abs, min_sum
    let sum_abs     = &x.abs() + &y.abs();
    let min_pos : V = FloatCore::min_positive();
    let min_sum     = &factor * &min_pos;
    //
    // check first condition
    let lt_min  = sum_abs.num_lt(&min_sum);
    if lt_min.is_one() {
        return true;
    }
    //
    // abs_diff, min_diff
    let abs_diff = (x - y).abs();
    let min_diff = &factor * &FloatCore::epsilon();
    //
    // check second condition
    let ratio  = &abs_diff / &sum_abs;
    let lt_min = ratio.num_lt(&min_diff);
    if lt_min.is_one() {
        return true;
    }
    if assert {
        panic!(
            "check_nearly_eq panic:\n\
            x                     = {:?}\n\
            y                     = {:?}\n\
            factor * min_positive = {:?}\n\
            factor * epsilon      = {:?}\n\
            Set RUST_BACKTRACE=1 to see this call to check_nearly_eq",
            x, y, min_sum, min_diff
        );
    }
    //
    false
}
