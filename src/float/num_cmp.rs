// ---------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the NumCmp trait.
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust and src/float.
//! This enables src/float to be directly included as part of a Dll library.
//!
// ---------------------------------------------------------------------------
//
// NumCmp
/// These comparisons results have the numeric value
/// 1 for true and 0 for false.
///
/// * Syntax :
///   ```text
///     result = lhs.num_lt(&rhs)
///     result = lhs.num_le(&rhs)
///     result = lhs.num_eq(&rhs)
///     result = lhs.num_ne(&rhs)
///     result = lhs.num_ge(&rhs)
///     result = lhs.num_gt(&rhs)
///   ```
///
/// * lhs  : is the left hand side operand in the comparison
/// * rhs  : is the right hand side operand in the comparison
/// * result :
///   * Type :
///     If lhs and rhs have the same type, result has that type.
///     Otherwise either lhs or rhs has type `AD<V>` .
///     If lhs (rhs) has type `AD<V>` and rhs (lhs) has type `V` ,
///     result has type `AD<V>` .
///   * Value :
///     The result will have value one (zero) if the comparison is true (false).
///     If the result is a [NumVec](crate::NumVec) type or `AD<V>` where `V`
///     is an NumVec type, the result is an element wise comparison.
///
pub trait NumCmp<Rhs = Self> {
    type Output;
    //
    /// self < rhs
    fn num_lt(&self, rhs : &Rhs) -> Self::Output;
    //
    /// self <= rhs
    fn num_le(&self, rhs : &Rhs) -> Self::Output;
    //
    /// self == rhs
    fn num_eq(&self, rhs : &Rhs) -> Self::Output;
    //
    /// self != rhs
    fn num_ne(&self, rhs : &Rhs) -> Self::Output;
    //
    /// self >= rhs
    fn num_ge(&self, rhs : &Rhs) -> Self::Output;
    //
    /// self > rhs
    fn num_gt(&self, rhs : &Rhs) -> Self::Output;
}
