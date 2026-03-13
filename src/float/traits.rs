// ---------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines some floating point traits
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust and src/float.
//! This enables src/float to be directly included as part of a Dll library.
//!
// ---------------------------------------------------------------------------
// ----------------------------------------------------------------------------
/// The FConst trait
///
/// See the file examples/f_const.rs .
pub trait FConst {
    // ------------------------------------------------------------------------
    // No Arguments
    // ------------------------------------------------------------------------
    fn pi()           -> Self;
    fn nan()          -> Self;
    fn one()          -> Self;
    fn zero()         -> Self;
    fn epsilon()      -> Self;
    fn min_positive() -> Self;
}
// ----------------------------------------------------------------------------
/// The floating point unary function trait
///
/// # Example
/// See the file examples/f_unary.rs .
pub trait FUnary {
    type Output;
    // ------------------------------------------------------------------------
    // BEGIN_SORT_THIS_LINE_PLUS_1
    fn abs(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
    fn exp(self) -> Self::Output;
    fn exp_m1(self) -> Self::Output;
    fn ln(self) -> Self::Output;
    fn ln_1p(self) -> Self::Output;
    fn minus(self) -> Self::Output;
    fn signum(self) -> Self::Output;
    fn sin(self) -> Self::Output;
    fn sinh(self) -> Self::Output;
    fn sqrt(self) -> Self::Output;
    fn square(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    // END_SORT_THIS_LINE_MINUS_1
    // ------------------------------------------------------------------------
    fn powi(self, rhs : i32) -> Self::Output;
}
//
// FBinary
/// The floating point binary function trait
///
/// * Syntax :
///   ```text
///     res = lhs.name(rhs)
///     res = FBinary::name(lhs, rhs)
///   ```
///
/// * lhs  : is the first function argument.
/// * rhs  : is the second function argument.
/// * res  : is the function result.
///
/// * Numical Comparison :
///   The following names are used for numerical comparison operators:
///   num_lt , num_le, num_eq, num_ne, num_ge, num_gt.
///   These function return the floating point value
///   one for true and zero for false.
///
/// Example
/// See the file examples/f_binary.rs
///
pub trait FBinary<Rhs> {
    type Output;
    //
    /// self < rhs
    fn num_lt(self, rhs : Rhs) -> Self::Output;
    //
    /// self <= rhs
    fn num_le(self, rhs : Rhs) -> Self::Output;
    //
    /// self == rhs
    fn num_eq(self, rhs : Rhs) -> Self::Output;
    //
    /// self != rhs
    fn num_ne(self, rhs : Rhs) -> Self::Output;
    //
    /// self >= rhs
    fn num_ge(self, rhs : Rhs) -> Self::Output;
    //
    /// self > rhs
    fn num_gt(self, rhs : Rhs) -> Self::Output;
    //
    // hypot(self, rhs)
    fn hypot(self, rhs : Rhs) -> Self::Output;
    //
    /// self^rhs
    fn powf(self, rhs : Rhs) -> Self::Output;
}
// ----------------------------------------------------------------------------
