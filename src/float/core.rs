// ---------------------------------------------------------------------------
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
/// See the file examples/float_core.rs .
pub trait FloatCore {
    // ------------------------------------------------------------------------
    // No Arguments
    // ------------------------------------------------------------------------
    fn pi()           -> Self;
    fn nan()          -> Self;
    fn one()          -> Self;
    fn zero()         -> Self;
    fn epsilon()      -> Self;
    fn min_positive() -> Self;
    // ------------------------------------------------------------------------
    // unary functions
    // ------------------------------------------------------------------------
    fn sqrt(&self) -> Self;
    fn tanh(&self) -> Self;
    fn tan(&self) -> Self;
    fn sinh(&self) -> Self;
    fn cosh(&self) -> Self;
    fn abs(&self) -> Self;
    fn exp(&self) -> Self;
    fn minus(&self) -> Self;
    fn cos(&self) -> Self;
    fn signum(&self) -> Self;
    fn sin(&self) -> Self;
    // ------------------------------------------------------------------------
    // binary functions
    // ------------------------------------------------------------------------
    fn powi(&self, rhs : i32) -> Self;
}
