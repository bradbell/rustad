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
/// assert_eq!(date, "2025.8.8");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.8.8" );
//
/// Document the rustad generic type parameters F and U.
///
/// * F :
/// is the floating point type used for value calculations.
/// To date the possible choices for *F* are f32 or f64 .
///
/// * U :
/// is the unsigned integer type that that is used to identify
/// components in an operation sequence.
/// It must be able to represent the maximum:
/// tape id, operator index, constant index, and operator argument index.
/// To date the possible choices for *U* are u32 or u64 .
///
pub fn doc_generic_f_and_u() {}
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
// vec_set
// 2DO: change this module to be pub(crate).
pub mod vec_set;
//
// gas
pub(crate) mod gas;
//
// operator
pub(crate) mod operator;
//
// record
pub(crate) mod record;
