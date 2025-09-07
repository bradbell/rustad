// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This module implements AD atomic functions
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::numvec::IndexT;
//
#[cfg(doc)]
use crate::numvec::{
        doc_generic_v,
        ADfn,
};
#[cfg(doc)]
use crate::numvec::adfn::{
    forward_zero::doc_forward_zero,
    forward_one::doc_forward_one,
    reverse_one::doc_reverse_one,
};
// ---------------------------------------------------------------------------
//
/// Atomic function evaluation type.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * callback_type = "forward_zero"
///     * cache  : see [forward_zero cache](doc_forward_zero#cache]
///     * vec_in : see [forward_zero domain_zero](doc_forward_zero#domain_zero)
///     * return : see [forward_zero range_zero](doc_forward_zero#range_zero)
///
/// * callback_type = "forward_one"
///     * cache  : see [forward_one cache](doc_forward_one#cache]
///     * vec_in : see [forward_one domain_one](doc_forward_one#domain_one)
///     * return : see [forward_one range_one](doc_forward_one#range_one)
///
/// * callback_type = "reverse_one"
///     * cache  : see [reverse_one cache](doc_reverse_one#cache]
///     * vec_in : see [reverse_one range_one](doc_reverse_one#range_one)
///     * return : see [reverse_one domain_one](doc_forward_one#domain_one)
///
pub type Callback<V> = fn(
    _cache         : &mut Vec<V> ,
    _vec_in        : Vec<V>      ,
    _trace         : bool        ,
    _callback_type : &str        ,
    _call_info     : usize       ,
) -> Vec<V> ;
//
// Sparsity
/// Atomic function dependency calculations.
///
/// see [ADfn::sub_sparsity] or [ADfn::for_sparsity]
pub type Sparsity = fn( _call_info : IndexT )-> Vec< [usize; 2] >;
//
// AtomEval
/// Functions necessary to evaluation one atomic function.
///
/// TODO: make fields in this struct private (once they are used)
pub struct AtomEval<V> {
        pub callback : Callback::<V> ,
        pub sparsity : Sparsity      ,
}
// ----------------------------------------------------------------------------
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use std::sync::RwLock;
    use super::AtomEval;
    //
    // AtomEvalVec
    pub trait AtomEvalVec
    where
        Self : Sized + 'static,
    {   fn get() -> &'static RwLock< Vec< AtomEval<Self> > >;
    }
}
//
// impl_atom_eval_vec!
/// Implement the atomic evaluation vector for value type V
///
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD<V>` is used. The rustad package automatically executes it
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::sync::RwLock;
/// ```
macro_rules! impl_atom_eval_vec{ ($V:ty) => {
    #[doc = concat!(
        "The atomic evaluation vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::numvec::atom::sealed::AtomEvalVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::numvec::atom::AtomEval<$V> > > {
            pub(crate) static ATOM_EVAL_VEC :
            RwLock< Vec< crate::numvec::atom::AtomEval<$V> > > =
                RwLock::new( Vec::new() );
            &ATOM_EVAL_VEC
        }
    }
} }
pub(crate) use impl_atom_eval_vec;
