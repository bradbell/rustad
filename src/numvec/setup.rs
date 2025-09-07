// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This module does the necessary setup for the possible value types.
//!
//! Link to [parent module](super)
//!
//! TODO: Enable this setup from an external crate.
// ----------------------------------------------------------------------------
//
// use
 use std::sync::LazyLock;
 use std::sync::RwLock;
 use std::thread::LocalKey;
 use std::cell::RefCell;
 use crate::numvec::ad::AD;
 use crate::numvec::NumVec;
 ///
 /// Set up rustad to do calculations with value type V; see
 /// [doc_generic_v](crate::numvec::doc_generic_v) .
 ///
 /// This macro must be executed once for any type *V*  where `AD<V>` is used.
 /// The rustad package automatically executes this macro
 /// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
 ///
 /// This macro requires the following use statements:
 /// ```text
 ///     use std::sync::LazyLock;
 ///     use std::sync::RwLock;
 ///     use std::thread::LocalKey;
 ///     use std::cell::RefCell;
 ///     use crate::numvec::ad::AD;
 /// ```
 macro_rules! setup_this_value_type{ ($V:ty) => {
         crate::numvec::tape::impl_this_thread_tape!($V);
         crate::numvec::op::info::impl_global_op_info_vec!($V);
         crate::numvec::ad::impl_value_op_ad!($V);
         crate::numvec::atom::impl_atom_eval_vec!($V);
 } }
 //
 setup_this_value_type!(f32);
 setup_this_value_type!(f64);
 setup_this_value_type!( NumVec<f32> );
 setup_this_value_type!( NumVec<f64> );
