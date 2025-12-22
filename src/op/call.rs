// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// --------------------------------------------------------------------------
//! Operator that calls an atomic function
//!
//! Link to [parent module](super)
//!
//! # CALL_OP
//!
//! ## Operator Arguments
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index that identifies the atomic function; i.e., atom_id |
//! | 1        | Extra information about this call; i.e. call_info        |
//! | 2        | Domain space dimension for function being called (n_dom) |
//! | 3        | Number of range components for this call         (n_rng) |
//! | 4        | Index in flag_all of first flag for this operator        |
//! | 4+1      | Variable, dynamic, or constant index for first call argument  |
//! | 4+2      | Variable, dynamic, or constant index for second call argument |
//! | ...      | ...                                                           |
//! | 4+n_dom   | Variable, dynamic, or constant index for last call argument  |
//!
//! ## Operator Flags
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | is True or false depending on trace for this call        |
//! | 1        | is first result a dependent for this call                |
//! | ...      | ...                                                      |
//! | n_rng    | is last result a dependent for this call                 |
//!
//! * CALL_RES_OP
//! The operation index for a CALL_OP operator, corresponds to the first
//! dependent created by the call.
//! There are n_dep - 1 CALL_RES_OP operators corresponding to the
//! other dependents created by the call operator.
//! Each such operator has the following arguments:
//!
//! ## Operator Arguments
//! | Index | Meaning |
//! | ----- | ------- |
//! | 0     | Dependent index corresponding to this CALL_RES_OP    |
//! |       | which is also the offset to get back to this CALL_OP |
// --------------------------------------------------------------------------
// use
//
use std::cmp::PartialEq;
use std::ops::AddAssign;
use std::mem::swap;
//
use crate::ad::ADType;
use crate::adfn::optimize;
use crate::tape::OpSequence;
use crate::op::info::OpInfo;
use crate::op::info::no_reverse_depend;
use crate::atom::sealed::GlobalAtomCallbackVec;
use crate::op::id::{
        CALL_OP,
        CALL_RES_OP,
};
use crate::{
    AD,
    IndexT,
    AtomCallback,
    ThisThreadTapePublic,
    ad_from_value,
};
// ----------------------------------------------------------------------
// BEGIN_DOM
/// Index, of the first argument for this call operator,
/// in an operator argument vector.
pub(crate) const BEGIN_DOM : usize = 5;
//
// BEGIN_FLAG
/// Index, of the first flag for this call operator,
/// in the vector of all the flags for this operation sequence.
pub(crate) const BEGIN_FLAG: usize = 4;
//
// NUMBER_NRNG
/// Index, of the number of range values for this call operator,
/// in an operator argument vector.
pub(crate) const NUMBER_RNG: usize = 3;
// ----------------------------------------------------------------------
// get_callback
fn get_callback<V> (atom_id : usize) -> AtomCallback<V>
where
    V               : GlobalAtomCallbackVec,
    AtomCallback<V> : Clone,
{   //
    // callback
    let callback : AtomCallback<V>;
    {   //
        // rw_lock
        let rw_lock = GlobalAtomCallbackVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let callback_vec  = read_lock.unwrap();
        callback          = callback_vec[atom_id].clone();
    }
    callback
}
//
// extract_call_info
pub(crate) fn extract_call_info<'a>(
    arg        : &'a [IndexT] ,
    flag_all   : &'a [bool]   ,
) -> (
    usize            , // atom_id
    IndexT           , // call_info
    usize            , // n_dom
    usize            , // n_rng
    bool             , // trace
    &'a [bool]       , // rng_is_dep
)
{
    // atom_id, call_info, n_dom, n_rng,
    let atom_id      = arg[0] as usize;
    let call_info    = arg[1];
    let n_dom        = arg[2] as usize;
    let n_rng        = arg[3] as usize;
    let start        = arg[4] as usize;
    let trace        = flag_all[start];
    let start        = start + 1;
    let rng_is_dep   = &flag_all[start .. start+n_rng];
    //
    (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    )
}
// ----------------------------------------------------------------------
// domain_value
fn domain_value<'a, 'b, V>(
    dyp_both   : &'a [V]       ,
    var_both   : &'a [V]       ,
    cop        : &'a [V]       ,
    arg        : &'b [IndexT]  ,
    arg_type   : &'b [ADType]  ,
    n_dom      : usize         ,
) -> Vec<&'a V>
where
    V : PartialEq + From<f32>,
{   //
    // nan_v
    let nan_v : V   = f32::NAN.into();
    //
    // no_var_both
    let no_var_both = var_both.len() == 1 && var_both[0] == nan_v;
    //
    let mut domain      : Vec<&V> = Vec::with_capacity( n_dom );
    for j_arg in 0 .. n_dom {
        let index   = arg[BEGIN_DOM + j_arg] as usize;
        let ad_type = &arg_type[BEGIN_DOM + j_arg];
        if ad_type.is_constant() {
            domain.push( &cop[index] );
        } else if ad_type.is_dynamic() {
            domain.push( &dyp_both[index] );
        } else {
            debug_assert!( ad_type.is_variable() );
            if no_var_both {
                domain.push( &var_both[0] );
            } else {
                domain.push( &var_both[index] );
            }
        }
    }
    domain
}
// ----------------------------------------------------------------------
// domain_acop
fn domain_acop <'a, 'b, V>(
    cop        : &'a [V]       ,
    arg        : &'b [IndexT]  ,
    arg_type   : &'b [ADType]  ,
    n_dom      : usize         ,
) -> Vec< AD<V> >
where
    V : Clone,
{   //
    let mut acop : Vec< AD<V> > = Vec::new();
    for i_arg in 0 .. n_dom {
        let ad_type = &arg_type[BEGIN_DOM + i_arg];
        if ad_type.is_constant() {
            let index = arg[BEGIN_DOM + i_arg] as usize;
            acop.push( ad_from_value( cop[index].clone() ) );
        }
    }
    acop
}
// ----------------------------------------------------------------------
// domain_ad
fn domain_ad<'a, 'b, V>(
    dyp_both   : &'a [AD<V>]    ,
    var_both   : &'a [AD<V>]    ,
    acop       : &'a [AD<V>]    ,
    arg        : &'b [IndexT]   ,
    arg_type   : &'b [ADType]   ,
    n_dom      : usize          ,
) -> Vec<&'a AD<V> >
where
    V : PartialEq,
{
    // no_var_both
    // This case is used during zero forward mode for dynamic parameters.
    // If no_var_both, then var_both[0] is nan.
    let no_var_both = var_both.len() == 1 &&
        var_both[0].value != var_both[0].value;
    //
    //
    let mut domain      : Vec<& AD<V> > = Vec::with_capacity( n_dom );
    let mut j_cop : usize = 0;
    for j_arg in 0 .. n_dom {
        let index   = arg[BEGIN_DOM + j_arg] as usize;
        let ad_type = &arg_type[BEGIN_DOM + j_arg];
        if ad_type.is_constant() {
            domain.push( &acop[j_cop] );
            j_cop += 1;
        } else if ad_type.is_dynamic() {
            domain.push( &dyp_both[index] );
        } else {
            debug_assert!( ad_type.is_variable() );
            if no_var_both {
                domain.push( &var_both[0] );
            } else {
                domain.push( &var_both[index] );
            }
        }
    }
    domain
}
// ==========================================================================
// call_forward_dyp
// ==========================================================================
// call_forward_dyp_value
/// Call operator V evaluation of dynamic parameters;
/// see [ForwardDyp](crate::op::info::ForwardDyp)
fn call_forward_dyp_value<V> (
    dyp_both   : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag_all   : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    V               : GlobalAtomCallbackVec + From<f32> + PartialEq,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_fun_value
    let forward_fun_value = &callback.forward_fun_value;
    if forward_fun_value.is_none() {
        panic!(
        "{} : forward_fun_value is not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_fun_value = forward_fun_value.unwrap();
    //
    // domain
    let nan_v    : V      = f32::NAN.into();
    let var_both : Vec<V> = vec![ nan_v ];
    let domain            = domain_value(
        dyp_both, &var_both, cop, arg, arg_type, n_dom
    );
    //
    // range
    let result = forward_fun_value( rng_is_dep, &domain, call_info, trace );
    let mut range = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_value error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!(
        n_rng,
        range.len(),
        "atom {} forward_fun_value return length: expected {}, found {}",
        callback.name,
        n_rng,
        range.len(),
    );
    //
    // dyp_both
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut dyp_both[res + dep_index], &mut range[rng_index] );
            dep_index += 1;
        }
    }
}
// ---------------------------------------------------------------------------
// call_forward_dyp_ad
/// Call operator `AD<V>` evaluation of dynamic parameters;
/// see [ForwardDyp](crate::op::info::ForwardDyp)
fn call_forward_dyp_ad<V> (
    adyp_both  : &mut Vec< AD<V> >   ,
    cop        : &Vec<V>             ,
    flag_all   : &Vec<bool>          ,
    arg        : &[IndexT]           ,
    arg_type   : &[ADType]           ,
    res        : usize               )
where
    V               : PartialEq + Clone + From<f32> + GlobalAtomCallbackVec,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_fun_ad
    let forward_fun_ad = &callback.forward_fun_ad;
    if forward_fun_ad.is_none() {
        panic!(
            "{} : forward_fun_ad is not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_fun_ad = forward_fun_ad.unwrap();
    //
    // adomain
    let acop     = domain_acop(cop, arg, arg_type, n_dom);
    let nan_v : V = f32::NAN.into();
    let anan      = ad_from_value( nan_v );
    let avar_both = vec! [ anan ];
    let adomain   = domain_ad(
        adyp_both, &avar_both, &acop, arg, arg_type, n_dom
    );
    //
    // arange_zero
    let result = forward_fun_ad( &rng_is_dep, &adomain, call_info, trace );
    let mut arange = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_ad error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!(
        n_rng,
        arange.len(),
        "atom {} forward_fun_ad return length: expected {}, found {}",
        callback.name,
        n_rng,
        arange.len(),
    );
    //
    // adyp_both
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut adyp_both[res + dep_index], &mut arange[rng_index] );
            dep_index += 1;
        }
    }
}
// ==========================================================================
// call_forward_var
// ==========================================================================
//
// call_forward_var_value
/// Call operator V evaluation of variables;
/// see [ForwardVar](crate::op::info::ForwardVar)
fn call_forward_var_value<V> (
    dyp_both   : &Vec<V>       ,
    var_both   : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag_all   : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    V               : GlobalAtomCallbackVec + PartialEq + From<f32>,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_fun_value
    let forward_fun_value = &callback.forward_fun_value;
    if forward_fun_value.is_none() {
        panic!(
        "{} : forward_fun_value is not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_fun_value = forward_fun_value.unwrap();
    //
    // domain
    let domain = domain_value(
        dyp_both, var_both, cop, arg, arg_type, n_dom
    );
    //
    // range
    let result = forward_fun_value(  rng_is_dep, &domain, call_info, trace );
    let mut range = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_value error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!(
        n_rng,
        range.len(),
        "atom {} forward_fun_value return length: expected {}, found {}",
        callback.name,
        n_rng,
        range.len(),
    );
    //
    // var_both
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut var_both[res + dep_index], &mut range[rng_index] );
            dep_index += 1;
        }
    }
}
// ---------------------------------------------------------------------------
// call_forward_var_ad
/// Call operator `AD<V>` evaluation of variables;
/// see [ForwardVar](crate::op::info::ForwardVar)
fn call_forward_var_ad<V> (
    adyp_both  : &Vec< AD<V> >       ,
    avar_both  : &mut Vec< AD<V> >   ,
    cop        : &Vec<V>             ,
    flag_all   : &Vec<bool>          ,
    arg        : &[IndexT]           ,
    arg_type   : &[ADType]           ,
    res        : usize               )
where
    V               : PartialEq + Clone + GlobalAtomCallbackVec,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_fun_ad
    let forward_fun_ad = &callback.forward_fun_ad;
    if forward_fun_ad.is_none() {
        panic!(
            "{} : forward_fun_ad is not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_fun_ad = forward_fun_ad.unwrap();
    //
    // adomain
    let acop     = domain_acop(cop, arg, arg_type, n_dom);
    let adomain  = domain_ad(
        adyp_both, avar_both, &acop, arg, arg_type, n_dom
    );
    //
    // arange
    let result = forward_fun_ad( &rng_is_dep, &adomain, call_info, trace );
    let mut arange = match result {
        Err(msg) => { panic!(
            "atom {} forward_fun_ad error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!(
        n_rng,
        arange.len(),
        "atom {} forward_fun_ad return length: expected {}, found {}",
        callback.name,
        n_rng,
        arange.len(),
    );
    //
    // avar_both
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut avar_both[res + dep_index], &mut arange[rng_index] );
            dep_index += 1;
        }
    }
}
// ==========================================================================
// call_forward_der
// ==========================================================================
//
// call_forward_der_value
/// Call operator V evaluation of forward mode derivatives;
/// see [ForwardDer](crate::op::info::ForwardDer)
fn call_forward_der_value<V> (
    dyp_both   : &Vec<V>       ,
    var_both   : &Vec<V>       ,
    var_der    : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag_all   : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    V               : PartialEq + GlobalAtomCallbackVec + From<f32>,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_der_value
    let forward_der_value  = &callback.forward_der_value;
    if forward_der_value.is_none() {
        panic!(
            "{} : forward_der_value not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_der_value  = forward_der_value.unwrap();
    //
    // domain
    let domain = domain_value(
        dyp_both, var_both, cop, arg, arg_type, n_dom
    );
    // domain_der
    let zero_v : V = 0f32.into();
    let mut domain_der : Vec<&V> = Vec::with_capacity( n_dom );
    for i_dom in 0 .. n_dom {
        let index   = arg[BEGIN_DOM + i_dom] as usize;
        let ad_type = arg_type[BEGIN_DOM + i_dom].clone();
        if ad_type.is_variable() {
            domain_der.push( &var_der[index] );
        } else {
            domain_der.push( &zero_v );
        }
    }
    // range_der
    let result = forward_der_value(
        &rng_is_dep, &domain, &domain_der, call_info, trace
    );
    let mut range_der = match result {
        Err(msg) => { panic!(
            "atom {} forward_der_value error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!( range_der.len(), n_rng);
    //
    // var_der
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut var_der[res + dep_index], &mut range_der[rng_index] );
            dep_index += 1;
        }
    }
}
// --------------------------------------------------------------------------
// call_forward_der_ad
//
/// Call operator `AD<V>` evaluation of forward mode derivatives;
/// see [ForwardDer](crate::op::info::ForwardDer)
fn call_forward_der_ad<V> (
    adyp_both  : &Vec< AD<V> >       ,
    avar_both  : &Vec< AD<V> >       ,
    avar_der   : &mut Vec< AD<V> >   ,
    cop        : &Vec<V>             ,
    flag_all   : &Vec<bool>          ,
    arg        : &[IndexT]           ,
    arg_type   : &[ADType]           ,
    res        : usize               )
where
    V               : PartialEq + From<f32> + Clone + GlobalAtomCallbackVec ,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // forward_der_ad
    let forward_der_ad       = callback.forward_der_ad.clone();
    if forward_der_ad.is_none() {
        panic!(
            "{} : forward_der_ad is not implemented for this atomic function",
            callback.name,
        );
    }
    let forward_der_ad = forward_der_ad.unwrap();
    //
    // adomain
    let acop    = domain_acop(cop, arg, arg_type, n_dom);
    let adomain = domain_ad(
        adyp_both, avar_both, &acop, arg, arg_type, n_dom
    );
    //
    // adomain_der
    let zero_v : V = 0.0f32.into();
    let azero      = ad_from_value(zero_v);
    let mut adomain_der : Vec<& AD<V> > = Vec::with_capacity(n_dom);
    for i_dom in 0 .. n_dom {
        let index   = arg[BEGIN_DOM + i_dom] as usize;
        let ad_type = arg_type[BEGIN_DOM + i_dom].clone();
        if ad_type.is_variable() {
            adomain_der.push( &avar_der[index] );
        } else {
            adomain_der.push( &azero );
        }
    }
    // arange_der
    let result = forward_der_ad(
        &rng_is_dep, &adomain, &adomain_der, call_info, trace
    );
    let mut arange_der = match result {
        Err(msg) => { panic!(
            "atom {} forward_der_ad error : {}", callback.name, msg);
        },
        Ok(range) => range,
    };
    assert_eq!( arange_der.len(), n_rng);
    //
    // avar_der
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            swap( &mut avar_der[res + dep_index], &mut arange_der[rng_index] );
            dep_index += 1;
        }
    }
}
// ==========================================================================
// call_reverse_der_value
// ===========================================================================
//
// call_reverse_der_value
/// Call operator V evaluation of reverse mode derivatives;
/// see [ReverseDer](crate::op::info::ReverseDer)
fn call_reverse_der_value<V> (
    dyp_both   : &Vec<V>       ,
    var_both   : &Vec<V>       ,
    var_der    : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag_all   : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    for<'a> V       : PartialEq + GlobalAtomCallbackVec + AddAssign<&'a V>  + From<f32>,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // reverse_der_value
    let reverse_der_value = &callback.reverse_der_value;
    if reverse_der_value.is_none() {
        panic!(
            "{}: reverse_der_value not implemented for this atomic function",
            callback.name,
        );
    }
    let reverse_der_value = reverse_der_value.unwrap();
    //
    // domain
    let domain = domain_value(
        dyp_both, var_both, cop, arg, arg_type, n_dom
    );
    //
    // range_der
    let zero_v : V = 0f32.into();
    let mut range_der : Vec<&V> = vec![ &zero_v ; n_rng];
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            range_der[rng_index] = &var_der[res + dep_index];
            dep_index += 1;
        }
    }
    //
    // domain_der
    let result = reverse_der_value(&domain, range_der, call_info, trace);
    let domain_der = match result {
        Err(msg) => { panic!(
            "atom {} reverse_der_value error : {}", callback.name, msg);
        },
        Ok(domain) => domain,
    };
    assert_eq!( domain_der.len(), n_dom);
    //
    // var_der
    for i_arg in 0 .. n_dom {
        let index    = arg[BEGIN_DOM + i_arg] as usize;
        let ad_type  = arg_type[BEGIN_DOM + i_arg].clone();
        if ad_type.is_variable() {
            var_der[index] += &domain_der[i_arg];
        }
    }
}
// --------------------------------------------------------------------------
// call_reverse_der_ad
/// Call operator `AD<V>` evaluation of reverse mode derivatives;
/// see [ReverseDer](crate::op::info::ReverseDer)
fn call_reverse_der_ad<V> (
    adyp_both   : &Vec< AD<V> >       ,
    avar_both   : &Vec< AD<V> >       ,
    avar_der    : &mut Vec< AD<V> >   ,
    cop         : &Vec<V>             ,
    flag_all   : &Vec<bool>           ,
    arg        : &[IndexT]            ,
    arg_type   : &[ADType]            ,
    res        : usize                )
where
    V                 : PartialEq + GlobalAtomCallbackVec + Clone + From<f32>,
    for<'a> AD<V> : AddAssign<&'a AD<V> >,
    AtomCallback<V>   : Clone,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // reverse_der_ad
    let reverse_der_ad = &callback.reverse_der_ad;
    if reverse_der_ad.is_none() {
        panic!(
            "{}: reverse_der_ad not implemented for this atomic function",
            callback.name,
        );
    }
    let reverse_der_ad = reverse_der_ad.unwrap();
    //
    // adomain
    let acop     = domain_acop(cop, arg, arg_type, n_dom);
    let adomain  = domain_ad(
        adyp_both, avar_both, &acop, arg, arg_type, n_dom
    );
    //
    // sarange_der
    let zero_v : V = 0f32.into();
    let azero      = ad_from_value(zero_v);
    let mut arange_der : Vec<& AD<V> > = vec![ &azero ; n_rng];
    let mut dep_index = 0;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            arange_der[rng_index] = &avar_der[res + dep_index];
            dep_index += 1;
        }
    }
    //
    // adomain_der
    let result = reverse_der_ad(&adomain, arange_der, call_info, trace);
    let adomain_der = match result {
        Err(msg) => { panic!(
            "atom {} reverse_der_ad error : {}", callback.name, msg);
        },
        Ok(domain) => domain,
    };
    assert_eq!( adomain_der.len(), n_dom);
    //
    // avar_der
    for i_arg in 0 .. n_dom {
        let index   = arg[BEGIN_DOM + i_arg] as usize;
        let ad_type = arg_type[BEGIN_DOM + i_arg].clone();
        if ad_type.is_variable() {
            avar_der[index] += &adomain_der[i_arg];
        }
    }
}
// ---------------------------------------------------------------------------
//
// call_res_dyp
/// [ForwardDyp](crate::op::info::ForwardDyp) function for call result operator
fn call_res_dyp<V, E>(
    _dyp_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// call_res_var
/// [ForwardVar](crate::op::info::ForwardVar) function for call result operator
fn call_res_var<V, E>(
    _dyp_both : &Vec<E>     ,
    _var_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// call_res_der
/// [ForwardDer](crate::op::info::ForwardDer) or
/// [ReverseDer](crate::op::info::ReverseDer) function for call result operator
fn call_res_der<V, E>(
    _dyp_both : &Vec<E>     ,
    _var_both : &Vec<E>     ,
    _var_der  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// call_res_rust_src
/// [RustSrc](crate::op::info::RustSrc) function for call result operator
fn call_res_rust_src<V> (
    _not_used : V           ,
    _res_type  : ADType      ,
    _dyp_n_dom : usize       ,
    _var_n_dom : usize       ,
    _flag_all  : &Vec<bool>  ,
    _arg       : &[IndexT]   ,
    _arg_type  : &[ADType]   ,
    _res       : usize       ,
) -> String
{   String::new() }
// ===========================================================================
// call_rust_src
// ===========================================================================
/// Call operator rust source code generation;
/// see [RustSrc](crate::op::info::RustSrc) function
fn call_rust_src<V> (
    _not_used : V           ,
    res_type  : ADType      ,
    dyp_n_dom : usize       ,
    var_n_dom : usize       ,
    flag_all  : &Vec<bool>  ,
    arg       : &[IndexT]   ,
    arg_type  : &[ADType]   ,
    res       : usize       ) -> String
where
    V     : GlobalAtomCallbackVec,
    AtomCallback<V> : Clone,
{   // ----------------------------------------------------------------------
    debug_assert!( res_type.is_dynamic() || res_type.is_variable() );
    //
    let (
        atom_id,
        call_info,
        call_n_dom,
        n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback(atom_id);
    //
    // src
    let mut src = String::new();
    //
    // name
    let name = &callback.name;
    //
    // call_dom
    src = src +
        "   //\n" +
        "   // call_dom\n" +
        "   let mut call_dom : Vec<&V> = " +
                "vec![&nan; " + &(call_n_dom.to_string()) + "];\n";
    for j_dom in 0 .. call_n_dom {
        let mut index   = arg[BEGIN_DOM + j_dom] as usize;
        let ad_type     = arg_type[BEGIN_DOM + j_dom].clone();
        if ad_type.is_constant() {
            src = src + "   " +
                &format!("call_dom[{j_dom}] = &cop[{index}];\n");
        } else if ad_type.is_dynamic() {
            if index < dyp_n_dom {
                src = src + "   " +
                    &format!("call_dom[{j_dom}] = dyp_dom[{index}];\n");
            } else {
                index = index - dyp_n_dom;
                src = src + "   " +
                    &format!("call_dom[{j_dom}] = &dyp_dep[{index}];\n");
            }
        } else {
            debug_assert!( ad_type.is_variable() );
            if res_type.is_dynamic() {
                src = src + "   " +
                    &format!("call_dom[{j_dom}] = &nan;\n");
            } else if index < var_n_dom {
                src = src + "   " +
                    &format!("call_dom[{j_dom}] = var_dom[{index}];\n");
            } else {
                index = index - var_n_dom;
                src = src + "   " +
                    &format!("call_dom[{j_dom}] = &var_dep[{index}];\n");
            }
        }
    }
    //
    // use_range
    src = src +
        "   //\n" +
        "   let mut use_range : Vec<bool> = " +
            &format!("vec![false; {n_rng}];\n");
    for i_rng in 0 .. n_rng {
        if rng_is_dep[i_rng] {
            let rng_is_dep_i = rng_is_dep[i_rng];
            src = src + "    " +
                &format!( "use_range[{i_rng}] = {rng_is_dep_i};\n");
        }
    }
    //
    // call_range
    src = src +
        "   //\n" +
        "   // call_range\n" +
        "   let call_info  = " + &call_info.to_string() + ";\n" +
        "   let trace      = " + &trace.to_string() + ";\n" +
        "   let mut call_range = " +
            "atom_" + &name + "(&use_range, &call_dom, call_info, trace) ?;\n";
    //
    // res_name, res_dep
    let res_name   : &str;
    let res_dep    : usize;
    if res_type.is_dynamic() {
        res_name = "dyp_dep";
        assert!( dyp_n_dom <= res );
        res_dep  = res - dyp_n_dom;
    } else {
        res_name = "var_dep";
        assert!( var_n_dom <= res );
        res_dep  = res - var_n_dom;
    };
    //
    let mut dep_index = res_dep;
    for rng_index in 0 .. n_rng {
        if rng_is_dep[rng_index] {
            src = src + "   " +
                &format!( "std::mem::swap(\
                    &mut {res_name}[{dep_index}], &mut call_range[{rng_index}] \
                );\n");
            dep_index += 1;
        }
    }
    //
    // call_domain, call_range
    src = src +
        "   //\n" +
        "   // call_dom, call_range\n" +
        "   drop(call_dom);\n" +
        "   drop(call_range);\n" ;
    //
    src
}
// ===========================================================================
// set_op_info
// ===========================================================================
no_reverse_depend!(Call);
/// Set the operator information for call.
///
/// * op_info_vec :
/// The map from operator id to operator information [OpInfo] .
/// The map results for CALL_OP and CALL_RES_OP are set.
pub(crate) fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    V     : Clone + From<f32> + PartialEq + GlobalAtomCallbackVec + ThisThreadTapePublic,
    for<'a> V : AddAssign<&'a V> ,
{
    op_info_vec[CALL_OP as usize] = OpInfo{
        name              : "call" ,
        forward_dyp_value : call_forward_dyp_value::<V>,
        forward_dyp_ad    : call_forward_dyp_ad::<V>,
        forward_var_value : call_forward_var_value::<V>,
        forward_var_ad    : call_forward_var_ad::<V>,
        forward_der_value : call_forward_der_value::<V>,
        forward_der_ad    : call_forward_der_ad::<V>,
        reverse_der_value : call_reverse_der_value::<V>,
        reverse_der_ad    : call_reverse_der_ad::<V>,
        rust_src          : call_rust_src::<V>,
        reverse_depend    : reverse_depend_none::<V>,
    };
    op_info_vec[CALL_RES_OP as usize] = OpInfo{
        name              : "call_res" ,
        forward_dyp_value : call_res_dyp::<V, V>,
        forward_dyp_ad    : call_res_dyp::<V, AD<V> >,
        forward_var_value : call_res_var::<V, V>,
        forward_var_ad    : call_res_var::<V, AD<V> >,
        forward_der_value : call_res_der::<V, V>,
        forward_der_ad    : call_res_der::<V, AD<V> >,
        reverse_der_value : call_res_der::<V, V>,
        reverse_der_ad    : call_res_der::<V, AD<V> >,
        rust_src          : call_res_rust_src::<V>,
        reverse_depend    : reverse_depend_none::<V>,
    };
}
// ===========================================================================
// call_depend
// ===========================================================================
/// Determine which dynamic parameters, variables, and constants
///  a call operator's result depends on.
///
/// * op_index ;
/// This is a index in the operation sequence. The corresponding operator is
/// an CALL_OP or CALL_RES_OP.
///
/// * op_seq :
/// This is the operation sequence that call or call result operator
/// appears in.
///
/// * atom_depend :
/// Only the capacity of this vector matters
/// (it is passed in to avoid reallocating memory).
///
/// * cop_depend :
/// On input this vector has size zero.
/// Upon return it contains the set of constant parameter indices that
/// identifies the constamts that the op_index result depends on.
///
/// * dyp_depend :
/// On input this vector has size zero.
/// Upon return it contains the set of dynamic parameter indices that
/// identifies the dynamics that the op_index result depends on.
///
/// * var_depend :
/// On input this vector has size zero.
/// Upon return it contains the set of variables indices that
/// identifies the variables that the op_index result depends on.
///
pub(crate) fn call_depend<V>(
    atom_depend     : &mut Vec<usize> ,
    cop_depend      : &mut Vec<IndexT> ,
    dyp_depend      : &mut Vec<IndexT> ,
    var_depend      : &mut Vec<IndexT> ,
    op_seq          : &OpSequence     ,
    mut op_index    : usize           )
where
    V               : GlobalAtomCallbackVec,
    AtomCallback<V> : Clone,
{
    atom_depend.clear();
    debug_assert!( dyp_depend.len() == 0 );
    debug_assert!( var_depend.len() == 0 );
    //
    // id_all, op_id
    let id_all = &op_seq.id_all;
    let op_id  = id_all[op_index];
    debug_assert!( op_id == CALL_OP || op_id == CALL_RES_OP );
    //
    // arg_start, arg_all, arg_type_all, flag_all
    let arg_start       = &op_seq.arg_start;
    let arg_all         = &op_seq.arg_all;
    let arg_type_all    = &op_seq.arg_type_all;
    let flag_all        = &op_seq.flag_all;
    //
    // op_index, dep_index
    let begin           = arg_start[op_index] as usize;
    let dep_index : usize ;
    if op_id == CALL_RES_OP {
        dep_index   = arg_all[begin] as usize;
        debug_assert!( 0 < dep_index );
        debug_assert!( dep_index <= op_index );
        op_index    = op_index - dep_index;
        debug_assert!( id_all[op_index] == CALL_OP );
    } else {
        dep_index = 0;
    }
    debug_assert!( id_all[op_index] == CALL_OP );
    //
    // arg, arg_type
    let begin    = arg_start[op_index] as usize;
    let end      = arg_start[op_index + 1] as usize;
    let arg      = &arg_all[begin .. end];
    let arg_type = &arg_type_all[begin .. end];
    //
    // callback, n_dom, call_info, trace
    let (
        atom_id,
        call_info,
        n_dom,
        _n_rng,
        trace,
        rng_is_dep,
    ) = extract_call_info(arg, flag_all);
    let callback = get_callback::<V>(atom_id);
    //
    // rng_index
    let mut dep_count = 0;
    let mut rng_index = 0;
    while dep_count < dep_index || ! rng_is_dep[rng_index] {
        if rng_is_dep[rng_index] {
            dep_count += 1;
        }
        rng_index += 1;
    }
    assert_eq!(dep_count, dep_index);
    assert!( rng_is_dep[rng_index] );
    //
    // rev_depend
    let rev_depend = &callback.rev_depend;
    if rev_depend.is_none() {
        panic!(
            "{} : rev_depend is not implemented for this atomic function",
            callback.name,
        );
    }
    let rev_depend = rev_depend.unwrap();
    //
    // atom_depend
    let error_msg = rev_depend(
        atom_depend, rng_index, n_dom, call_info, trace
    );
    if error_msg != "" {
        panic!(
            "{} : rev_depend error_msg = {}", callback.name, error_msg
        );
    }
    //
    // dyp_depend, var_depend
    for k in 0 .. atom_depend.len() {
        if n_dom <= atom_depend[k] {
            panic!(
                "atom {} rev_depend : \
                rng_index   = {},
                n_dom = {}, \
                k = {} \
                depend[k] = {} >= n_dom",
                callback.name, rng_index, n_dom, k, atom_depend[k]
           );
        }
        let arg_index = BEGIN_DOM + atom_depend[k];
        match arg_type[arg_index] {
            ADType::ConstantP => cop_depend.push( arg[arg_index] ),
            ADType::DynamicP  => dyp_depend.push( arg[arg_index] ),
            ADType::Variable  => var_depend.push( arg[arg_index] ),
            _                 => { },
        }
    }
}
