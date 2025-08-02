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
/// assert_eq!(date, "2025.7.11");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.7.11" );
// ----------------------------------------------------------------------------
//
// gas
pub mod ptrait;
//
// utility
pub mod utility;
//
// ad
pub mod ad;
//
// operator
pub(crate) mod operator;
//
// record
pub(crate) mod record;
//
// function
pub mod function;
//
// checkpoint
pub mod checkpoint;
//
