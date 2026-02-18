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
///     res = lhs.cmp(&rhs)
///   ```
///   where either lhs or rhs has type `AD<V>` .
///
/// * lhs : is the left comparison operand .
/// * rhs : is the right comparison operand .
/// * cmp : is one of the following :
///   `num_lt` , `num_le`, `num_eq`, `num_ne`, `num_ge`, `num_gt`
/// * res : is one (zero) if the comparison result is true (false).
pub trait NumCmp<Rhs> {
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
}
