// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
//
//! The rustad Automatic Differentiation Package
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.6.19");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.19" );
// ----------------------------------------------------------------------------
//
// utility
pub mod utility;
//
// Index
/// Type used for indexing vectors in the tape.
/// It must be able to represent the total number of
/// tape ids, operator indices, constants, and arguments to operators.
pub type Index = usize;
//
// Float
/// Floating point Type used for AD operations.
pub type Float = f64;
//
// AD
pub mod ad;
pub use ad::AD;
//
// ad_tape
pub(crate) mod ad_tape;
//
// operator
pub(crate) mod operator;
//
// function
pub mod function;
