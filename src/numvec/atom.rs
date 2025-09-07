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
use crate::numvec::AtomEvalVecPublic;
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
pub type Callback<V> = fn(
    _cache         : &mut Vec<V> ,
    _vec_in        : Vec<V>      ,
    _trace         : bool        ,
    _call_info     : usize       ,
) -> Vec<V> ;
//
// Sparsity
/// Atomic function dependency calculations.
///
/// see [ADfn::sub_sparsity] or [ADfn::for_sparsity]
pub type Sparsity = fn(
    _trace       : bool   ,
    _call_info   : IndexT ,
)-> Vec< [usize; 2] >;
//
// AtomEval
/// Functions that evaluate an atomic function.
pub struct AtomEval<V> {
    // forward_zero
    /// forward_zero Callback parameters
    /// * cache  : see [forward_zero cache](doc_forward_zero#cache]
    /// * vec_in : see [forward_zero domain_zero](doc_forward_zero#domain_zero)
    /// * return : see [forward_zero range_zero](doc_forward_zero#range_zero)
    pub  forward_zero : Callback::<V> ,
    //
    // forward_one
    /// forward_zero Callback parameters
    /// * cache  : see [forward_one cache](doc_forward_one#cache]
    /// * vec_in : see [forward_one domain_one](doc_forward_one#domain_one)
    /// * return : see [forward_one range_one](doc_forward_one#range_one)
    pub  forward_one  : Callback::<V> ,
    //
    // reverse_one
    /// reverse_one Callback parameters
    /// * cache  : see [reverse_one cache](doc_reverse_one#cache]
    /// * vec_in : see [reverse_one range_one](doc_reverse_one#range_one)
    /// * return : see [reverse_one domain_one](doc_forward_one#domain_one)
    pub  reverse_one  : Callback::<V> ,
    //
    // sparsity
    /// see [ADfn::sub_sparsity] or [ADfn::for_sparsity]
    pub sparsity      : Sparsity      ,
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
// ----------------------------------------------------------------------------
// register_atom
/// Register an atomic function.
///
/// ```text
///     atom_index = register_atom(atom_eval)
/// ```
///
/// * atom_eval :
/// contains references to the callback functions that compute
/// values for this atomic function.
///
/// *atom_index :
/// is the index that is used to identify this atomic function.
///
pub fn register_atom<V>( atom_eval : AtomEval<V> ) -> usize
where
    V : AtomEvalVecPublic ,
{   //
    // rwlock
    let rw_lock    = <V as sealed::AtomEvalVec>::get();
    //
    // atom_index
    let atom_index : usize;
    {   //
        // write_lock
        let write_lock = rw_lock.write();
        assert!( write_lock.is_ok() );
        //
        // Rest of this block has a lock, so it has to be fast and can't fail.
        let mut atom_eval_vec = write_lock.unwrap();
        atom_index            = atom_eval_vec.len();
        atom_eval_vec.push( atom_eval );
    }
    atom_index
}
