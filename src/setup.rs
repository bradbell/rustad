// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub(crate) module does setup for the possible value types.
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
use crate::{
    AD,
    NumVec,
    AzFloat,
};
///
/// Set up rustad to do calculations with value type V; see
/// [doc_generic_v](crate::doc_generic_v) .
///
/// This macro must be executed once for any type *V*  where `AD<V>` is used.
/// The rustad package automatically executes this macro
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
///
/// ```text
///     use std::sync::LazyLock;
///     use std::sync::RwLock;
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
///     use crate::ad::AD;
/// ```
macro_rules! setup_this_value_type{ ($V:ty) => {
        crate::tape::impl_this_thread_tape!($V);
        crate::ad::impl_value_op_ad!($V);
        crate::ad::impl_ad_from_f32!($V);
        crate::atom::impl_global_atom_callback_vec!($V);
        crate::checkpoint::impl_global_checkpoint_vec!($V);
        crate::op::info::impl_global_op_info_vec!($V);
} }
//
// AzFloat value types
setup_this_value_type!( AzFloat<f32> );
setup_this_value_type!( AzFloat<f64> );
crate::ad::impl_ad_from_f64!( AzFloat<f64> );
//
// NumVec<AzFloat> value types
setup_this_value_type!( NumVec< AzFloat<f32> > );
setup_this_value_type!( NumVec< AzFloat<f64> > );
crate::ad::impl_ad_from_f64!( NumVec< AzFloat<f64> > );
//
// Float value types
// setup_this_value_type!(f32);
// setup_this_value_type!(f64);
// crate::ad::impl_ad_from_f64!(f64);
//
// NumVec<Float> value types
// setup_this_value_type!( NumVec<f32> );
// setup_this_value_type!( NumVec<f64> );
// crate::ad::impl_ad_from_f64!( NumVec<f64> );
