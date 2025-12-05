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
// az_float
pub mod az_float;
//
// num_vec
pub mod num_vec;
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
pub use az_float::{
    AzFloat,
};
pub use num_vec::{
    NumVec,
};
pub use ad::{
    AD,
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
    start_recording_var,
    start_recording_dyp_var,
    stop_recording,
};
pub use atom::{
    register_atom,
    call_atom,
    AtomCallback,
};
pub use dll_lib::{
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
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
// AtomInfoVecPublic
/// This is the public interface to a sealed trait
pub trait AtomInfoVecPublic : atom::sealed::AtomInfoVec
{ }
impl<V> AtomInfoVecPublic for V
where
    V : atom::sealed::AtomInfoVec ,
{ }
//
//
// GlobalOpInfoVecPublic
/// This is the public interface to a sealed trait
pub trait GlobalOpInfoVecPublic : op::info::sealed::GlobalOpInfoVec
{ }
impl<V> GlobalOpInfoVecPublic for V
where
    V : op::info::sealed::GlobalOpInfoVec ,
{ }
//
// ----------------------------------------------------------------------------
// Constants
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.12.5");
/// ```
pub const YEAR_MONTH_DAY : &str = "2025.12.5";
//
// AZ_FLOAT_SRC
/// is the source code for the [AzFloat] class.
/// This is needed at the beginning of a dll library that include
/// [ADfn::rust_src] .
pub const AZ_FLOAT_SRC : &str = include_str!( "az_float.rs" );
