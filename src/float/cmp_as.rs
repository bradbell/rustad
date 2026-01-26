// ============================================================================
// BEGIN az_float.rs
// ============================================================================
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the CmpAsLhs and CmpAsRhs traits.
//!
//! Link to [parent module](super)
//!
//! This module does not have dependencies outside standard rust and src/float.
//! This enables src/float to be directly included as part of a Dll library.
//!
// ---------------------------------------------------------------------------
//
// CmpAsLhs
/// These comparisons results are 1 for true and 0 for false and
/// have the same type as the left operand.
///
/// For cmp equal to lt, le, eq, ne, ge, gt :
/// The left_cmp function returns one (zero) if
/// self compare other is true (false).
///
/// The not operator will return zero (one)
pub trait CmpAsLhs<Rhs = Self> {
    /// self < other
    fn left_lt(&self, other : &Rhs) -> Self;
    /// self <= other
    fn left_le(&self, other : &Rhs) -> Self;
    /// self == other
    fn left_eq(&self, other : &Rhs) -> Self;
    /// self != other
    fn left_ne(&self, other : &Rhs) -> Self;
    /// self >= other
    fn left_ge(&self, other : &Rhs) -> Self;
    /// self > other
    fn left_gt(&self, other : &Rhs) -> Self;
}
//
// CmpAsRhs Trait
/// These comparisons results are 1 for true and 0 for false and
/// have the same type as the right operand.
///
/// For cmp equal to lt, le, eq, ne, ge, gt :
/// The cmp_right function returns one (zero) if
/// self compare other is true (false).
///
/// The not operator will return zero (one)
pub trait CmpAsRhs<Rhs = Self> {
    /// self < other
    fn lt_right(&self, other : &Rhs) -> Rhs;
    /// self <= other
    fn le_right(&self, other : &Rhs) -> Rhs;
    /// self == other
    fn eq_right(&self, other : &Rhs) -> Rhs;
    /// self != other
    fn ne_right(&self, other : &Rhs) -> Rhs;
    /// self >= other
    fn ge_right(&self, other : &Rhs) -> Rhs;
    /// self > other
    fn gt_right(&self, other : &Rhs) -> Rhs;
}
