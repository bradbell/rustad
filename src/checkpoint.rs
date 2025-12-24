// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the checkpoint utilities.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// use
use std::sync::RwLock;
//
use crate::{
    IndexT,
    ADfn,
    AtomCallback,
    register_atom,
};
//
use sealed::GlobalCheckpointVec;
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::atom::sealed::GlobalAtomCallbackVec;
//
#[cfg(doc)]
use crate::doc_generic_v;
// ---------------------------------------------------------------------------
// TODO: Change to pub(crate) after general purpose examples/checkpoint.rs
// code moves to this file.
pub mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use std::sync::RwLock;
    //
    use crate::ADfn;
    //
    #[cfg(doc)]
    use crate::doc_generic_v;
    //
    // GlobalCheckpointVec
    pub trait GlobalCheckpointVec
    where
        Self : Sized + 'static,
    {   /// Returns a reference to the map from checkpoint_id to ADfn objects.
        ///
        /// ```text
        ///     let rw_lock = GlobalCheckpointVec::get();
        /// ```
        ///
        /// * Self : must be a value type V in [doc_generic_v]
        ///
        /// * rw_lock :
        /// is a read-write lock object [std::sync::RwLock]
        ///
        /// * write_lock :
        /// ``` text
        ///     let write_lock     = rw_lock.write();
        ///     let checkpoint_vec = write_lock.unwrap();
        /// ```
        ///
        /// * read_lock :
        /// ``` text
        ///     let read_lock      = rw_lock.read();
        ///     let checkpoint_vec = read_lock.unwrap();
        /// ```
        ///
        /// * checkpont_vec :
        /// checkpont_vec\[checkpoint_id\] is the [ADfn] corresponding to
        /// checkpoint_id.
        ///
        fn get() -> &'static RwLock< Vec< ADfn<Self> > >;
    }
}
//
// impl_global_checkpoint_vec!
/// Implement the global checkpoint vector for value type V
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
macro_rules! impl_global_checkpoint_vec{ ($V:ty) => {
    #[doc = concat!(
        "The global Checkpoint vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::checkpoint::sealed::GlobalCheckpointVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::ADfn<$V> > > {
            pub(crate) static CHECKPOINT_VEC :
                RwLock< Vec< crate::ADfn<$V> > > =
                    RwLock::new( Vec::new() );
            &CHECKPOINT_VEC
        }
    }
} }
pub(crate) use impl_global_checkpoint_vec;
// ----------------------------------------------------------------------------
// register_checkpoint
/// Move a function object to the global chekpoint vector.
///
/// * Syntax :
/// ```text
///     checkpoint_id = register_checkpoint(ad_fn)
/// ```
///
/// * V : see [doc_generic_v]
///
/// * ad_fn :
/// is the ad_fn that is being moved to the global checkpoint vector.
///
/// * checkpoint_id :
/// is the index that is used to identify this checkpoint function.
///
pub fn register_checkpoint<V>( ad_fn : ADfn<V> ) -> IndexT
where
    V : GlobalCheckpointVec,
{   //
    // rwlock
    let rw_lock : &RwLock< Vec< ADfn<V> > > =
        sealed::GlobalCheckpointVec::get();
    //
    // checkpoint_id
    let checkpoint_id  : IndexT;
    let id_too_large   : bool;
    {   //
        // write_lock
        let write_lock = rw_lock.write();
        assert!( write_lock.is_ok() );
        //
        let mut checkpoint_vec = write_lock.unwrap();
        let id_usize           = checkpoint_vec.len();
        id_too_large           = (IndexT::MAX as usize) < id_usize;
        checkpoint_id          = checkpoint_vec.len() as IndexT;
        checkpoint_vec.push( ad_fn );
    }
    assert!( ! id_too_large );
    checkpoint_id
}
// ----------------------------------------------------------------------------
// Value Routines
// -------------------------------------------------------------------------
//
// checkpoint_forward_fun_value
fn checkpoint_forward_fun_value<V>(
    _use_range       : &[bool]      ,
    domain           : &[&V]        ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointVec,
{   //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // checkpoint_id
    let checkpoint_id = call_info as usize;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointVec::get();
    let read_lock         = rw_lock.read();
    let checkpoint_vec    = read_lock.unwrap();
    let ad_fn : &ADfn<V>  = &checkpoint_vec[checkpoint_id];
    //
    // range
    let (range, _)        = ad_fn.forward_zero_value( domain_clone, trace );
    Ok( range )
}
//
// checkpoint_forward_der_value
fn checkpoint_forward_der_value<V>(
    _use_range       : &[bool]     ,
    domain           : &[&V]       ,
    domain_der       : &[&V]       ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointVec,
{   //
    assert_eq!( domain.len(), domain_der.len() );
    //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // domain_der_clone
    let mut domain_der_clone : Vec<V> = Vec::with_capacity( domain_der.len() );
    for j in 0 .. domain_der.len() {
        domain_der_clone.push( (*domain_der[j]).clone() );
    }
    //
    // checkpoint_id
    let checkpoint_id = call_info as usize;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointVec::get();
    let read_lock         = rw_lock.read();
    let checkpoint_vec    = read_lock.unwrap();
    let ad_fn : &ADfn<V>  = &checkpoint_vec[checkpoint_id];
    //
    // range_der
    let (_, var_both)     = ad_fn.forward_zero_value(domain_clone, trace);
    let range_der         = ad_fn.forward_one_value(
        &var_both, domain_der_clone, trace
    );
    Ok( range_der )
}
//
// checkpoint_reverse_der_value
fn checkpoint_reverse_der_value<V>(
    domain           : &[&V]       ,
    range_der        : Vec<&V>     ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointVec,
{   //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // range_der_clone
    let mut range_der_clone : Vec<V> = Vec::with_capacity( range_der.len() );
    for j in 0 .. range_der.len() {
        range_der_clone.push( (*range_der[j]).clone() );
    }
    //
    // checkpoint_id
    let checkpoint_id = call_info as usize;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointVec::get();
    let read_lock         = rw_lock.read();
    let checkpoint_vec    = read_lock.unwrap();
    let ad_fn : &ADfn<V>  = &checkpoint_vec[checkpoint_id];
    //
    // domain_der
    let (_, var_both)     = ad_fn.forward_zero_value(domain_clone, trace);
    let domain_der        = ad_fn.reverse_one_value(
        &var_both, range_der_clone, trace
    );
    Ok( domain_der )
}
//
// checkpoint_rev_depend
fn checkpoint_rev_depend<V>(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    _n_dom       : usize           ,
    call_info    : IndexT          ,
    trace        : bool            ,
) -> String
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointVec + GlobalAtomCallbackVec,
{
    assert_eq!( depend.len(), 0 );
    //
    // compute_dyp
    let compute_dyp = false;
    //
    // checkpoint_id
    let checkpoint_id = call_info as usize;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointVec::get();
    let read_lock         = rw_lock.read();
    let checkpoint_vec    = read_lock.unwrap();
    let ad_fn : &ADfn<V>  = &checkpoint_vec[checkpoint_id];
    //
    // pattern
    // TODO: store the sparsity pattern in a static structure for this
    // checkpoint function so do not need to recompute. Also sort it so it
    // and store point to beginning of each row so depend computes faster.
    let (_, pattern)    = ad_fn.sub_sparsity(trace, compute_dyp);
    //
    // depend
    for [i, j] in pattern.iter() {
        if *i == rng_index {
            depend.push( *j );
        }
    }
    let error_msg = String::from("");
    error_msg
}
// -------------------------------------------------------------------------
// AD routines
// -------------------------------------------------------------------------
//
// -------------------------------------------------------------------------
// register_checkpoint_atom
// -------------------------------------------------------------------------
// TODO: make this private
pub fn register_checkpoint_atom<V>()-> IndexT
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointVec + GlobalAtomCallbackVec,
{
    //
    // checkpoint_callback
    let checkpoint_callback = AtomCallback {
        name                 : &"checkpoint",
        rev_depend           :  Some( checkpoint_rev_depend::<V> ),
        //
        forward_fun_value    :  Some( checkpoint_forward_fun_value::<V> ),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  Some( checkpoint_forward_der_value::<V> ),
        forward_der_ad       :  None,
        //
        reverse_der_value    :  Some( checkpoint_reverse_der_value::<V> ),
        reverse_der_ad       :  None,
    };
    //
    // atom_id
    let atom_id = register_atom( checkpoint_callback );
    atom_id
}
