// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
//
//! A rust Automatic Differentiation library
//!
//! This library is Under Construction and its API is not stable; see its
//! [readme file](<https://github.com/bradbell/rustad/blob/main/readme.md>)
// ----------------------------------------------------------------------------
// sub-modules
//
// utility
pub mod utility;
//
// numvec
pub mod numvec;
//
// ad
pub mod ad;
//
// tape
pub mod tape;
//
// adfn
pub mod adfn;
//
// atom
pub mod atom;
//
// dll_lib
pub mod dll_lib;
//
// vec_set
pub(crate) mod vec_set;
//
// op
pub(crate) mod op;
//
// setup
pub(crate) mod setup;
//
// ----------------------------------------------------------------------------
// use
// https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html
// ---------------------------------------------------------------------------
// re-export
//
pub use numvec::{
    NumVec,
};
pub use ad::{
    AD,
    ADType,
    ad_from_value,
    ad_from_vector,
    ad_to_vector,
    doc_generic_v,
};
pub use adfn::{
    ADfn,
    doc_generic_e,
};
pub use tape::{
    IndexT,
    start_recording,
    start_recording_both,
    stop_recording,
};
pub use atom::{
    register_atom,
    call_atom,
    AtomEval,
};
pub use dll_lib::{
    get_lib,
    RustSrcFn,
    get_rust_src_fn,
};
//
pub use op::info::{
    OpInfo,
};
// ---------------------------------------------------------------------------
// Sealed Traits
//
// ThisThreadTapePublic
/// This is the public interface to a sealed trait
pub trait ThisThreadTapePublic : tape::sealed::ThisThreadTape
{ }
impl<V> ThisThreadTapePublic for V
where
    V : tape::sealed::ThisThreadTape ,
{ }
//
// AtomEvalVec
/// This is the public interface to a sealed trait
pub trait AtomEvalVecPublic : atom::sealed::AtomEvalVec
{ }
impl<V> AtomEvalVecPublic for V
where
    V : atom::sealed::AtomEvalVec ,
{ }
// ----------------------------------------------------------------------------
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.11.1");
/// ```
pub const YEAR_MONTH_DAY : &str = "2025.11.1";
