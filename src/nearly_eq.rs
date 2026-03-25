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
    FConst,
    FUnary,
    FBinary,
    FValue,
};
// ----------------------------------------------------------------------------
// nearly_eq
/// Check if two values are nearly equal.
///
/// * Syntax :
/// ```text
///     bval = nearly_eq(x, y, opt_vec)
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
/// * opt_vec :
///   is an [opt_vec](crate::doc_opt_vec) with the following possible keys:
///
///     * factor :
///       is a string representation of an f32 value that multiplies
///       the minimum positive value and machine epsilon; see below.
///       The default factor is 100.
///
///     * assert :
///       must be true or false. If it is true, nearly_eq
///       will panic with an error message when the comparison (bval) is false.
///       The default value for assert is true.
///
/// * min_positive :
///   use use the notaiton [min_positive](FConst::min_positive) below.
///
/// * epsilon :
///   use use the notaiton [epsilon](FConst::epsilon) below.
///
/// * bval :
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
///     FConst,
///     FUnary,
///     nearly_eq,
/// };
/// type V = AzFloat<f32>;
/// //
/// //
/// let one_v           = V::from(1);
/// let epsilon_v       = V::epsilon();
/// let near_one_v      = one_v + V::from(10) * epsilon_v;
/// let x               = V::from( 1e-20 );
/// let y               = x * near_one_v;
/// let mut opt_vec     = vec![ ["assert", "false"] ];
/// assert!( nearly_eq::<V>(&x, &y, &opt_vec) );
/// opt_vec.push( ["factor", "2"] );
/// assert!( ! nearly_eq::<V>( &x, &y, &opt_vec ) );
/// ```
///
/// # NumVec Example :
/// ```
/// use rustad::{
///     AzFloat,
///     NumVec,
///     FConst,
///     FUnary,
///     nearly_eq,
/// };
/// type S = AzFloat<f32>;
/// type V = NumVec<S>;
/// //
/// let opt_vec = vec![ ["assert", "false"] ];
/// //
/// let one_v           = V::one();
/// let epsilon_v       = V::epsilon();
/// let near_one_v      = &one_v + &( &V::from(10f32) * &epsilon_v );
/// //
/// let x  = V::new( vec![ S::from(1e-20) ,  S::from(1e+20) ] );
/// let y  = &x * &near_one_v;
/// assert!( nearly_eq::<V>(&x, &y, &opt_vec) );
/// //
/// let y  = V::new( vec![ S::from(1.01e-20) ,  S::from(1e+20) ] );
/// assert!( ! nearly_eq::<V>(&x, &y, &opt_vec) );
/// ```
///
pub fn nearly_eq<V>(x : &V, y : &V, opt_vec : &Vec< [&str; 2] >) -> bool
where
    V  : FConst + FValue + From<f32> + std::fmt::Debug,
    for<'a> &'a V : FUnary<Output=V>,
    for<'a> &'a V : FBinary<&'a V, Output = V> ,
    for<'a> &'a V : Add<&'a V, Output=V> ,
    for<'a> &'a V : Mul<&'a V, Output=V> ,
    for<'a> &'a V : Sub<&'a V, Output=V> ,
    for<'a> &'a V : Div<&'a V, Output=V> ,
{   //
    // factor, assert
    let mut factor  = V::from(100f32);
    let mut assert  = true;
    for opt in opt_vec {
        match opt[0] {
            "factor" => {
                let result = opt[1].parse::<f32>();
                if result.is_err() { panic!(
                    "nearly_eq opt_vec: can't convert factor to f32"
                ); }
                factor = V::from( result.unwrap() );
            },
            "assert" => {
                match opt[1] {
                    "true"  => { assert = true; }
                    "false" => { assert = false; }
                    _ => { panic!(
                        "nearly_eq opt_vec: assert is not true of false"
                    ); }
                }
            },
            _ => panic!( "nearly_eq opt_vec: invalid key" ),
        }
    }
    //
    //
    // sum_abs, min_sum
    let sum_abs     = &x.abs() + &y.abs();
    let min_pos     = V::min_positive();
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
    let min_diff = &factor * &FConst::epsilon();
    //
    // check second condition
    let ratio  = &abs_diff / &sum_abs;
    let lt_min = ratio.num_lt(&min_diff);
    if lt_min.is_one() {
        return true;
    }
    if assert {
        panic!(
            "nearly_eq panic:\n\
            x                     = {:?}\n\
            y                     = {:?}\n\
            factor * min_positive = {:?}\n\
            factor * epsilon      = {:?}\n\
            Set RUST_BACKTRACE=1 to see this call to nearly_eq",
            x, y, min_sum, min_diff
        );
    }
    //
    false
}
