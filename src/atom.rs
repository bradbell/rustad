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
use std::cmp::max;
//
use crate::tape::OpSequence;
use crate::op::id::CALL_OP;
use crate::op::id::CALL_RES_OP;
use crate::op::call::BEGIN_DOM;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    ad_from_vector,
    GlobalAtomCallbackVecPublic,
    ThisThreadTapePublic,
};
//
#[cfg(doc)]
use crate::{
        doc_generic_v,
        ADfn,
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
// AtomRevDepend
/// Callback to atomic functions to determine its sparsity pattern.
///
///
/// * Required :
/// This callback is required for all atomic functions.
///
/// * Syntax :
/// ```text
///     error_msg = rev_depend(&mut depend, rng_index, n_dom, call_info, trace)
/// ```
///
/// * rev_depend :
/// is the AtomRevDepend callback for this atomic function.
///
/// * depend :
/// This vector is empty on input,
/// only its capacity matters on input (to avoid reallocating memory).
/// Upon return, it contains the domain index values that the specified
/// range index value depends on.
/// If range component i does not depend on domain component j,
/// domain component j may be any value when computing range component i;
/// e.g. nan.
///
/// * rng_index   :
/// is the range index that that the dependencies are computed for.
///
/// * n_dom :
/// This has the length as the domain vector in the corresponding
/// [call_atom].
///
/// Other Arguments : see [doc_common_arguments]
///
/// * error_msg :
/// If *error_msg* is empty, there was no error.
/// Otherwise it contains an error message and the value in *depend* is not
/// specified.
///
pub type AtomRevDepend = fn(
    _depend          : &mut Vec<usize>   ,
    _rng_index       : usize             ,
    _n_dom           : usize             ,
    _call_info       : IndexT            ,
    _trace           : bool              ,
)-> String;
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
///     range = forward_fun_value(&use_range, &domain, call_info, trace) ?
/// ```
///
/// * forward_fun_value :
/// is the AtomForwardFunValue callback for this atomic function.
///
/// * use_range :
/// If use_range\[i\] is true (false),
/// the value range\[i\] is used (is not used).
/// This may be used to avoid computations that are not needed.
/// This vector has length equal to n_range in [call_atom] .
///
/// * Arguments : see [doc_common_arguments]
///
/// * range :
/// contains the value of the atomic function range variables.
///
pub type AtomForwardFunValue<V> = fn(
    _use_range     : &[bool]     ,
    _domain        : &[&V]       ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Result< Vec<V>, String > ;
//
// AtomForwardFunAD
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
///     range = forward_fun_ad(&use_range, &domain, call_info, trace) ?
/// ```
///
/// * forward_fun_ad :
/// is the AtomForwardFunAD callback for this atomic function.
///
/// * use_range :
/// If use_range\[i\] is true (false),
/// the value range\[i\] is used (is not used).
/// This may be used to avoid computations that are not needed.
/// This vector has length equal to n_range in [call_atom] .
///
/// * Arguments : see [doc_common_arguments]
///
/// * range :
/// contains the value of the atomic function range variables.
///
pub type AtomForwardFunAD<V> = fn(
    _use_range     : &[bool]            ,
    _domain        : &[& AD<V>]         ,
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
/// * use_range :
/// If use_range\[i\] is true (false),
/// the value range_der\[i\] is used (is not used).
/// This may be used to avoid computations that are not needed.
/// This vector has length equal to n_range in [call_atom] .
///
/// * domain_der    :
/// this contains the domain space direction for the directional derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * range_der :
/// is the directional derivative for each of the range space variables.
/// ```text
///     range_der = f'(domain) * domain_der
/// ```
pub type AtomForwardDerValue<V> = fn(
    _use_range     : &[bool]     ,
    _domain        : &[&V]       ,
    _domain_der    : &[&V]       ,
    _call_info     : IndexT      ,
    _trace         : bool        ,
) -> Result< Vec<V>, String >;
//
// AtomForwardDerAD
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
/// is the AtomForwardDerAD callback for this atomic function.
///
/// * domain_der :
/// this contains the domain space direction for the directional derivative.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * range_der :
/// is the directional derivative for each of the range space variables.
/// ```text
///     range_der = f'(domain) * domain_der
/// ```
pub type AtomForwardDerAD<V> = fn(
    _use_range     : &[bool]           ,
    _domain        : &[& AD<V>]        ,
    _domain_der    : &[& AD<V>]        ,
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
///     domain_der = reverse_der_value(&domain, &range_der, call_info, trace) ?
/// ```
///
/// * reverse_der_value :
/// is the AtomReverseDerValue callback for this atomic function.
///
/// * range_der :
/// this contains the range space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * domain_der :
/// is the partial derivative for each of the domain space variables.
/// ```text
///     domain_der = range_der * f'(domain)
/// ```
pub type AtomReverseDerValue<V> = fn(
    _domain        : &[&V]       ,
    _range_der     : &[&V]       ,
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
///     domain_der = reverse_der_ad(&domain, &range_der, call_info, trace) ?
/// ```
///
/// * reverse_der_ad :
/// is the AtomReverseDerAD callback for this atomic function.
///
/// * range_der :
/// this contains the range space weights for the partial derivatives.
///
/// * Other Arguments : see [doc_common_arguments]
///
/// * return :
/// is the partial derivative for each of the domain space variables.
/// ```text
///     domain_der = range_der * f'(domain)
/// ```
pub type AtomReverseDerAD<V> = fn(
    _domain        : &[& AD<V>]        ,
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
    pub rev_depend           : Option< AtomRevDepend >,
    //
    pub forward_fun_value    : Option< AtomForwardFunValue::<V> > ,
    pub forward_fun_ad       : Option< AtomForwardFunAD::<V> >    ,
    //
    pub forward_der_value    : Option< AtomForwardDerValue::<V> > ,
    pub forward_der_ad       : Option< AtomForwardDerAD::<V> >    ,
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
    #[cfg(doc)]
    use crate::doc_generic_v;
    //
    // GlobalAtomCallbackVec
    pub trait GlobalAtomCallbackVec
    where
        Self : Sized + 'static,
    {   /// Returns a reference to the map from atom_id to callback information
        ///
        /// ```text
        ///     let rw_lock = GlobalAtomCallbackVec::get();
        /// ```
        ///
        /// * Self : must be a value type V in [doc_generic_v]
        ///
        /// * rw_lock :
        /// is a read-write lock object [std::sync::RwLock]
        ///
        /// * write_lock :
        /// ``` text
        ///     let write_lock    = rw_lock.write();
        ///     let callback_vec  = write_lock.unwrap();
        /// ```
        ///
        /// * read_lock :
        /// ``` text
        ///     let read_lock    = rw_lock.read();
        ///     let callback_vec = read_lock.unwrap();
        /// ```
        ///
        /// * callback_vec :
        /// callback_vec\[atom_id\] is the callback information
        /// for the atomic function corresponding to atom_id.
        ///
        fn get() -> &'static RwLock< Vec< AtomCallback<Self> > >;
    }
}
//
// impl_global_atom_callback_vec!
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
macro_rules! impl_global_atom_callback_vec{ ($V:ty) => {
    #[doc = concat!(
        "The atomic evaluation vector for value type `", stringify!($V), "`"
    ) ]
    impl crate::atom::sealed::GlobalAtomCallbackVec for $V {
        fn get() -> &'static
        RwLock< Vec< crate::atom::AtomCallback<$V> > > {
            pub(crate) static ATOM_CALLBACK_VEC :
                RwLock< Vec< crate::atom::AtomCallback<$V> > > =
                    RwLock::new( Vec::new() );
            &ATOM_CALLBACK_VEC
        }
    }
} }
pub(crate) use impl_global_atom_callback_vec;
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
    V : GlobalAtomCallbackVecPublic ,
{   //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomCallback<V> > > =
        sealed::GlobalAtomCallbackVec::get();
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
    rev_depend            : AtomRevDepend                 ,
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
    // rng_ad_type
    let mut rng_ad_type   : Vec<ADType> = Vec::with_capacity(n_res);
    let mut depend        : Vec<usize>  = Vec::new();
    for rng_index in 0 .. n_res {
        depend.clear();
        let error_msg = rev_depend(
            &mut depend, rng_index, n_dom, call_info, trace
        );
        if error_msg.len() != 0 {
            panic!( "atom {} rev_depend error_msg : {}", name, error_msg);
        }
        let mut ad_type = ADType::ConstantP;
        for k in 0 .. depend.len() {
            let j = depend[k];
            if  n_dom <= j {
                panic!(
                    "atom {} rev_depend : \
                    rng_index   = {},
                    n_dom = {}, \
                    k = {} \
                    depend[k] = {} >= n_dom",
                    name, rng_index, n_dom, k, depend[k]
                );
            }
            ad_type = max( ad_type, domain_ad_type[j].clone() );
        }
        rng_ad_type.push( ad_type );
    }
    //
    // n_dyp, n_var
    let n_dyp = tape.dyp.n_dom + tape.dyp.n_dep;
    let n_var = tape.var.n_dom + tape.var.n_dep;
    //
    // arange, dyp_dep, var_dep
    let mut dyp_dep  : Vec<usize> = Vec::new();
    let mut var_dep  : Vec<usize> = Vec::new();
    let mut dyp_flag              = vec![false; n_res];
    let mut var_flag              = vec![false; n_res];
    for i in 0 .. n_res {
        if rng_ad_type[i].is_variable() {
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::Variable;
            arange[i].index     = n_var + var_dep.len();
            var_dep.push( i );
            var_flag[i] = true;
         } else if rng_ad_type[i].is_dynamic() {
            arange[i].tape_id   = tape.tape_id;
            arange[i].ad_type   = ADType::DynamicP;
            arange[i].index     = n_dyp + dyp_dep.len();
            dyp_dep.push( i );
            dyp_flag[i] = true;
         } else {
            assert!( rng_ad_type[i].is_constant() );
        }
    }
    for k in 0 .. 2 {
        //
        // op_seq, dep, n_dep
        let op_seq   : &mut OpSequence;
        let dep      : &Vec<usize>;
        let flag     : &Vec<bool>;
        if k == 0 {
            op_seq   = &mut tape.dyp;
            dep      = &dyp_dep;
            flag     = &dyp_flag;
        } else {
            op_seq   = &mut tape.var;
            dep      = &var_dep;
            flag     = &var_flag;
        }
        let n_dep = dep.len();
        //
        // op_seq
        if n_dep > 0 {
            //
            // op_seq.id_all, op_seq.arg_start
            op_seq.id_all.push( CALL_OP );
            op_seq.arg_start.push( op_seq.arg_all.len() as IndexT );
            //
            // op_seq.arg_all, tape.cop
            op_seq.arg_all.push( atom_id );                        // arg[0]
            op_seq.arg_all.push( call_info );                      // arg[1]
            op_seq.arg_all.push( n_dom as IndexT );                // arg[2]
            op_seq.arg_all.push( n_res as IndexT );                // arg[3]
            // arg[4]
            op_seq.arg_all.push( op_seq.flag_all.len() as IndexT );
            //
            // op_seq.arg_type_all
            for _j in 0 .. BEGIN_DOM {
                op_seq.arg_type_all.push( ADType::Empty );
            }
            //
            // op_seq.arg_type_all, op_seq.arg_all
            for j in 0 .. n_dom {
                op_seq.arg_type_all.push( domain_ad_type[j].clone() );
                if domain_ad_type[j].is_constant() {
                    let index = tape.cop.len();
                    tape.cop.push( adomain[j].value.clone() );
                    op_seq.arg_all.push( index as IndexT ); // arg[BEGIN_DOM+j]
                } else {
                    let index = adomain[j].index;
                    op_seq.arg_all.push( index as IndexT ); // arg[BEGIN_DOM+j]
                }
            }
            //
            // op_seq.flag_all
            op_seq.flag_all.push( trace );  // flag_all[ arg[5] ]
            for i in 0 .. n_res {
                //flag_all[ arg[5] + i + 1 ]
                op_seq.flag_all.push( flag[i] )
            }
            //
            // op_seq.n_dep
            op_seq.n_dep += n_dep;
            //
            // op_seq.id_all, op_seq.arg_start
            for dep_index in 1 .. n_dep {
                op_seq.id_all.push( CALL_RES_OP );
                op_seq.arg_start.push( op_seq.arg_all.len() as IndexT );
                //
                op_seq.arg_all.push( dep_index as IndexT );
                op_seq.arg_type_all.push( ADType::Empty );
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
/// * Syntax :
/// ```text
///     arange= call_atom(n_range, adomain, atom_id, call_info, trace)
/// ```
///
///
/// * V : see [doc_generic_v]
///
/// * n_range :
/// is the range space dimension for this atomic function call.
/// Note that the dimension of the range space may depend on the call.
///
/// * adomain :
/// This is the value of the arguments for this atomic function call.
/// Note that the dimension of the domain space may depend on the call.
///
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
/// * arange :
/// The return value *arange* is the range, as a function of the domain,
/// for this atomic function.
///
pub fn call_atom<V>(
    n_range     : usize        ,
    adomain     : Vec< AD<V> > ,
    atom_id     : IndexT       ,
    call_info   : IndexT       ,
    trace       : bool         ,
) -> Vec< AD<V> >
where
    V : Clone + From<f32> + ThisThreadTapePublic + GlobalAtomCallbackVecPublic,
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
        sealed::GlobalAtomCallbackVec::get();
    //
    // forward_fun_value, rev_depend
    let name               : &'static str;
    let forward_fun_value  : Option< AtomForwardFunValue<V> >;
    let rev_depend         : Option< AtomRevDepend >;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let callback_vec   = read_lock.unwrap();
        let callback       = &callback_vec[atom_id as usize];
        forward_fun_value  = callback.forward_fun_value.clone();
        name               = callback.name;
        rev_depend         = callback.rev_depend.clone();
    }
    if rev_depend.is_none() { panic!(
        "{} : rev_depend is not implemented for this atomic function",
        name,
    ); }
    if forward_fun_value.is_none() { panic!(
        "{} : forward_fun_value is not implemented for this atomic function",
        name,
    ); }
    let rev_depend   = rev_depend.unwrap();
    let forward_fun_value = forward_fun_value.unwrap();
    //
    // domain
    let mut domain      : Vec<&V> = Vec::with_capacity( adomain.len() );
    for j in 0 .. adomain.len() {
        domain.push( &adomain[j].value );
    }
    //
    // use_range
    let use_range = vec![true; n_range];
    //
    // range
    let result  = forward_fun_value(  &use_range, &domain, call_info, trace );
    let range = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_value error : {}", name, msg);
        },
        Ok(range) => range,
    };
    if range.len() != n_range { panic!(
        "atom {} forward_fun_value : domain.len() = {} \
        expected range.len() = {} found range.len() = {}",
        name, domain.len(), n_range, range.len()
    ); }
    //
    // arange
    let arange : Vec< AD<V> >;
    if ! recording {
        arange = ad_from_vector(range);
    } else {
        arange = local_key.with_borrow_mut( |tape| record_call_atom::<V>(
            tape,
            name,
            rev_depend,
            adomain,
            range,
            atom_id,
            call_info,
            trace,
        ) );
    }
    arange
}
