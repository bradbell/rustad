// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This module implements AD atomic functions
//!
//! They are called atomic functions because they are recorded as a
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
use crate::tape::OpSequence;
use crate::op::id::CALL_OP;
use crate::op::id::CALL_RES_OP;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::{
    IndexT,
    AD,
    ADType,
    ad_from_vector,
    AtomEvalVecPublic,
    ThisThreadTapePublic,
};
//
#[cfg(doc)]
use crate::{
        doc_generic_v,
        ADfn,
};
#[cfg(doc)]
use crate::adfn::{
    forward_zero::doc_forward_zero,
    forward_one::doc_forward_one,
    reverse_one::doc_reverse_one,
};
// ---------------------------------------------------------------------------
//
// AtomForwardType
/// Callback to atomic functions to determine ADType of results.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     range_ad_type = atom_forward_type(domain_ad_type, call_info, trace)
/// ```
///
/// forward_ad_type :
/// is the AtomForwardType callback for this atomic function.
///
/// * domain_ad_type :
/// This has the same length as *adomain* in the corresponding [call_atom].
/// The j-th component is the [ADType] of the j-th component
/// of *adomain* .
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * range_ad_type :
/// This has the same length as *arange* in the corresponding [call_atom].
/// The i-th component is the [ADType] of the i-th component
/// of *arange* .
/// Note that if a result depends on two values, the ADType of the result
/// is the maximum of the ADType for the two values.
///
pub type AtomForwardType = fn(
    _domain_ad_type  : &Vec<ADType> ,
    _call_info       : IndexT       ,
    _trace           : bool         ,
)-> Vec<ADType>;
//
// AtomForwardVarValue
/// Callback to atomic functions during [ADfn::forward_zero_value]
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *range_one*
/// contains the value of the atomic function range variables.
///
pub type AtomForwardVarValue<V> = fn(
    _domain_zero   : &Vec<&V>    ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtomForwardOneValue
/// Callback to atomic functions during [ADfn::forward_one_value]
///
/// * Required :
/// If you will not use this atomic function with
/// [ADfn::forward_one_value], its corresponding value in
/// [AtomEval] can be None.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * domain_one :
/// this contains the direction for the directional derivative.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *range_one* is
/// ```text
///     range_one = f'(domain_zero) * domain_one
/// ```
pub type AtomForwardOneValue<V> = fn(
    _domain_zero   : &Vec<&V>    ,
    _domain_one    : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtomReverseOneValue
/// Callback to atomic functions during [ADfn::reverse_one_value]
///
/// * Required :
/// If you will not use this atomic function with
/// [ADfn::reverse_one_value],
/// this callbacks value in [AtomEval] can be None.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * range_one :
/// this contains the function weights for the partial derivatives.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *domain_one* is
/// ```text
///     domain_one = range_one * f'(domain_zero)
/// ```
pub type AtomReverseOneValue<V> = fn(
    _domain_zero   : &Vec<&V>    ,
    _range_one     : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtomForwardVarAd
/// Callback to atomic functions during [ADfn::forward_zero_ad]
///
/// * Required :
/// If you will not use this atomic function with
/// [ADfn::forward_one_ad] ,
/// this callbacks value in [AtomEval] can be None.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *arange_one*
/// contains the value of the atomic function range variables.
///
pub type AtomForwardVarAd<V> = fn(
    _domain_zero   : &Vec<& AD<V> >     ,
    _call_info     : IndexT             ,
    _trace         : bool               ,
) -> Vec< AD<V> > ;
//
// AtomForwardOneAD
/// Callback to atomic functions during [ADfn::forward_one_ad]
///
/// * Required :
/// If you will not use this atomic function with
/// [ADfn::forward_one_ad] ,
/// this callbacks value in [AtomEval] can be None.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * domain_one :
/// this contains the direction for the directional derivative.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *range_one* is
/// ```text
///     range_one = f'(domain_zero) * domain_one
/// ```
pub type AtomForwardOneAD<V> = fn(
    _domain_zero   : &Vec<& AD<V> >    ,
    _domain_one    : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Vec< AD<V> > ;
//
// AtomReverseOneAD
/// Callback to atomic functions during [ADfn::reverse_one_ad]
///
/// * Required :
/// If you will not use this atomic function with
/// [ADfn::reverse_one_ad] ,
/// this callbacks value in [AtomEval] can be None.
///
/// * domain_zero :
/// this contains the value of the atomic function domain variables.
///
/// * range_one :
/// this contains the function weights for the partial derivatives.
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
///
/// * return :
/// The return value *domain_one* is
/// ```text
///     domain_one = range_one * f'(domain_zero)
/// ```
pub type AtomReverseOneAD<V> = fn(
    _domain_zero   : &Vec<& AD<V> >    ,
    _range_one     : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Vec< AD<V> > ;
//
// AtomEval
/// Atomic function evaluation routines.
#[derive(Clone)]
pub struct AtomEval<V> {
    //
    // required
    pub name                 : &'static str              ,
    pub forward_type         : AtomForwardType           ,
    //
    pub forward_zero_value   : Option< AtomForwardVarValue::<V> >,
    pub forward_zero_ad      : Option< AtomForwardVarAd::<V> >,
    //
    pub forward_one_value    : Option< AtomForwardOneValue::<V> > ,
    pub forward_one_ad       : Option< AtomForwardOneAD::<V> >    ,
    //
    pub reverse_one_value    : Option< AtomReverseOneValue::<V> > ,
    pub reverse_one_ad       : Option< AtomReverseOneAD::<V> >    ,
    //
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
    impl crate::atom::sealed::AtomEvalVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::atom::AtomEval<$V> > > {
            pub(crate) static ATOM_EVAL_VEC :
            RwLock< Vec< crate::atom::AtomEval<$V> > > =
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
    tape                  : &mut Tape<V>                  ,
    forward_type          : AtomForwardType               ,
    adomain               : Vec< AD<V> >                  ,
    range_zero            : Vec<V>                        ,
    atom_id               : IndexT                        ,
    call_info             : IndexT                        ,
    trace                 : bool                          ,
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
    // domain_ad_type
    let domain_ad_type : Vec<ADType> = adomain.iter().map(
        |adomain_j| adomain_j.ad_type.clone()
    ).collect();
    //
    // range_ad_type
    let range_ad_type = forward_type(&domain_ad_type, call_info, trace);
    //
    // n_dyp, n_var
    let n_dyp = tape.dyp.n_dom + tape.dyp.n_dep;
    let n_var = tape.var.n_dom + tape.var.n_dep;
    //
    // arange, n_dyp_res, n_var_res
    let mut n_dyp_res = 0;
    let mut n_var_res = 0;
    for i in 0 .. call_n_res {
        if range_ad_type[i].is_variable() {
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::DependentV;
            arange[i].index     = n_var + n_var_res;
            n_var_res += 1;
         } else {
            debug_assert!( range_ad_type[i].is_dynamic() );
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::DependentP;
            arange[i].index     = n_dyp + n_dyp_res;
            n_dyp_res += 1;
         }
    }
    for k in 0 .. 2 {
        //
        // n_res, sub_tape
        let n_res    : usize;
        let sub_tape : &mut OpSequence;
        if k == 0 {
            sub_tape = &mut tape.dyp;
            n_res    = n_dyp_res;
        } else {
            sub_tape = &mut tape.var;
            n_res    = n_var_res;
        }
        //
        // sub_tape
        if n_res > 0 {
            //
            // sub_tape.id_seq, sub_tape.arg_seq
            sub_tape.id_seq.push( CALL_OP );
            sub_tape.arg_seq.push( sub_tape.arg_all.len() as IndexT );
            //
            // sub_tape.arg_all, tape.cop
            sub_tape.arg_all.push( atom_id );                        // arg[0]
            sub_tape.arg_all.push( call_info );                      // arg[1]
            sub_tape.arg_all.push( call_n_arg as IndexT );           // arg[2]
            sub_tape.arg_all.push( call_n_res as IndexT );           // arg[3]
            sub_tape.arg_all.push( sub_tape.flag.len() as IndexT );  // arg[4]
            for _j in 0 .. 5 {
                sub_tape.arg_type.push( ADType::NoType );
            }
            //
            // sub_tape.arg_type, sub_tape.arg_all
            for j in 0 .. call_n_arg {
                sub_tape.arg_type.push( domain_ad_type[j].clone() );
                if domain_ad_type[j].is_constant() {
                    let index = tape.cop.len();
                    tape.cop.push( adomain[j].value.clone() );
                    sub_tape.arg_all.push( index as IndexT );   // arg[5+j]
                } else {
                    let index = adomain[j].index;
                    sub_tape.arg_all.push( index as IndexT );   // arg[5+j]
                }
            }
            //
            // sub_tape.flag
            sub_tape.flag.push( trace );          // flag[ arg[4] ]
            for j in 0 .. call_n_arg {
                let flag_j = domain_ad_type[j].is_variable();
                sub_tape.flag.push( flag_j );     // flag[ arg[4] + j + 1]
            }
            for i in 0 .. call_n_res {
                let flag_i = range_ad_type[i].is_variable();
                sub_tape.flag.push( flag_i );     // flag[ arg[4] + n_res + i]
            }
            //
            // sub_tape.n_dep
            sub_tape.n_dep += n_res;
            //
            // sub_tape.id_seq, sub_tape.arg_seq
            for _i in 0 .. (n_res - 1) {
                sub_tape.id_seq.push( CALL_RES_OP );
                sub_tape.arg_seq.push( sub_tape.arg_all.len() as IndexT );
            }
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
/// * V : see [doc_generic_v]
///
/// * adomain :
/// This is the value of the arguments to the atomic function.
/// `
/// * atom_id :
/// The [atom_id](register_atom#atom_id) returned by register_atom for this
/// atomic function.
///
/// * call_info :
/// This is information about this call that is be passed on to the
/// callback functions specified by [atom_eval](register_atom#atom_eval).
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
/// This may be useful for debugging atomic functions.
///
/// * return :
/// The return value *arange* is the range, as a function of the domain,
/// for this atomic function.
///
pub fn call_atom<V>(
    adomain     : Vec< AD<V> > ,
    atom_id     : IndexT       ,
    call_info   : IndexT       ,
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
    // forward_zero, forward_type
    let name           : &'static str;
    let forward_zero   : Option< AtomForwardVarValue<V> >;
    let forward_type   : AtomForwardType;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval     = &atom_eval_vec[atom_id as usize];
        forward_zero      = atom_eval.forward_zero_value.clone();
        name              = atom_eval.name;
        forward_type      = atom_eval.forward_type.clone();
    }
    if forward_zero.is_none() { panic!(
        "{} : forward_zero_value is not implemented for this atomic function",
        name,
    ); }
    let forward_zero = forward_zero.unwrap();
    //
    // domain_zero
    let mut domain_zero : Vec<&V> = Vec::with_capacity( adomain.len() );
    for j in 0 .. adomain.len() {
        domain_zero.push( &adomain[j].value );
    }
    //
    // range_zero
    let range_zero  = forward_zero( &domain_zero, call_info, trace );
    //
    // arange
    let arange : Vec< AD<V> >;
    if ! recording {
        arange = ad_from_vector(range_zero);
    } else {
        arange = local_key.with_borrow_mut( |tape| record_call_atom::<V>(
            tape,
            forward_type,
            adomain,
            range_zero,
            atom_id,
            call_info,
            trace,
        ) );
    }
    arange
}
