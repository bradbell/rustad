// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines the checkpoint utilities.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// use
use std::sync::RwLock;
use std::collections::HashMap;
//
use crate::{
    AD,
    IndexT,
    ADfn,
    AtomCallback,
    register_atom,
    start_recording,
    stop_recording,
    FloatCore,
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
// Direction
#[derive(PartialEq)]
pub enum Direction {
    Forward,
    Reverse,
}
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
        /// The user name for this checkpoint function
        pub name         : String         ,
        //
        /// The function object used for value evaluations and AD evlaution of
        /// the function.
        pub ad_fn         : ADfn<V>        ,
        //
        /// The checkpoint_id for AD evaluation of forward mode derivatives.
        pub ad_forward_id : Option<IndexT> ,
        //
        /// The checkpoint_id for AD evaluation of reverse mode derivatives.
        pub ad_reverse_id: Option<IndexT>  ,
    }
    impl<V> CheckpointInfo<V> {
        pub(crate) fn new(
            new_name          : String          ,
            new_ad_fn         : ADfn<V>         ,
            new_ad_forward_id : Option<IndexT>  ,
            new_ad_reverse_id : Option<IndexT> ,
    )-> Self {
            Self{
                name          : new_name          ,
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
        ///   is the [atom_id](crate::atom::register_atom) used to evaluate
        ///   checkpoint functions.
        ///
        /// * rw_lock :
        ///   is a read-write lock object [std::sync::RwLock]
        ///
        /// * write_lock :
        ///   ``` text
        ///     let write_lock = rw_lock.write();
        ///     let mut info_vec : &Vec< CheckpointInfo<V> >
        ///         = &*write_lock.unwrap();
        ///   ```
        ///
        /// * read_lock :
        ///   ``` text
        ///     let read_lock  = rw_lock.read();
        ///     let info_vec : &Vec< CheckpointINfo<V> >
        ///         = &*read_lock.unwrap();
        ///   ```
        ///
        /// * info_vec :
        ///   info_vec\[checkpoint_id\] is the [CheckpointInfo' corresponding to
        ///   checkpoint_id; see [register_checkpoint] .
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
    V : ThisThreadTape + FloatCore,
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
        forward_der_ad       :  Some( checkpoint_forward_der_ad::<V>    ),
        //
        reverse_der_value    :  Some( checkpoint_reverse_der_value::<V> ),
        reverse_der_ad       :  Some( checkpoint_reverse_der_ad::<V>    ),
    };
    //
    // atom_id
    let atom_id = register_atom( checkpoint_callback );
    atom_id
}
//
// register_checkpoint
/// Convert a function object to a checkpoint function.
///
/// A checkpoint function call [call_checkpoint] only puts a call operator
/// in the current tape (instead of all the operations in ad_fn).
/// If a function is used many times, this can greatly reduce
/// the size of the operation sequence.
///
/// * See Also : [register_atom]
///
/// * Syntax :
///   ```text
///     checkpoint_id = register_checkpoint(ad_fn, directions, hash_map)
///   ```
///
/// * V : see [doc_generic_v]
///
/// * ad_fn :
///   is the ad_fn that is being moved to the global checkpoint vector.
///
/// * hash_map :
///   The following is a description of the valid key :
///
///     * name :
///       the value is the name that identifies this checkpoint function
///       (default no_name).
///
///     * trace :
///       the value is true or false (the default is false).
///       If trace is true, and ! directions.is_empty(), the creation
///       of the new [ADfn] objects use for AD derivative evaluation is traced.
///
/// * directions :
///   If directions\[k\] is Forward (Reverse), then (k+1)-th order forward
///   (reverse) AD derivatives of this checkpoint function can be computed.
///   This requires creating new ADfn objects for each element of directions.
///   To be more specific, if direction\[k\] is Forward (Reverse)
///   the (k+1)-th order derivative is computer using
///   the ADfn that is the forward (reverse) derivative of
///   the k-th order derivative.
///   (The zero order derivative corresponds to ad_fn.)
///
///
/// ## checkpoint_id :
/// is the index that is used to identify this checkpoint function.
///
pub fn register_checkpoint<V>(
    ad_fn         : ADfn<V>                 ,
    directions    : &[Direction]            ,
    mut hash_map  : HashMap<&str, String>   ,
) -> IndexT
where
    V : Clone + From<f32> + std::fmt::Display + FloatCore,
    V : ThisThreadTape + GlobalOpInfoVec + GlobalCheckpointInfoVec,
{   //
    if 0 < ad_fn.dyp_len() { panic!(
        "register_checkpoint: 0 > ad_fun.dyp_len()"
    ); }
    //
    // name, trace
    let mut name  = "no_name".to_string();
    let mut trace = false;
    for key in hash_map.keys() { match key {
        &"name" => {
            name = hash_map.get(key).unwrap().clone();
        },
        &"trace" => {
            let value = hash_map.get( "trace").unwrap();
            if value != "true" && value != "false" { panic!(
                "registem_checkpoint: hash_map.get(trace): \
                {} is not a valid value", value
            ); }
            trace = value == "true";
        },
        _ => panic!(
            "registem_checkpoint hash_map : {} is not a valid key", key
        ),
    } }
    //
    // one_v
    let one_v : V = 0f32.into();
    //
    // ad_forward_id, ad_reverse_id
    let mut ad_forward_id : Option<IndexT> = None;
    let mut ad_reverse_id : Option<IndexT> = None;
    if ! directions.is_empty()  {
        let directions_tail = &directions[1 .. directions.len()];
        if directions[0] == Direction::Forward {
            hash_map.insert( "name", name.clone() + ".forward" );
            let nx            = ad_fn.var_dom_len();
            let x_dx          = vec![one_v; 2 * nx ];
            let (_, ax_dx)    = start_recording(None, x_dx);
            let ax            = ax_dx[0 .. nx].to_vec();
            let adx           = ax_dx[nx .. 2*nx].to_vec();
            let (_ay, av)     = ad_fn.forward_var_ad(None, ax, trace);
            let ady           = ad_fn.forward_der_ad(None, &av, adx, trace);
            let ad_fn_for     = stop_recording(ady);
            let checkpoint_id = register_checkpoint(
                ad_fn_for, directions_tail, hash_map
            );
            ad_forward_id     = Some(checkpoint_id);
        } else {
            debug_assert!( directions[0] == Direction::Reverse );
            hash_map.insert( "name", name.clone() + ".reverse" );
            let nx            = ad_fn.var_dom_len();
            let ny            = ad_fn.rng_len();
            let x_dy          = vec![one_v; nx + ny ];
            let (_, ax_dy)    = start_recording(None, x_dy);
            let ax            = ax_dy[0 .. nx].to_vec();
            let ady           = ax_dy[nx .. nx + ny].to_vec();
            let (_ay, av)     = ad_fn.forward_var_ad(None, ax, trace);
            let adx           = ad_fn.reverse_der_ad(None, &av, ady, trace);
            let ad_fn_rev     = stop_recording(adx);
            let checkpoint_id = register_checkpoint(
                ad_fn_rev, directions_tail, hash_map
            );
            ad_reverse_id     = Some(checkpoint_id);
        }
    }
    //
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
        let info               = CheckpointInfo::new(
            name, ad_fn, ad_forward_id, ad_reverse_id
        );
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
///   This is the value of the arguments for this atomic function call.
///   Note that the dimension of the domain only depends on checkpoint_id.
///
/// * checkpoint_id :
///   The [checkpoint_id](register_checkpoint#checkpoint_id)
///   returned by register_checkpoint for this checkpoint function.
///
/// * trace :
///   if true, a trace of the calculations may be printed on stdout.
///   This may be useful for debugging.
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + FloatCore,
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
    let (range, _)        = ad_fn.forward_var_value(None, domain_clone, trace );
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + FloatCore,
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
    let (_, var_both)     = ad_fn.forward_var_value(None, domain_clone, trace);
    let range_der         = ad_fn.forward_der_value(
        None, &var_both, domain_der_clone, trace
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
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + FloatCore,
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
    let (_, var_both)     = ad_fn.forward_var_value(None, domain_clone, trace);
    let domain_der        = ad_fn.reverse_der_value(
        None, &var_both, range_der_clone, trace
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
//
// checkpoint_forward_der_ad
fn checkpoint_forward_der_ad<V>(
    _use_range       : &[bool]     ,
    adomain          : &[& AD<V> ] ,
    adomain_der      : &[& AD<V> ] ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec< AD<V> >, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + ThisThreadTape,
    V : GlobalAtomCallbackVec,
{   //
    assert_eq!( adomain.len(), adomain_der.len() );
    //
    // adomain_both
    let mut adomain_both = ref_slice2vec(adomain);
    let mut adomain_der_clone = ref_slice2vec(adomain_der);
    adomain_both.append( &mut adomain_der_clone );
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_forward_id
    let rw_lock           = GlobalCheckpointInfoVec::get();
    let read_lock         = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_forward_id = &info_vec[checkpoint_id as usize].ad_forward_id;
    let name          = &info_vec[checkpoint_id as usize].name;
    if ad_forward_id.is_none() {
        panic!( "forward_der_ad not requested: checkpoint name = {}", name);
    }
    let ad_forward_id = ad_forward_id.unwrap();
    //
    // arange_der
    let arange_der = call_checkpoint(adomain_both, ad_forward_id, trace);
    //
    Ok( arange_der )
}
//
// checkpoint_reverse_der_ad
fn checkpoint_reverse_der_ad<V>(
    adomain          : &[& AD<V> ] ,
    arange_der       : &[& AD<V> ] ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec< AD<V> >, String >
where
    V : Clone + From<f32> + std::fmt::Display,
    V : GlobalOpInfoVec + GlobalCheckpointInfoVec + ThisThreadTape,
    V : GlobalAtomCallbackVec,
{   //
    // adomain_both
    let mut adomain_both    = ref_slice2vec(adomain);
    let mut arange_der_clone = ref_slice2vec(&arange_der);
    adomain_both.append( &mut arange_der_clone );
    //
    // checkpoint_id
    let checkpoint_id = call_info;
    //
    // rw_lock, ad_reverser_id
    let rw_lock           = GlobalCheckpointInfoVec::get();
    let read_lock         = rw_lock.read();
    let info_vec : &Vec< CheckpointInfo<V> > = &*read_lock.unwrap();
    let ad_reverse_id = &info_vec[checkpoint_id as usize].ad_reverse_id;
    let name          = &info_vec[checkpoint_id as usize].name;
    if ad_reverse_id.is_none() {
        panic!( "reverse_der_ad not requested: checkpoint name = {}", name);
    }
    let ad_reverse_id = ad_reverse_id.unwrap();
    //
    // adomain_der
    let adomain_der = call_checkpoint(adomain_both, ad_reverse_id, trace);
    //
    Ok( adomain_der )
}
