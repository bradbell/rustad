// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This module implements AD atomic functions
//!
//! Then are called atomic functions because they are recorded as a
//! single operation in tapes and ADfn objects.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use std::sync::RwLock;
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::numvec::op::id::CALL_OP;
use crate::numvec::op::id::CALL_RES_OP;
use crate::numvec::tape::Tape;
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::{
    IndexT,
    AD,
    ad_from_vector,
    AtomEvalVecPublic,
    ThisThreadTapePublic,
};
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
// Callback
/// Atomic function evaluation type.
///
pub type Callback<V> = fn(
    _var_zero      : &mut Vec<V> ,
    _vec_in        : &Vec<&V>    ,
    _trace         : bool        ,
    _call_info     : IndexT      ,
) -> Vec<V> ;
//
// ForwardDepend
/// Atomic function forward dependency type.
///
/// * is_var_domain :
/// This has the same length as *adomain* in the corresponding [call_atom].
/// The j-th component is true (false) if the j-th component
/// of *adomain* is a variable (constant).
///
/// * return :
/// This has the same length as *arange* in the corresponding [call_atom].
/// The i-th component is true (false) if the i-th component
/// of *arange* depends on a variable in *adomaion.
///
pub type ForwardDepend = fn(
    _is_var_domain  : &Vec<bool> ,
    _trace          : bool       ,
    _call_info      : IndexT     ,
)-> Vec<bool>;
//
// AtomEval
/// Functions that evaluate an atomic function.
pub struct AtomEval<V> {
    // forward_zero_value
    /// Callback function used during [ADfn::forward_zero_value]
    ///
    /// *Syntax*
    /// ```text
    ///     range_zero = forward_zero_value(
    ///         &mut var_zero, &domain_zero, trace, call_info
    ///     )
    /// ```
    ///
    /// * var_zero
    /// This vector will have size zero on input.
    /// It can be used to cache information for use by forward_one
    /// and reverse one (and has no other restrictions).
    ///
    /// * domain_zero :
    /// this contains the value of the atomic function domain variables
    /// ( called *vec_in* in [Callback] ).
    ///
    /// * trace :
    /// if true, a trace of the calculations may be printed on stdout.
    ///
    /// * call_info :
    /// is the *call_info* value used when the atomic function was called.
    ///
    /// * range_zero :
    /// this contains the value of the atomic function range variables.
    ///
    pub  forward_zero_value : Callback::<V> ,
    //
    // forward_one_value
    /// Callback function used during [ADfn::forward_one_value]
    ///
    /// *Syntax*
    /// ```text
    ///     range_one = forward_zero_value(
    ///         &mut var_zero, &domain_one, trace, call_info
    ///     )
    /// ```
    ///
    /// * var_zero :
    /// This will contain the values set by forward_zero for the
    /// same call to this atomic function; i.e., same [call_atom].
    ///
    /// * domain_one :
    /// this contains the direction for the directional derivative
    /// ( called *vec_in* in [Callback] ).
    ///
    /// * trace :
    /// if true, a trace of the calculations may be printed on stdout.
    ///
    /// * call_info :
    /// is the *call_info* value used when the atomic function was called.
    ///
    /// * range_one :
    /// Let *domain_zero* be its value in the call to forward_zero
    /// that set *var_zero* . The return value is
    /// ```text
    ///     range_one = f'(domain_zero) * domain_one
    /// ```
    pub  forward_one_value  : Callback::<V> ,
    //
    // reverse_one_value
    /// Callback function used during [ADfn::reverse_one_value]
    ///
    /// *Syntax*
    /// ```text
    ///     domain_one = forward_zero_value(
    ///         &mut var_zero, &range_one, trace, call_info
    ///     )
    /// ```
    ///
    /// * var_zero :
    /// This will contain the values set by forward_zero for the
    /// same call to this atomic function; i.e., same [call_atom].
    ///
    /// * range_one :
    /// this contains the function weights for the partial derivatives
    /// ( called *vec_in* in [Callback] ).
    ///
    /// * trace :
    /// if true, a trace of the calculations may be printed on stdout.
    ///
    /// * call_info :
    /// is the *call_info* value used when the atomic function was called.
    ///
    /// * domain_one :
    /// Let *domain_zero* be its value in the call to forward_zero
    /// that set *var_zero* . The return value is
    /// ```text
    ///     domain_one = range_one * f'(domain_zero)
    /// ```
    pub  reverse_one_value  : Callback::<V> ,
    //
    // forward_depend
    /// Callback function used during forward_zero to determine
    /// which range components are variables.
    ///
    /// A range component that depends on a variable is also a variable.
    pub forward_depend  : ForwardDepend ,
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
/// * Syntax :
/// ```text
///     atom_id = register_atom(atom_eval)
/// ```
///
/// * V : see [doc_generic_v]
///
/// ## atom_eval :
/// contains references to the callback functions that compute
/// values for this atomic function.
///
/// ## atom_id :
/// is the index that is used to identify this atomic function.
///
pub fn register_atom<V>( atom_eval : AtomEval<V> ) -> IndexT
where
    V : AtomEvalVecPublic ,
{   //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomEval<V> > > = sealed::AtomEvalVec::get();
    //
    // atom_id
    let atom_id           : IndexT;
    let atom_id_too_large : bool;
    {   //
        // write_lock
        let write_lock = rw_lock.write();
        assert!( write_lock.is_ok() );
        //
        // Rest of this block has a lock, so it has to be fast and can't fail.
        let mut atom_eval_vec = write_lock.unwrap();
        let atom_id_usize     = atom_eval_vec.len();
        atom_id_too_large     = (IndexT::MAX as usize) < atom_id_usize;
        atom_id               = atom_eval_vec.len() as IndexT;
        atom_eval_vec.push( atom_eval );
    }
    assert!( ! atom_id_too_large );
    atom_id
}
// ----------------------------------------------------------------------------
// record_call_atom
fn record_call_atom<V>(
    tape             : &mut Tape<V>                  ,
    forward_depend   : ForwardDepend                 ,
    range_zero       : Vec<V>                        ,
    atom_id          : IndexT                        ,
    call_info        : IndexT                        ,
    adomain          : Vec< AD<V> >                  ,
    trace            : bool                          ,
) -> Vec< AD<V> >
where
    V : Clone ,
{   //
    // tape.recordng
    debug_assert!( tape.recording );
    //
    // call_n_arg, call_n_res
    let call_n_arg = adomain.len();
    let call_n_res = range_zero.len();
    //
    // arange
    let mut arange : Vec< AD<V> > = ad_from_vector(range_zero);
    //
    // is_var_arg
    let is_var_arg : Vec<bool> = adomain.iter().map(
        |adomain_j| (*adomain_j).tape_id == tape.tape_id
    ).collect();
    //
    // is_var_res
    let is_var_res = forward_depend(&is_var_arg, trace, call_info);
    //
    // arange, n_var_res
    let mut n_var_res = 0;
    for i in 0 .. call_n_res {
        if is_var_res[i] {
            arange[i].tape_id   = tape.tape_id;
            arange[i].var_index = tape.n_var + n_var_res;
            n_var_res += 1;
        }
    }
    if n_var_res > 0 {
        //
        // tape.id_all, tape.op2arg
        tape.id_all.push( CALL_OP );
        tape.op2arg.push( tape.arg_all.len() as IndexT );
        //
        // tape.arg_all, tape.con_all
        tape.arg_all.push( atom_id );                        // arg[0]
        tape.arg_all.push( call_info );                      // arg[1]
        tape.arg_all.push( call_n_arg as IndexT );           // arg[2]
        tape.arg_all.push( call_n_res as IndexT );           // arg[3]
        tape.arg_all.push( tape.flag_all.len() as IndexT );  // arg[4]
        //
        // tape.arg_all
        for j in 0 .. call_n_arg {
            let index = if is_var_arg[j] {
                adomain[j].var_index
            } else {
                let con_index = tape.con_all.len();
                tape.con_all.push( adomain[j].value.clone() );
                con_index
            };
            tape.arg_all.push( index as IndexT );            // arg[5+j]
        }
        //
        // tape.flag_all
        tape.flag_all.push( trace );               // flag[ arg[4] ]
        for j in 0 .. call_n_arg {
            tape.flag_all.push( is_var_arg[j] );   // flag[ arg[4] + j + 1]
        }
        for i in 0 .. call_n_res {
            tape.flag_all.push( is_var_res[i] );   // flag[ arg[4] + n_res + i]
        }
        //
        // tape.n_var
        tape.n_var += n_var_res;
        //
        // tape.id_all, tape.op2arg
        for _i in 0 .. (n_var_res - 1) {
            tape.id_all.push( CALL_RES_OP );
            tape.op2arg.push( tape.arg_all.len() as IndexT );
        }
    }
    arange
}
// ----------------------------------------------------------------------------
// call_atom
/// Make an AD call to an atomic function.
///
/// Compute the result of an atomic function and,
/// if this thread is currently recording, include the call in its tape.
///
/// * Syntax :
/// ```text
///     arange = call_atom_ad(atom_id, call_info, adomain, trace)
/// ```
///
/// * V : see [doc_generic_v]
/// `
/// * atom_id :
/// The [atom_id](register_atom#atom_id) returned by register_atom for this
/// atomic function.
///
/// * call_info :
/// This is information about this call that is be passed on to the
/// callback functions specified by [atom_eval](register_atom#atom_eval).
///
/// * adomain :
/// This is the value of the arguments to the atomic function.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
/// This may be useful for debugging atomic functions.
pub fn call_atom<V>(
    atom_id     : IndexT       ,
    call_info   : IndexT       ,
    adomain     : Vec< AD<V> > ,
    trace       : bool         ,
) -> Vec< AD<V> >
where
    V   : Clone + From<f32> + ThisThreadTapePublic + AtomEvalVecPublic ,
{
    //
    // local_key
    let local_key : &LocalKey< RefCell< Tape<V> > > = ThisThreadTape::get();
    //
    // recording
    let recording : bool = local_key.with_borrow( |tape| tape.recording );
    //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomEval<V> > > = sealed::AtomEvalVec::get();
    //
    // forward_zero, forward_depend
    let forward_zero : Callback<V>;
    let forward_depend : ForwardDepend;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval     = &atom_eval_vec[atom_id as usize];
        forward_zero      = atom_eval.forward_zero_value.clone();
        forward_depend    = atom_eval.forward_depend.clone();
    }
    //
    // domain_zero
    let mut domain_zero : Vec<&V> = Vec::new();
    for j in 0 .. adomain.len() {
        domain_zero.push( &adomain[j].value );
    }
    //
    // range_zero
    // restore domain_zero using var_zero.
    let mut var_zero : Vec<V> = Vec::new();
    let range_zero  = forward_zero(
        &mut var_zero, &domain_zero, trace, call_info
    );
    //
    // arange
    let arange : Vec< AD<V> >;
    if ! recording {
        arange = ad_from_vector(range_zero);
    } else {
        arange = local_key.with_borrow_mut( |tape| record_call_atom::<V>(
            tape,
            forward_depend,
            range_zero,
            atom_id,
            call_info,
            adomain,
            trace,
        ) );
    }
    arange
}
