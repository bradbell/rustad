// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module defines the FloatCore trait
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust src/float.
//! This enables src/float to be directly included as part of a Dll library.
//!
// ----------------------------------------------------------------------------
/// The FloatCore trait
///
pub trait FloatCore {
    // nan
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     FloatCore,
    /// };
    /// type V = AzFloat<f32>;
    /// let nan_v : V = FloatCore::nan();
    /// // AzFloat<f32> defines nan as equal to itself
    /// assert_eq!( nan_v, nan_v );
    /// // f32 defines nan as not equal to itself
    /// assert_ne!( nan_v.to_inner(), nan_v.to_inner() );
    /// ```
    fn nan()  -> Self;
    //
    // zero
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     NumVec,
    ///     FloatCore,
    /// };
    /// type S = AzFloat<f64>;
    /// type V = NumVec<S>;
    /// let zero_v : V = FloatCore::zero();
    /// let two_v      = V::from( S::from(2) );
    /// assert_eq!( two_v , &zero_v + &two_v );
    /// ```
    fn zero() -> Self;
    //
    // one
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     NumVec,
    ///     FloatCore,
    /// };
    /// type S = AzFloat<f64>;
    /// type V = NumVec<S>;
    /// let one_v : V  = FloatCore::one();
    /// let two_v      = V::from( S::from(2) );
    /// assert_eq!( two_v , &one_v * &two_v );
    /// ```
    fn one()  -> Self;
    //
    // unary functions
    //
    // exp
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     FloatCore,
    /// };
    /// type V = AzFloat<f64>;
    /// let three_v   = V::from( 3.0 );
    /// let exp_three = FloatCore::exp( &three_v );
    /// assert_eq!( exp_three.to_inner(), f64::exp(3.0) );
    /// ```
    fn exp(&self) -> Self;
    //
    // minus
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     FloatCore,
    /// };
    /// type V = AzFloat<f64>;
    /// let one_v : V  = FloatCore::one();
    /// let minus_one  = one_v.minus();
    /// assert_eq!( minus_one.to_inner(), - 1.0f64 );
    /// ```
    fn minus(&self) -> Self;
    //
    // cos
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     FloatCore,
    /// };
    /// type V = AzFloat<f32>;
    /// let one_v : V  = FloatCore::one();
    /// let cos_one    = one_v.cos();
    /// assert_eq!( cos_one.to_inner(), f32::cos( 1.0 ) );
    /// ```
    fn cos(&self) -> Self;
    //
    // sin
    /// ```
    /// use rustad::{
    ///     AzFloat,
    ///     FloatCore,
    /// };
    /// type V = AzFloat<f32>;
    /// let one_v : V  = FloatCore::one();
    /// let sin_one    = FloatCore::sin(&one_v);
    /// assert_eq!( sin_one.to_inner(), f32::sin( 1.0 ) );
    /// ```
    fn sin(&self) -> Self;
}
