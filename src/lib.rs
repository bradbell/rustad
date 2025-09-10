// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
//
//! The rustad Automatic Differentiation Package
// ----------------------------------------------------------------------------
// sub-modules
//
// utility
pub mod utility;
//
// numvec
pub mod numvec;
//
// vec_set
pub(crate) mod vec_set;
//
// ----------------------------------------------------------------------------
// use
// https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html
//
// ----------------------------------------------------------------------------
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.9.10");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.9.10" );
