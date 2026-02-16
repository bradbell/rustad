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
///     result = left.num_lt(&right)
///     result = left.num_le(&right)
///     result = left.num_eq(&right)
///     result = left.num_ne(&right)
///     result = left.num_ge(&right)
///     result = left.num_gt(&right)
///   ```
///
/// * left  : is the left operand in the comparison
/// * right : is the right operand in the comparison
/// * result :
///   * Type :
///     If left and right have the same type, result has that type.
///     Otherwise either left or right has type `AD<V>` .
///     If left (right) has type `AD<V>` and right (left) has type `V` ,
///     result has type `AD<V>` .
///   * Value :
///     The result will have value one (zero) if the comparison is true (false).
///     If the result is a [NumVec](crate::NumVec) type or `AD<V>` where `V`
///     is an NumVec type, the result is an element wise comparison.
///
pub trait NumCmp<Right = Self> {
    type Output;
    //
    /// self < right
    fn num_lt(&self, right : &Right) -> Self::Output;
    //
    /// self <= right
    fn num_le(&self, right : &Right) -> Self::Output;
    //
    /// self == right
    fn num_eq(&self, right : &Right) -> Self::Output;
    //
    /// self != right
    fn num_ne(&self, right : &Right) -> Self::Output;
    //
    /// self >= right
    fn num_ge(&self, right : &Right) -> Self::Output;
    //
    /// self > right
    fn num_gt(&self, right : &Right) -> Self::Output;
}
