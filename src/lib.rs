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
// gad
pub(crate) mod gad;
//
// function
pub(crate) mod function;
//
// ptrait
pub(crate) mod ptrait;
//
// checkpoint
pub(crate) mod checkpoint;
//
// vec_set
pub(crate) mod vec_set;
//
// gas
pub(crate) mod gas;
//
// operator
pub(crate) mod operator;
//
// record
pub(crate) mod record;
// ----------------------------------------------------------------------------
// use
// https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html
//
pub use crate::gad::{
    GAD,
    doc_gad_from,
    doc_binary_gad_operator,
    doc_binary_gad_assign_op,
};
pub use crate::function::{
    GADFun,
    ad_domain,
    ad_fun,
};
pub use crate::function::sweep::{
    doc_forward_zero,
    doc_forward_one,
    doc_reverse_one,
};
//
pub use crate::ptrait::{
    ThisThreadTapePublic,
    ThisThreadCheckpointAllPublic,
};
pub use crate::checkpoint::{
    store_checkpoint,
    use_checkpoint,
};
// ----------------------------------------------------------------------------
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.8.9");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.8.9" );
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
