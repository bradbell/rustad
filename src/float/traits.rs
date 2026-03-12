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
/// The FUnary trait
///
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
    fn tan(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    // END_SORT_THIS_LINE_MINUS_1
    // ------------------------------------------------------------------------
    fn powi(self, rhs : i32) -> Self::Output;
}
//
// FBinary
/// These comparisons results have the numeric value
/// 1 for true and 0 for false.
///
/// * Syntax :
///   ```text
///     res = lhs.cmp(rhs)
///   ```
///   where either lhs or rhs has type `AD<V>` .
///
/// * lhs : is the left comparison operand .
/// * rhs : is the right comparison operand .
/// * cmp : is one of the following :
///   `num_lt` , `num_le`, `num_eq`, `num_ne`, `num_ge`, `num_gt`
/// * res : is one (zero) if the comparison result is true (false).
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
    /// self^rhs
    fn powf(self, rhs : Rhs) -> Self::Output;
}
// ----------------------------------------------------------------------------
