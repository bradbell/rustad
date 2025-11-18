// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub module implements AD atomic functions
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
    AtomInfoVecPublic,
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
// AtomDepend
/// Callback to atomic functions to determine its sparsity pattern.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     pattern = depend(n_dom, call_info, trace) ?
/// ```
///
/// * sparsity :
/// is the AtomDepend callback for this atomic function.
///
/// * n_dom :
/// This has the length as the domain vector in the corresponding
/// [call_atom].
///
/// Other Arguments : see [doc_common_arguments]
///
/// * pattern :
/// The the return value *pattern* is vector of \[row, column\] pairs.
/// Each row (column) is less than the range (domain)
/// dimension for this atomic function call.
/// If a pair \[i, j\] does not appear, the range component
/// with index i does not depend on the domain component with index j.
///
pub type AtomDepend = fn(
    _n_dom           : usize        ,
    _call_info       : IndexT       ,
    _trace           : bool         ,
)-> Result< Vec< [usize; 2] >, String >;
//
// AtomForwardType
/// Callback to atomic functions to determine ADType of results.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     range_ad_type = forward_type(&dom_ad_type, call_info, trace) ?
/// ```
///
/// * forward_ad_type :
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
)-> Result< Vec<ADType>, String >;
// -------------------------------------------------------------------------
//
// AtomForwardFunValue
/// Callback to atomic functions during
/// forward_dyp_value and forward_var_value.
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     range = forward_fun_value(&domain, call_info, trace) ?
/// ```
///
/// * forward_fun_value :
/// is the AtomForwardFunValue callback for this atomic function.
///
/// * Arguments : see [doc_common_arguments]
///
/// * range :
/// contains the value of the atomic function range variables.
///
pub type AtomForwardFunValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Result< Vec<V>, String > ;
//
// AtomForwardFunAd
/// Callback to atomic functions during
/// forward_dyp_ad and forward_var_ad.
///
/// * Required :
/// If you do not use this atomic function with
/// [ADfn::forward_dyp_ad] or [ADfn::forward_var_ad]
/// the callback in [AtomCallback] can be None.
///
/// * Syntax :
/// ```text
///     range = forward_fun_ad(&domain, call_info, trace) ?
/// ```
///
/// * forward_fun_ad :
/// is the AtomForwardFunAd callback for this atomic function.
///
/// * Arguments : see [doc_common_arguments]
///
/// * range :
/// contains the value of the atomic function range variables.
///
pub type AtomForwardFunAd<V> = fn(
    _domain        : &Vec<& AD<V> >     ,
    _call_info     : IndexT             ,
    _trace         : bool               ,
) -> Result< Vec< AD<V> >, String > ;
// -------------------------------------------------------------------------
//
// AtomForwardDerValue
/// Callback to atomic functions during forward_der_value
///
/// * Required :
/// If you do not use this atomic function with [ADfn::forward_der_value],
/// this callback in [AtomCallback] can be None.
///
/// * Syntax :
/// ```text
///     range_der = forward_der_value(&domain, domain_der, call_info, trace) ?
/// ```
///
/// * forward_der_value :
/// is the AtomForwardDerValue callback for this atomic function.
///
/// * dom_der    :
/// this contains the domain space direction for the directional derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * range_der :
/// is the directional derivative for each of the range space variables.
/// ```text
///     range_der = f'(domain) * dom_der
/// ```
pub type AtomForwardDerValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _dom_der       : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Result< Vec<V>, String >;
//
// AtomForwardDerAd
/// Callback to atomic functions during forward_der_ad
///
/// * Required :
/// If you do not use this atomic function with [ADfn::forward_der_ad],
/// this callback in [AtomCallback] can be None.
///
/// * Syntax :
/// ```text
///     range_der = forward_der_ad(&domain, domain_der, call_info, trace) ?
/// ```
///
/// * forward_der_ad :
/// is the AtomForwardDerAd callback for this atomic function.
///
/// * dom_der    :
/// this contains the domain space direction for the directional derivative.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * range_der :
/// is the directional derivative for each of the range space variables.
/// ```text
///     range_der = f'(domain) * dom_der
/// ```
pub type AtomForwardDerAd<V> = fn(
    _domain        : &Vec<& AD<V> >    ,
    _dom_der       : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Result< Vec< AD<V> >, String >;
// -------------------------------------------------------------------------
//
// AtomReverseDerValue
/// Callback to atomic functions during reverse_der_value
///
/// * Required :
/// If you do not use this atomic function with [ADfn::reverse_der_value],
/// this callback in [AtomCallback] can be None.
///
/// * Syntax :
/// ```text
///     dom_der = reverse_der_value(&domain, range_der, call_info, trace) ?
/// ```
///
/// * reverse_der_value :
/// is the AtomReverseDerValue callback for this atomic function.
///
/// * range_der :
/// this contains the ramge space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * dom_der :
/// is the partial derivative for each of the domain space variables.
/// ```text
///     dom_der = range_der * f'(domain)
/// ```
pub type AtomReverseDerValue<V> = fn(
    _domain        : &Vec<&V>    ,
    _range_der     : Vec<&V>     ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Result< Vec<V>, String>;
//
// AtomReverseDerAD
/// Callback to atomic functions during reverse_der_ad
///
/// * Required :
/// If you do not use this atomic function with [ADfn::reverse_der_ad],
/// this callback in [AtomCallback] can be None.
///
/// * Syntax :
/// ```text
///     dom_der = reverse_der_ad(&domain, range_der, call_info, trace) ?
/// ```
///
/// * reverse_der_ad :
/// is the AtomReverseDerAd callback for this atomic function.
///
/// * range_der :
/// this contains the ramge space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// is the partial derivative for each of the domain space variables.
/// ```text
///     dom_der = range_der * f'(domain)
/// ```
pub type AtomReverseDerAD<V> = fn(
    _domain        : &Vec<& AD<V> >    ,
    _range_der     : Vec<& AD<V> >     ,
    _call_info     : IndexT            ,
    _trace         : bool              ,
) -> Result< Vec< AD<V> >, String >;
// ----------------------------------------------------------------------------
//
// AtomCallback
/// Atomic function evaluation routines.
#[derive(Clone)]
pub struct AtomCallback<V> {
    //
    /// name used to distinguish this atomic function.
    pub name                 : &'static str,
    //
    pub depend               : Option< AtomDepend >,
    pub forward_type         : Option< AtomForwardType >,
    //
    pub forward_fun_value    : Option< AtomForwardFunValue::<V> >,
    pub forward_fun_ad       : Option< AtomForwardFunAd::<V> >,
    //
    pub forward_der_value    : Option< AtomForwardDerValue::<V> > ,
    pub forward_der_ad       : Option< AtomForwardDerAd::<V> >    ,
    //
    pub reverse_der_value    : Option< AtomReverseDerValue::<V> > ,
    pub reverse_der_ad       : Option< AtomReverseDerAD::<V> >    ,
    //
}
// ----------------------------------------------------------------------------
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use std::sync::RwLock;
    use super::AtomCallback;
    //
    // AtomInfoVec
    pub trait AtomInfoVec
    where
        Self : Sized + 'static,
    {   fn callback_vec() -> &'static RwLock< Vec< AtomCallback<Self> > >;
    }
}
//
// impl_callback_vec!
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
macro_rules! impl_callback_vec{ ($V:ty) => {
    #[doc = concat!(
        "The atomic evaluation vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::atom::sealed::AtomInfoVec for $V {
        fn callback_vec() -> &'static
        RwLock< Vec< crate::atom::AtomCallback<$V> > > {
            pub(crate) static ATOM_CALLBACK_VEC :
            RwLock< Vec< crate::atom::AtomCallback<$V> > > =
                RwLock::new( Vec::new() );
            &ATOM_CALLBACK_VEC
        }
    }
} }
pub(crate) use impl_callback_vec;
// ----------------------------------------------------------------------------
// register_atom
/// Register an atomic function.
///
/// * Syntax :
/// ```text
///     atom_id = register_atom(callback)
/// ```
///
/// * V : see [doc_generic_v]
///
/// ## callback :
/// contains references to the callback functions that compute
/// values for this atomic function.
///
/// ## atom_id :
/// is the index that is used to identify this atomic function.
///
pub fn register_atom<V>( callback : AtomCallback<V> ) -> IndexT
where
    V : AtomInfoVecPublic ,
{   //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomCallback<V> > > =
        sealed::AtomInfoVec::callback_vec();
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
        let mut callback_vec = write_lock.unwrap();
        let atom_id_usize     = callback_vec.len();
        atom_id_too_large     = (IndexT::MAX as usize) < atom_id_usize;
        atom_id               = callback_vec.len() as IndexT;
        callback_vec.push( callback );
    }
    assert!( ! atom_id_too_large );
    atom_id
}
// ----------------------------------------------------------------------------
// record_call_atom
fn record_call_atom<V>(
    tape                  : &mut Tape<V>                  ,
    name                  : &str                          ,
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
    let result = forward_type(&domain_ad_type, call_info, trace);
    let range_ad_type = match result {
        Err(msg) => {
            panic!( "atom {} forward_type error : {}", name, msg);
        },
        Ok(vec_ad_type) => vec_ad_type,
    };
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
            sub_tape.arg_all.push( sub_tape.flag_all.len() as IndexT );  // arg[5]
            for _j in 0 .. 6 {
                sub_tape.arg_type_all.push( ADType::Empty );
            }
            //
            // sub_tape.arg_type_all, sub_tape.arg_all
            for j in 0 .. n_dom {
                sub_tape.arg_type_all.push( domain_ad_type[j].clone() );
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
            // sub_tape.flag_all
            if trace {
                sub_tape.flag_all.push( ADType::True );          // flag_all[ arg[5] ]
            } else {
                sub_tape.flag_all.push( ADType::False );         // flag_all[ arg[5] ]
            }
            for i in 0 .. n_res {
                //flag_all[ arg[5] + i + 1 ]
                sub_tape.flag_all.push( arange[i].ad_type.clone() )
            }
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
/// callback functions specified by [callback](register_atom#callback).
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
    V   : Clone + From<f32> + ThisThreadTapePublic + AtomInfoVecPublic ,
{
    //
    // local_key
    let local_key : &LocalKey< RefCell< Tape<V> > > = ThisThreadTape::get();
    //
    // recording
    let recording : bool = local_key.with_borrow( |tape| tape.recording );
    //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomCallback<V> > > =
        sealed::AtomInfoVec::callback_vec();
    //
    // forward_zero, forward_type
    let name           : &'static str;
    let forward_zero   : Option< AtomForwardFunValue<V> >;
    let forward_type   : Option< AtomForwardType >;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let callback_vec = read_lock.unwrap();
        let callback     = &callback_vec[atom_id as usize];
        forward_zero      = callback.forward_fun_value.clone();
        name              = callback.name;
        forward_type      = callback.forward_type.clone();
    }
    if forward_type.is_none() { panic!(
        "{} : forward_type is not implemented for this atomic function",
        name,
    ); }
    if forward_zero.is_none() { panic!(
        "{} : forward_fun_value is not implemented for this atomic function",
        name,
    ); }
    let forward_type = forward_type.unwrap();
    let forward_zero = forward_zero.unwrap();
    //
    // domain
    let mut domain      : Vec<&V> = Vec::with_capacity( adomain.len() );
    for j in 0 .. adomain.len() {
        domain.push( &adomain[j].value );
    }
    //
    // range
    let result  = forward_zero( &domain, call_info, trace );
    let range = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_value error : {}", name, msg);
        },
        Ok(range) => range,
    };
    //
    // arange
    let arange : Vec< AD<V> >;
    if ! recording {
        arange = ad_from_vector(range);
    } else {
        arange = local_key.with_borrow_mut( |tape| record_call_atom::<V>(
            tape,
            name,
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
