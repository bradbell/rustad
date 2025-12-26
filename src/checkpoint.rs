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
    AD,
    IndexT,
    ADfn,
    AtomCallback,
    register_atom,
};
//
use sealed::{
    CheckpointInfo,
    GlobalCheckpointInfoVec,
};
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::atom::sealed::GlobalAtomCallbackVec;
use crate::atom::call_atom;
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::doc_generic_v;
// ---------------------------------------------------------------------------
// ref_slice2vec
fn ref_slice2vec<E>(ref_slice : &[&E]) -> Vec<E>
where
    E : Clone,
{
    let n                 = ref_slice.len();
    let mut vec  : Vec<E> = Vec::with_capacity(n);
    for i in 0 .. n {
        vec.push( (*ref_slice[i]).clone() );
    }
    vec
}
// ---------------------------------------------------------------------------
pub(crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use std::sync::RwLock;
    use std::sync::LazyLock;
    //
    use crate::{
        ADfn,
        IndexT,
    };
    //
    #[cfg(doc)]
    use crate::doc_generic_v;
    #[cfg(doc)]
    use super::register_checkpoint;
    // ------------------------------------------------------------------------
    // CheckpointInfo
    /// Information for one checkpoint function.
    ///
    /// TODO: change this from pub to pub(crate)
    pub struct CheckpointInfo<V> {
        /// The function object used for value evaluations and AD evlaution of
        /// the function.
        pub ad_fn         : ADfn<V>        ,
        /// The checkpoint_id for AD evaluation of forward mode derivatives.
        pub ad_forward_id : Option<IndexT> ,
        /// The checkpoint_id for AD evaluation of reverse mode derivatives.
        pub ad_reverse_id: Option<IndexT>  ,
    }
    impl<V> CheckpointInfo<V> {
        pub(crate) fn new(
            new_ad_fn         : ADfn<V>         ,
            new_ad_forward_id : Option<IndexT>  ,
            new_ad_reverse_id : Option<IndexT> ,
    )-> Self {
            Self{
                ad_fn         : new_ad_fn         ,
                ad_forward_id : new_ad_forward_id ,
                ad_reverse_id : new_ad_reverse_id ,
            }
        }
    }
    //
    // GlobalCheckpointInfoVec
    pub trait GlobalCheckpointInfoVec
    where
        Self : Sized + 'static,
    {   /// Returns a reference to map from checkpoint_id to CheckpointInfo.
        ///
        /// ```text
        ///     let rw_lock  = GlobalCheckpointInfoVec::get();
        ///     let atom_id  = **GlobalCheckpointInfoVec::atom_id();
        /// ```
        ///
        /// * Self : must be a value type V in [doc_generic_v]
        ///
        /// * atom_id:
        /// is the [atom_id](crate::atom::register_atom) used to evaluate
        /// checkpoint functions.
        ///
        /// * rw_lock :
        /// is a read-write lock object [std::sync::RwLock]
        ///
        /// * write_lock :
        /// ``` text
        ///     let write_lock = rw_lock.write();
        ///     let mut info_vec : &Vec< CheckpointInfo<V> >
        ///         = &*write_lock.unwrap();
        /// ```
        ///
        /// * read_lock :
        /// ``` text
        ///     let read_lock  = rw_lock.read();
        ///     let info_vec : &Vec< CheckpointINfo<V> >
        ///         = &*read_lock.unwrap();
        /// ```
        ///
        /// * info_vec :
        /// info_vec\[checkpoint_id\] is the [CheckpointInfo' corresponding to
        /// checkpoint_id; see [register_checkpoint] .
        ///
        fn get() -> &'static RwLock< Vec< CheckpointInfo<Self> > >;
        fn atom_id()-> &'static LazyLock<IndexT>;
    }
}
//
// impl_global_checkpoint_info!
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
macro_rules! impl_global_checkpoint_info{ ($V:ty) => {
    #[doc = concat!(
        "The global Checkpoint vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::checkpoint::sealed::GlobalCheckpointInfoVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::checkpoint::sealed::CheckpointInfo<$V> > > {
            pub(crate) static CHECKPOINT_VEC :
                RwLock< Vec< crate::checkpoint::sealed::CheckpointInfo<$V> > > =
                    RwLock::new( Vec::new() );
            &CHECKPOINT_VEC
        }
        fn atom_id() -> &'static LazyLock<crate::IndexT> {
            pub static ATOM_ID : LazyLock<crate::IndexT> = LazyLock::new(
                || crate::checkpoint::register_checkpoint_atom::<$V>()
            );
            &ATOM_ID
        }



    }
} }
pub(crate) use impl_global_checkpoint_info;
// -------------------------------------------------------------------------
pub(crate) fn register_checkpoint_atom<V>()-> IndexT
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + GlobalAtomCallbackVec,
    V : ThisThreadTape,
{
    //
    // checkpoint_callback
    let checkpoint_callback = AtomCallback {
        name                 : &"checkpoint",
        rev_depend           :  Some( checkpoint_rev_depend::<V> ),
        //
        forward_fun_value    :  Some( checkpoint_forward_fun_value::<V> ),
        forward_fun_ad       :  Some( checkpoint_forward_fun_ad::<V>    ),
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
//
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
/// ## checkpoint_id :
/// is the index that is used to identify this checkpoint function.
///
pub fn register_checkpoint<V>( ad_fn : ADfn<V> ) -> IndexT
where
    V : GlobalCheckpointInfoVec,
{   //
    if 0 < ad_fn.dyp_len() { panic!(
        "register_checkpoint: 0 > ad_fun.dyp_len()"
    ); }
    // rwlock
    let rw_lock : &RwLock< Vec< CheckpointInfo<V> > > =
        sealed::GlobalCheckpointInfoVec::get();
    //
    // checkpoint_id
    let checkpoint_id  : IndexT;
    let id_too_large   : bool;
    {   //
        // write_lock
        let write_lock = rw_lock.write();
        assert!( write_lock.is_ok() );
        //
        let mut info_vec = write_lock.unwrap();
        let id_usize           = info_vec.len();
        id_too_large           = (IndexT::MAX as usize) < id_usize;
        checkpoint_id          = info_vec.len() as IndexT;
        let info               = CheckpointInfo::new( ad_fn, None, None );
        info_vec.push( info );
    }
    assert!( ! id_too_large );
    checkpoint_id
}
//
// call_checkpoint
/// Make an AD call to a checkpoint function.
///
/// Compute the result of a checkpoint function and,
/// if this thread is currently recording, include the call in its tape.
///
/// * adomain :
/// This is the value of the arguments for this atomic function call.
/// Note that the dimension of the domain only depends on checkpoint_id.
///
/// * checkpoint_id :
/// The [checkpoint_id](register_checkpoint#checkpoint_id)
/// returned by register_checkpoint for this checkpoint function.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
/// This may be useful for debugging.
///
pub fn call_checkpoint<V>(
    adomain       : Vec< AD<V> > ,
    checkpoint_id : IndexT       ,
    trace         : bool         ,
) -> Vec< AD<V> >
where
    V : Clone + From<f32>,
    V : ThisThreadTape + GlobalAtomCallbackVec + GlobalCheckpointInfoVec ,
{   //
    // n_var_dom
    let n_range =
    {   let rw_lock   = GlobalCheckpointInfoVec::get();
        let read_lock  = rw_lock.read();
        let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
        let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
        ad_fn.rng_len()
    };
    //
    // arange
    let atom_id   = **< V as GlobalCheckpointInfoVec>::atom_id();
    let call_info = checkpoint_id;
    let arange    = call_atom(n_range, adomain, atom_id, call_info, trace);
    arange
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec,
{   //
    // domain_clone
    let domain_clone = ref_slice2vec(domain);
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_fn
    let rw_lock   = GlobalCheckpointInfoVec::get();
    let read_lock  = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec,
{   //
    assert_eq!( domain.len(), domain_der.len() );
    //
    // domain_clone
    let domain_clone = ref_slice2vec(domain);
    //
    // domain_der_clone
    let domain_der_clone = ref_slice2vec(domain_der);
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointInfoVec::get();
    let read_lock         = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
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
    range_der        : &[&V]       ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec,
{   //
    // domain_clone
    let domain_clone = ref_slice2vec(domain);
    //
    // range_der_clone
    let range_der_clone = ref_slice2vec(&range_der);
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointInfoVec::get();
    let read_lock         = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + GlobalAtomCallbackVec,
{
    assert_eq!( depend.len(), 0 );
    //
    // compute_dyp
    let compute_dyp = false;
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_fn
    let rw_lock           = GlobalCheckpointInfoVec::get();
    let read_lock         = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
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
// checkpoint_forward_fun_ad
fn checkpoint_forward_fun_ad<V>(
    _use_range       : &[bool]      ,
    adomain          : &[& AD<V> ]  ,
    call_info        : IndexT       ,
    trace            : bool         ,
) -> Result< Vec< AD<V> >, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + GlobalAtomCallbackVec,
    V : ThisThreadTape,
{   //
    // adomain_clone
    let adomain_clone = ref_slice2vec(adomain);
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // n_var_dom
    let n_range =
    {   let rw_lock   = GlobalCheckpointInfoVec::get();
        let read_lock  = rw_lock.read();
        let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
        let ad_fn = &info_vec[checkpoint_id as usize].ad_fn;
            ad_fn.rng_len()
    };
    //
    // arange
    let atom_id   = **< V as GlobalCheckpointInfoVec>::atom_id();
    let call_info = checkpoint_id;
    let arange = call_atom(n_range, adomain_clone, atom_id, call_info, trace);
    Ok(arange)
}
