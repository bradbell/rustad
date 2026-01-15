// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ----------------------------------------------------------------------------
//
//! A rust Automatic Differentiation library
//!
//! This library is Under Construction and its API is not stable; see its
//! [readme file](<https://github.com/bradbell/rustad/blob/main/readme.md>)
// ----------------------------------------------------------------------------
// public sub-modules
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
// checkpoint
pub mod checkpoint;
//
// dll_lib
pub mod dll_lib;
//
// float
pub mod float;
//
// ----------------------------------------------------------------------------
// private sub-modules
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
    CompareAsLeft,
    CompareAsRight,
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
    start_recording,
    stop_recording,
};
pub use atom::{
    register_atom,
    call_atom,
    AtomCallback,
};
pub use checkpoint::{
    Direction,
    register_checkpoint,
    call_checkpoint,
};
pub use dll_lib::{
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
};
pub use float::{
    core::FloatCore,
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
// GlobalAtomCallbackVecPublic
/// This is the public interface to a sealed trait
pub trait GlobalAtomCallbackVecPublic : atom::sealed::GlobalAtomCallbackVec
{ }
impl<V> GlobalAtomCallbackVecPublic for V
where
    V : atom::sealed::GlobalAtomCallbackVec ,
{ }
//
// GlobalCheckpointInfoVecPublic
/// This is the public interface to a sealed trait
pub trait GlobalCheckpointInfoVecPublic : checkpoint::sealed::GlobalCheckpointInfoVec
{ }
impl<V> GlobalCheckpointInfoVecPublic for V
where
    V : checkpoint::sealed::GlobalCheckpointInfoVec ,
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
/// assert_eq!(date, "2026.1.15");
/// ```
pub const YEAR_MONTH_DAY : &str = "2026.1.15";
//
// AZ_FLOAT_SRC
/// is the source code for the [AzFloat] class.
/// This is needed at the beginning of a dll library that include
/// [ADfn::rust_src] .
pub const AZ_FLOAT_SRC : &str = include_str!( "az_float.rs" );
