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
// doc_common_arguments
/// Common arguments for atomic function callbacks.
///
/// * domain :
/// this contains a value for the atomic function domain vector.
/// It will have the same length as the domain vector in the
/// corresponding [call_atom].
///
/// * call_info :
/// is the *call_info* value used when the atomic function was called.
///
/// * trace :
/// if true, a trace of the calculations may be printed on stdout.
/// (The callback function can decide not the print any output.)
#[cfg(doc)]
pub fn doc_common_arguments() {}
// -------------------------------------------------------------------------
//
// AtomForwardType
/// Callback to atomic functions to determine ADType of results.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     range_ad_type = forward_type(dom_ad_type, call_info, trace)
/// ```
///
/// forward_ad_type :
/// is the AtomForwardType callback for this atomic function.
///
/// * dom_ad_type :
/// This has the same length as the domain vector in the corresponding
/// [call_atom].
/// Its j-th component is the [ADType] of the j-th component
/// of the domain vector.
///
/// Other Arguments : see [doc_common_arguments]
///
/// * range_ad_type :
/// This has the same length as *arange* in the corresponding [call_atom].
/// The i-th component is the [ADType] of the i-th component
/// of *arange* .
/// Note that if a result depends on two arguments, the ADType of the result
/// is the maximum of the ADType for the two arguments.
///
pub type AtomForwardType = fn(
    _domain_ad_type  : &[ADType]    ,
    _call_info       : IndexT       ,
    _trace           : bool         ,
)-> Vec<ADType>;
// -------------------------------------------------------------------------
//
// AtonForwardFunValue
/// Callback to atomic functions during
/// forward_dyp_value and forward_var_value.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Arguments : see [doc_common_arguments]
///
/// * return :
/// The return
/// contains the value of the atomic function range variables.
///
pub type AtonForwardFunValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtonForwardFunAd
/// Callback to atomic functions during
/// forward_dyp_ad and forward_var_ad.
///
/// * Required :
/// If you do not use this atomic function with [ADfn::forward_der_ad],
/// its corresponding value in [AtomEval] can be None.
///
/// * Arguments : see [doc_common_arguments]
///
/// * return :
/// The return
/// contains the value of the atomic function range variables.
///
pub type AtonForwardFunAd<V> = fn(
    _domain        : &Vec<& AD<V> >     ,
    _call_info     : IndexT             ,
    _trace         : bool               ,
) -> Vec< AD<V> > ;
// -------------------------------------------------------------------------
//
// AtonForwardDerValue
/// Callback to atomic functions during forward_der_value
///
/// * Required :
/// If you do not use this atomic function with [ADfn::forward_der_value],
/// its corresponding value in [AtomEval] can be None.
///
/// * dom_der    :
/// this contains the domain space direction for the directional derivative.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// The return value *range_der* is
/// ```text
///     range_der = f'(domain) * dom_der
/// ```
pub type AtonForwardDerValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _dom_der       : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtonForwardDerAD
/// Callback to atomic functions during forward_der_ad
///
/// * Required :
/// If you do not use this atomic function with [ADfn::forward_der_ad],
/// its corresponding value in [AtomEval] can be None.
///
/// * dom_der    :
/// this contains the domain space direction for the directional derivative.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// The return value *range_der* is
/// ```text
///     range_der = f'(domain) * dom_der
/// ```
pub type AtonForwardDerAD<V> = fn(
    _domain        : &Vec<& AD<V> >    ,
    _dom_der       : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Vec< AD<V> > ;
// -------------------------------------------------------------------------
//
// AtomReverseOneValue
/// Callback to atomic functions during reverse_der_value
///
/// * Required :
/// If you do not use this atomic function with [ADfn::reverse_der_value],
/// its corresponding value in [AtomEval] can be None.
///
/// * range_der :
/// this contains the ramge space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// The return value *dom_der* is
/// ```text
///     dom_der = range_der * f'(domain)
/// ```
pub type AtomReverseOneValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _range_der     : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Vec<V> ;
//
// AtomReverseOneAD
/// Callback to atomic functions during reverse_der_ad
///
/// * Required :
/// If you do not use this atomic function with [ADfn::reverse_der_ad],
/// its corresponding value in [AtomEval] can be None.
///
/// * range_der :
/// this contains the ramge space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// The return value *dom_der* is
/// ```text
///     dom_der = range_der * f'(domain)
/// ```
pub type AtomReverseOneAD<V> = fn(
    _domain        : &Vec<& AD<V> >    ,
    _range_der     : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Vec< AD<V> > ;
// ----------------------------------------------------------------------------
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
    pub forward_fun_value    : Option< AtonForwardFunValue::<V> >,
    pub forward_fun_ad       : Option< AtonForwardFunAd::<V> >,
    //
    pub forward_der_value    : Option< AtonForwardDerValue::<V> > ,
    pub forward_der_ad       : Option< AtonForwardDerAD::<V> >    ,
    //
    pub reverse_der_value    : Option< AtomReverseOneValue::<V> > ,
    pub reverse_der_ad       : Option< AtomReverseOneAD::<V> >    ,
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
    range                 : Vec<V>                        ,
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
    // n_dom, n_res
    let n_dom = adomain.len();
    let n_res = range.len();
    //
    // arange
    let mut arange : Vec< AD<V> > = ad_from_vector(range);
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
    // arange, n_dyp_dep, n_var_dep
    let mut n_dyp_dep = 0;
    let mut n_var_dep = 0;
    for i in 0 .. n_res {
        if range_ad_type[i].is_variable() {
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::Variable;
            arange[i].index     = n_var + n_var_dep;
            n_var_dep += 1;
         } else if range_ad_type[i].is_dynamic() {
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::DynamicP;
            arange[i].index     = n_dyp + n_dyp_dep;
            n_dyp_dep += 1;
         } else {
            assert!( range_ad_type[i].is_constant() );
        }
    }
    for k in 0 .. 2 {
        //
        // n_dep, sub_tape
        let n_dep    : usize;
        let sub_tape : &mut OpSequence;
        if k == 0 {
            sub_tape = &mut tape.dyp;
            n_dep    = n_dyp_dep;
        } else {
            sub_tape = &mut tape.var;
            n_dep    = n_var_dep;
        }
        //
        // sub_tape
        if n_dep > 0 {
            //
            // sub_tape.id_seq, sub_tape.arg_seq
            sub_tape.id_seq.push( CALL_OP );
            sub_tape.arg_seq.push( sub_tape.arg_all.len() as IndexT );
            //
            // sub_tape.arg_all, tape.cop
            sub_tape.arg_all.push( atom_id );                        // arg[0]
            sub_tape.arg_all.push( call_info );                      // arg[1]
            sub_tape.arg_all.push( n_dom as IndexT );                // arg[2]
            sub_tape.arg_all.push( n_res as IndexT );                // arg[3]
            sub_tape.arg_all.push( n_dep as IndexT );                // arg[4]
            sub_tape.arg_all.push( sub_tape.flag.len() as IndexT );  // arg[5]
            for _j in 0 .. 6 {
                sub_tape.arg_type.push( ADType::NoType );
            }
            //
            // sub_tape.arg_type, sub_tape.arg_all
            for j in 0 .. n_dom {
                sub_tape.arg_type.push( domain_ad_type[j].clone() );
                if domain_ad_type[j].is_constant() {
                    let index = tape.cop.len();
                    tape.cop.push( adomain[j].value.clone() );
                    sub_tape.arg_all.push( index as IndexT );   // arg[6+j]
                } else {
                    let index = adomain[j].index;
                    sub_tape.arg_all.push( index as IndexT );   // arg[6+j]
                }
            }
            //
            // sub_tape.flag
            sub_tape.flag.push( trace );          // flag[ arg[5] ]
            //
            // sub_tape.n_dep
            sub_tape.n_dep += n_dep;
            //
            // sub_tape.id_seq, sub_tape.arg_seq
            for _i in 0 .. (n_dep - 1) {
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
    let forward_zero   : Option< AtonForwardFunValue<V> >;
    let forward_type   : AtomForwardType;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval     = &atom_eval_vec[atom_id as usize];
        forward_zero      = atom_eval.forward_fun_value.clone();
        name              = atom_eval.name;
        forward_type      = atom_eval.forward_type.clone();
    }
    if forward_zero.is_none() { panic!(
        "{} : forward_fun_value is not implemented for this atomic function",
        name,
    ); }
    let forward_zero = forward_zero.unwrap();
    //
    // domain
    let mut domain      : Vec<&V> = Vec::with_capacity( adomain.len() );
    for j in 0 .. adomain.len() {
        domain.push( &adomain[j].value );
    }
    //
    // range
    let range  = forward_zero( &domain, call_info, trace );
    //
    // arange
    let arange : Vec< AD<V> >;
    if ! recording {
        arange = ad_from_vector(range);
    } else {
        arange = local_key.with_borrow_mut( |tape| record_call_atom::<V>(
            tape,
            forward_type,
            adomain,
            range,
            atom_id,
            call_info,
            trace,
        ) );
    }
    arange
}
