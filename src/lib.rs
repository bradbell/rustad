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
/// assert_eq!(date, "2025.8.3");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.8.3" );
// ----------------------------------------------------------------------------
//
// ptrait
pub mod ptrait;
//
// utility
pub mod utility;
//
// gad
pub mod gad;
//
// function
pub mod function;
//
// checkpoint
pub mod checkpoint;
//
// gas
pub(crate) mod gas;
//
// operator
pub(crate) mod operator;
//
// record
pub(crate) mod record;
