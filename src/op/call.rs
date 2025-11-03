// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// --------------------------------------------------------------------------
//! Operator that calls an atomic function
//!
//! Link to [parent module](super)
//!
//! # TODO
//! Remove the argument flags because arg_type yields the type of each argument.
//! In addition, change the result flags to ADType for the results.
//!
//! # Operator Id
//!  CALL_OP
//!
//! # Operator Arguments
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index that identifies the atomic function; i.e., atom_id |
//! | 1        | Extra information about this call; i.e. call_info        |
//! | 2        | Domain space dimension for function being called (n_dom) |
//! | 3        | Number of results for the function being called  (n_res) |
//! | 4        | Index of the first boolean for this operator             |
//! | 5        | Variable or parameter index for first argument to call   |
//! | 6        | Variable or parameter index for second argument to call  |
//! | ...      | ... |
//! | 4+n_dom  | Variable or parameter index for last argument to call    |
//!
//! # Operator Booleans
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | is the value of the trace argument of this call            |
//! | 1        | true (false) if first argument is a variable (parameter)   |
//! | 2        | true (false) if second argument is a variable (parameter)  |
//! | ...      | ... |
//! | n_dom    | true (false) if last argument is a variable (parameter)    |
//! | n_dom+1  | true (false) if first result is a variable (parameter)     |
//! | n_dom+2  | true (false) if second result is a variable (parameter)    |
//! | n_dom+n_res | true (false) if last result is a variable (parameter)   |
//!
//! # Operator Results
//! We use n_res for the number of results that are variables.
//! There are n_res - 1 CALL_RES_OP directly after each CALL_OP
//! operator in the sequence of operations. These are place holders so that
//! there is a direct correspondence between variable, or dynamic parameter,
//! and operator indices.
// --------------------------------------------------------------------------
// use
//
use std::sync::RwLock;
use std::cmp::PartialEq;
//
use crate::op::info::OpInfo;
use crate::atom::{
    sealed::AtomEvalVec,
};
use crate::op::id::{
        CALL_OP,
        CALL_RES_OP,
};
use crate::op::info::{
    no_forward_dyp_ad,
};
use crate::{
    AD,
    ADType,
    IndexT,
    AtomEval,
    ThisThreadTapePublic,
    ad_from_value,
};
// ----------------------------------------------------------------------
//
// extract_call_info
fn extract_call_info<V>(
    arg        : &[IndexT] ,
    arg_type   : &[ADType] ,
    flag       : &[bool]   ,
) -> (
    IndexT       , // call_info
    usize        , // n_dom
    usize        , // n_res
    bool         , // trace
    AtomEval<V>  , // atom_eval
    Vec<ADType>  , // res_ad_type
)
where
    V           : AtomEvalVec,
    AtomEval<V> : Clone,
{
    // atom_id, call_info, n_dom, n_res,
    let atom_id    = arg[0] as usize;
    let call_info  = arg[1];
    let n_dom      = arg[2] as usize;
    let n_res      = arg[3] as usize;
    let trace      = flag[ arg[4] as usize ];
    //
    // atom_eval
    let atom_eval : AtomEval<V>;
    {   //
        // rw_lock
        let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec  = read_lock.unwrap();
        atom_eval          = atom_eval_vec[atom_id].clone();
    }
    //
    // res_ad_type
    let forward_type = &atom_eval.forward_type;
    let dom_ad_type  = &arg_type[5 .. 5 + n_dom];
    let res_ad_type  = forward_type(dom_ad_type, call_info, trace);
    assert_eq!(
        n_res,
        res_ad_type.len(),
        "atom {} forward_type return length: expected {}, found {}",
        atom_eval.name,
        n_res,
        res_ad_type.len(),
    );
    //
    (
        call_info,
        n_dom,
        n_res,
        trace,
        atom_eval,
        res_ad_type,
    )
}
//
// extract_call_info_old
fn extract_call_info_old<'a, V>(
    arg        : &'a [IndexT]  ,
    flag       : &'a Vec<bool> ,
) -> (
    IndexT       , // call_info
    usize        , // n_dom
    usize        , // n_res
    bool         , // trace
    &'a [bool]   , // is_arg_var
    &'a [bool]   , // is_res_var
    AtomEval<V>  , // atom_eval
)
where
    V           : AtomEvalVec,
    AtomEval<V> : Clone,
{
    // atom_id, call_info, n_dom, n_res,
    let atom_id    = arg[0] as usize;
    let call_info  = arg[1];
    let n_dom      = arg[2] as usize;
    let n_res      = arg[3] as usize;
    let trace      = flag[ arg[4] as usize ];
    //
    // is_arg_var, is_res_var
    let mut begin  = (arg[4] as usize) + 1;
    let mut end     = begin + n_dom;
    let is_arg_var  = &flag[begin .. end];
    begin           = end;
    end             = begin + n_res;
    let is_res_var  = &flag[begin .. end];
    //
    // atom_eval
    let atom_eval : AtomEval<V>;
    {   //
        // rw_lock
        let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec  = read_lock.unwrap();
        atom_eval          = atom_eval_vec[atom_id].clone();
    }
    (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    )
}
// ----------------------------------------------------------------------
//
// domain_zero_value
fn domain_zero_value<'a, 'b, V>(
    dyp_zero   : &'a [V]       ,
    var_zero   : &'a [V]       ,
    cop        : &'a [V]       ,
    arg        : &'b [IndexT]  ,
    arg_type   : &'b [ADType]  ,
    n_dom      : usize         ,
) -> Vec<&'a V>
where
    V : PartialEq,
{   //
    // no_var_zero
    // This case is used during zero forward mode for dynamic parameters.
    // If no_var_zero, then var_zero[0] is nan.
    let no_var_zero = var_zero.len() == 1 && var_zero[0] != var_zero[0];
    //
    let mut domain_zero : Vec<&V> = Vec::with_capacity( n_dom );
    for j_arg in 0 .. n_dom {
        let index   = arg[5 + j_arg] as usize;
        let ad_type = &arg_type[5 + j_arg];
        if ad_type.is_constant() {
            domain_zero.push( &cop[index] );
        } else if ad_type.is_dynamic() {
            domain_zero.push( &dyp_zero[index] );
        } else {
            debug_assert!( ad_type.is_variable() );
            if no_var_zero {
                domain_zero.push( &var_zero[0] );
            } else {
                domain_zero.push( &var_zero[index] );
            }
        }
    }
    domain_zero
}
//
// call_domain_zero_value
fn call_domain_zero_value<'a, 'b, V>(
    var_zero   : &'a Vec<V>    ,
    cop        : &'a Vec<V>    ,
    arg        : &'b [IndexT]  ,
    n_dom      : usize         ,
    is_arg_var : &'b [bool]    ,
) -> Vec<&'a V>
{
    //
    let mut call_domain_zero : Vec<&V> = Vec::with_capacity( n_dom );
    for i_arg in 0 .. n_dom {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_domain_zero.push( &var_zero[index] );
        } else {
            call_domain_zero.push( &cop[index] );
        }
    }
    call_domain_zero
}
// ----------------------------------------------------------------------
//
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
        let ad_type = &arg_type[5 + i_arg];
        if ad_type.is_constant() {
            let index = arg[5 + i_arg] as usize;
            acop.push( ad_from_value( cop[index].clone() ) );
        }
    }
    acop
}
//
// call_domain_acop
fn call_domain_acop<'a, 'b, V>(
    cop        : &'a Vec<V>    ,
    arg        : &'b [IndexT]  ,
    n_dom      : usize         ,
    is_arg_var : &'b [bool]    ,
) -> Vec< AD<V> >
where
    V : Clone,
{
    //
    let mut acop : Vec< AD<V> > = Vec::new();
    for i_arg in 0 .. n_dom {
        if ! is_arg_var[i_arg] {
            let index = arg[i_arg + 5] as usize;
            acop.push( ad_from_value( cop[index].clone() ) );
        }
    }
    acop
}
// ----------------------------------------------------------------------
//
// domain_zero_ad
fn domain_zero_ad<'a, 'b, V>(
    dyp_zero   : &'a [AD<V>]    ,
    var_zero   : &'a [AD<V>]    ,
    acop       : &'a [AD<V>]    ,
    arg        : &'b [IndexT]   ,
    arg_type   : &'b [ADType]   ,
    n_dom      : usize          ,
) -> Vec<&'a AD<V> >
{
    //
    let mut domain_zero : Vec<& AD<V> > = Vec::with_capacity( n_dom );
    let mut j_cop : usize = 0;
    for j_arg in 0 .. n_dom {
        let index   = arg[5 + j_arg] as usize;
        let ad_type = &arg_type[5 + j_arg];
        if ad_type.is_constant() {
            domain_zero.push( &acop[j_cop] );
            j_cop += 1;
        } else if ad_type.is_dynamic() {
            domain_zero.push( &dyp_zero[index] );
        } else {
            debug_assert!( ad_type.is_variable() );
            domain_zero.push( &var_zero[index] );
        }
    }
    domain_zero
}
//
// call_domain_zero_ad
fn call_domain_zero_ad<'a, 'b, V>(
    avar_zero   : &'a Vec< AD<V> >    ,
    acop        : &'a Vec< AD<V> >    ,
    arg         : &'b [IndexT]        ,
    n_dom      : usize                ,
    is_arg_var : &'b [bool]           ,
) -> Vec<&'a AD<V> >
{
    //
    let mut call_domain_zero : Vec<& AD<V> > = Vec::with_capacity( n_dom );
    let mut i_cop : usize = 0;
    for i_arg in 0 .. n_dom {
        if is_arg_var[i_arg] {
            let index = arg[i_arg + 5] as usize;
            call_domain_zero.push( &avar_zero[index] );
        } else {
            call_domain_zero.push( &acop[i_cop] );
            i_cop += 1;
        }
    }
    call_domain_zero
}
// ==========================================================================
// call_forward_dyp
// ==========================================================================
//
// call_forward_value
/// atomic function callback for V evaluation of variables.
fn call_forward_dyp_value<V> (
    dyp_zero   : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    V           : AtomEvalVec + From<f32> + PartialEq,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        atom_eval,
        res_ad_type,
    ) = extract_call_info(arg, arg_type, flag);
    //
    // forward_zero_value
    let forward_zero_value = &atom_eval.forward_zero_value;
    if forward_zero_value.is_none() {
        panic!(
        "{} : forward_zero_value is not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_zero_value = forward_zero_value.unwrap();
    //
    // domain_zero
    let nan_v    : V      = f32::NAN.into();
    let var_zero : Vec<V> = vec![ nan_v ];
    let domain_zero = domain_zero_value(
        dyp_zero, &var_zero, cop, arg, arg_type, n_dom
    );
    //
    // range_zero
    let mut range_zero = forward_zero_value(
        &domain_zero, call_info, trace
    );
    assert_eq!(
        n_res,
        range_zero.len(),
        "atom {} forward_zero_value return length: expected {}, found {}",
        atom_eval.name,
        n_res,
        range_zero.len(),
    );
    //
    // dyp_zero
    let mut j_res = 0;
    range_zero.reverse();
    for i_res in (0 .. n_res).rev() {
        let ad_type_i = &res_ad_type[i_res];
        let range_i   = range_zero.pop();
        debug_assert!( range_i.is_some() );
        if ad_type_i.is_dynamic() {
            dyp_zero[res + j_res] = range_i.unwrap();
            j_res += 1;
        }
    }
}
// ==========================================================================
// call_forward_var
// ==========================================================================
//
// call_forward_value
/// atomic function callback for V evaluation of variables.
fn call_forward_var_value<V> (
    dyp_zero   : &Vec<V>       ,
    var_zero   : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    arg_type   : &[ADType]     ,
    res        : usize         )
where
    V           : AtomEvalVec + PartialEq,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        atom_eval,
        res_ad_type,
    ) = extract_call_info(arg, arg_type, flag);
    //
    // forward_zero_value
    let forward_zero_value = &atom_eval.forward_zero_value;
    if forward_zero_value.is_none() {
        panic!(
        "{} : forward_zero_value is not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_zero_value = forward_zero_value.unwrap();
    //
    // domain_zero
    let domain_zero = domain_zero_value(
        dyp_zero, var_zero, cop, arg, arg_type, n_dom
    );
    //
    // range_zero
    let mut range_zero = forward_zero_value(
        &domain_zero, call_info, trace
    );
    assert_eq!(
        n_res,
        range_zero.len(),
        "atom {} forward_zero_value return length: expected {}, found {}",
        atom_eval.name,
        n_res,
        range_zero.len(),
    );
    //
    // var_zero
    let mut j_res = 0;
    range_zero.reverse();
    for i_res in (0 .. n_res).rev() {
        let ad_type_i = &res_ad_type[i_res];
        let range_i   = range_zero.pop();
        debug_assert!( range_i.is_some() );
        if ad_type_i.is_variable() {
            var_zero[res + j_res] = range_i.unwrap();
            j_res += 1;
        }
    }
}
//
// call_forward_var_ad
/// atomic function callback for `AD<V>` evaluation of variables.
fn call_forward_var_ad<V> (
    adyp_zero  : &Vec< AD<V> >       ,
    avar_zero  : &mut Vec< AD<V> >   ,
    cop        : &Vec<V>             ,
    flag       : &Vec<bool>          ,
    arg        : &[IndexT]           ,
    arg_type   : &[ADType]           ,
    res        : usize               )
where
    V           : Clone + AtomEvalVec,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        atom_eval,
        res_ad_type,
    ) = extract_call_info(arg, arg_type, flag);
    //
    // forward_zero_ad
    let forward_zero_ad = &atom_eval.forward_zero_ad;
    if forward_zero_ad.is_none() {
        panic!(
            "{} : forward_zero_ad is not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_zero_ad = forward_zero_ad.unwrap();
    //
    // domain_zero
    let acop = domain_acop(cop, arg, arg_type, n_dom);
    let adomain_zero = domain_zero_ad(
        adyp_zero, avar_zero, &acop, arg, arg_type, n_dom
    );
    //
    // arange_zero
    let mut arange_zero = forward_zero_ad(
        &adomain_zero, call_info, trace
    );
    assert_eq!(
        n_res,
        arange_zero.len(),
        "atom {} forward_zero_ad return length: expected {}, found {}",
        atom_eval.name,
        n_res,
        arange_zero.len(),
    );
    //
    // avar_zero
    let mut j_res = 0;
    arange_zero.reverse();
    for i_res in (0 .. n_res).rev() {
        let ad_type_i = &res_ad_type[i_res];
        let arange_i  = arange_zero.pop();
        debug_assert!( arange_i.is_some() );
        if ad_type_i.is_variable() {
            avar_zero[res + j_res] = arange_i.unwrap();
            j_res += 1;
        }
    }
}
// ==========================================================================
// call_forward_1_value
//
/// V evaluation of first order forward call operator for atomic functions
fn call_forward_1_value<V> (
    var_zero   : &Vec<V>       ,
    var_one    : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    V           : AtomEvalVec + From<f32>,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    ) = extract_call_info_old(arg, flag);
    //
    // forward_zero_value, forward_one_value
    let forward_zero_value = &atom_eval.forward_zero_value;
    let forward_one_value  = &atom_eval.forward_one_value;
    if forward_zero_value.is_none() {
        panic!(
            "{} : forward_zero_value not implemented for this atomic function",
            atom_eval.name,
        );
    }
    if forward_one_value.is_none() {
        panic!(
            "{} : forward_one_value not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_zero_value = forward_zero_value.unwrap();
    let forward_one_value  = forward_one_value.unwrap();
    //
    // call_domain_zero
    let call_domain_zero = call_domain_zero_value(
        var_zero, cop, arg, n_dom, is_arg_var
    );
    // ----------------------------------------------------------------------
    //
    // call_var_zero
    forward_zero_value(&call_domain_zero, call_info, trace);
    //
    // call_domain_one
    let zero_v : V = 0f32.into();
    let mut call_domain_one : Vec<&V> = Vec::with_capacity( n_dom );
    for i_arg in 0 .. n_dom {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_domain_one.push( &var_one[index] );
        } else {
            call_domain_one.push( &zero_v );
        }
    }
    // call_range_one
    let mut call_range_one = forward_one_value(
        &call_domain_zero, call_domain_one, call_info, trace
    );
    assert_eq!( call_range_one.len(), n_res);
    //
    // var_one
    let mut j_res = 0;
    call_range_one.reverse();
    for i_res in 0 .. n_res {
        let range_i = call_range_one.pop();
        debug_assert!( range_i.is_some() );
        if is_res_var[i_res] {
            var_one[res + j_res] = range_i.unwrap();
            j_res += 1;
        }
    }
}
// --------------------------------------------------------------------------
// call_reverse_1_value
//
/// V evaluation of first order reverse call operator for atomic functions
fn call_reverse_1_value<V> (
    var_zero   : &Vec<V>       ,
    var_one    : &mut Vec<V>   ,
    cop        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    for<'a> V   : AtomEvalVec + std::ops::AddAssign<&'a V>  + From<f32>,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    ) = extract_call_info_old(arg, flag);
    //
    // reverse_one_value
    let reverse_one_value = &atom_eval.reverse_one_value;
    if reverse_one_value.is_none() {
        panic!(
            "{}: reverse_one_value not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let reverse_one_value = reverse_one_value.unwrap();
    //
    // call_domain_zero
    let call_domain_zero = call_domain_zero_value(
        var_zero, cop, arg, n_dom, is_arg_var
    );
    //
    // call_range_one
    let zero_v : V = 0f32.into();
    let mut call_range_one : Vec<&V> = Vec::with_capacity( n_res );
    let mut j_res = 0;
    for i_res in 0 .. n_res {
        if is_res_var[i_res] {
            call_range_one.push( &var_one[res + j_res] );
            j_res += 1;
        } else {
            call_range_one.push( &zero_v );
        }
    }
    // call_domain_one
    let call_domain_one = reverse_one_value(
        &call_domain_zero, call_range_one, call_info, trace
    );
    assert_eq!( call_domain_one.len(), n_dom);
    //
    // var_one
    for i_arg in 0 .. n_dom {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            var_one[index] += &call_domain_one[i_arg];
        }
    }
}
// --------------------------------------------------------------------------
// call_forward_1_ad
//
/// `AD<V>` evaluation of first order forward call operator for atomic functions
fn call_forward_1_ad<V> (
    avar_zero  : &Vec< AD<V> >       ,
    avar_one   : &mut Vec< AD<V> >   ,
    cop        : &Vec<V>             ,
    flag       : &Vec<bool>          ,
    arg        : &[IndexT]           ,
    res        : usize               )
where
    V           : From<f32> + Clone + AtomEvalVec ,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    ) = extract_call_info_old(arg, flag);
    //
    // forward_zero_ad, forward_one_ad
    let forward_zero_ad      = atom_eval.forward_zero_ad.clone();
    let forward_one_ad       = atom_eval.forward_one_ad.clone();
    if forward_zero_ad.is_none() {
        panic!(
            "{} : forward_zero_ad is not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_zero_ad = forward_zero_ad.unwrap();
    if forward_one_ad.is_none() {
        panic!(
            "{} : forward_one_ad is not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let forward_one_ad = forward_one_ad.unwrap();
    //
    // call_adomain_zero
    let acop = call_domain_acop(cop, arg, n_dom, is_arg_var);
    let call_adomain_zero = call_domain_zero_ad(
        avar_zero, &acop, arg, n_dom, is_arg_var
    );
    //
    // call_avar_zero
    forward_zero_ad(&call_adomain_zero, call_info, trace);
    //
    // call_adomain_one
    let zero_v : V = 0.0f32.into();
    let azero      = ad_from_value(zero_v);
    let mut call_adomain_one : Vec<& AD<V> > = Vec::with_capacity(n_dom);
    for i_arg in 0 .. n_dom {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_adomain_one.push( &avar_one[index] );
        } else {
            call_adomain_one.push( &azero );
        }
    }
    // call_arange_one
    let mut call_arange_one = forward_one_ad(
        &call_adomain_zero, call_adomain_one, call_info, trace
    );
    assert_eq!( call_arange_one.len(), n_res);
    //
    // avar_one
    let mut j_res = 0;
    call_arange_one.reverse();
    for i_res in 0 .. n_res {
        let arange_i = call_arange_one.pop();
        debug_assert!( arange_i.is_some() );
        if is_res_var[i_res] {
            avar_one[res + j_res] = arange_i.unwrap();
            j_res += 1;
        }
    }
}
// --------------------------------------------------------------------------
// call_reverse_1_ad
//
/// `AD<V>` evaluation of first order reverse call operator (atomic functions)
fn call_reverse_1_ad<V> (
    avar_zero   : &Vec< AD<V> >       ,
    avar_one    : &mut Vec< AD<V> >   ,
    cop         : &Vec<V>             ,
    flag       : &Vec<bool>           ,
    arg        : &[IndexT]            ,
    res        : usize                )
where
    V             : AtomEvalVec + Clone + From<f32>,
    for<'a> AD<V> : std::ops::AddAssign<&'a AD<V> >,
    AtomEval<V>   : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    ) = extract_call_info_old(arg, flag);
    //
    // reverse_one_ad
    let reverse_one_ad = &atom_eval.reverse_one_ad;
    if reverse_one_ad.is_none() {
        panic!(
            "{}: reverse_one_ad not implemented for this atomic function",
            atom_eval.name,
        );
    }
    let reverse_one_ad = reverse_one_ad.unwrap();
    //
    // call_adomain_zero
    let acop = call_domain_acop(cop, arg, n_dom, is_arg_var);
    let call_adomain_zero = call_domain_zero_ad(
        avar_zero, &acop, arg, n_dom, is_arg_var
    );
    //
    // call_range_one
    let zero_v : V    = 0f32.into();
    let azero         = ad_from_value(zero_v);
    let mut call_arange_one : Vec<& AD<V>> = Vec::with_capacity( n_res );
    let mut j_res = 0;
    for i_res in 0 .. n_res {
        if is_res_var[i_res] {
            call_arange_one.push( &avar_one[res + j_res] );
            j_res += 1;
        } else {
            call_arange_one.push( &azero );
        }
    }
    // call_adomain_one
    let call_adomain_one = reverse_one_ad(
        &call_adomain_zero, call_arange_one, call_info, trace
    );
    assert_eq!( call_adomain_one.len(), n_dom);
    //
    // avar_one
    for i_arg in 0 .. n_dom {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            avar_one[index] += &call_adomain_one[i_arg];
        }
    }
}
// --------------------------------------------------------------------------
//
// call_arg_var_index
/// vector of variable indices that are arguments to this call operator
fn call_arg_var_index(
    arg_var_index : &mut Vec<IndexT>,
    flag          : &Vec<bool>,
    arg           : &[IndexT]
)
{
    //
    // n_dom
    let n_dom = arg[2] as usize;
    //
    // is_var
    let begin    = arg[3] as usize;
    let end      = begin + n_dom;
    let is_var   = &flag[begin .. end];
    //
    // arg_var_index
    let zero_t = 0 as IndexT;
    arg_var_index.resize(0, zero_t);
    for call_i_arg in 0 .. n_dom {
        if is_var[call_i_arg] {
            arg_var_index.push( arg[5 + call_i_arg]  );
        }
    }
    assert_ne!( arg_var_index.len() , 0 );
}
// ---------------------------------------------------------------------------
//
// set_op_info
no_forward_dyp_ad!(Call);
//
/// Set the operator information for call.
///
/// * op_info_vec :
/// The map from operator id to operator information [OpInfo] .
/// The map results for CALL_OP and CALL_RES_OP are set.
pub(crate) fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    V : Clone + From<f32> + PartialEq + AtomEvalVec + ThisThreadTapePublic,
    for<'a> V : std::ops::AddAssign<&'a V> ,
{
    op_info_vec[CALL_OP as usize] = OpInfo{
        name              : "call" ,
        forward_dyp_value : call_forward_dyp_value::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_var_value : call_forward_var_value::<V>,
        forward_var_ad    : call_forward_var_ad::<V>,
        forward_1_value   : call_forward_1_value::<V>,
        forward_1_ad      : call_forward_1_ad::<V>,
        reverse_1_value   : call_reverse_1_value::<V>,
        reverse_1_ad      : call_reverse_1_ad::<V>,
        rust_src          : call_rust_src::<V>,
        arg_var_index     : call_arg_var_index,
    };
    op_info_vec[CALL_RES_OP as usize] = OpInfo{
        name              : "call_res" ,
        forward_dyp_value : no_op_dyp::<V, V>,
        forward_dyp_ad    : no_op_dyp::<V, AD<V> >,
        forward_var_value : no_op_var::<V, V>,
        forward_var_ad    : no_op_var::<V, AD<V> >,
        forward_1_value   : no_op_one::<V, V>,
        forward_1_ad      : no_op_one::<V, AD<V> >,
        reverse_1_value   : no_op_one::<V, V>,
        reverse_1_ad      : no_op_one::<V, AD<V> >,
        rust_src          : no_op_rust_src::<V>,
        arg_var_index     : no_op_arg_var_index,
    };
}
// ---------------------------------------------------------------------------
//
// no_op_dyp
/// [ForwardDyp](crate::op::info::ForwardDyp) function
fn no_op_dyp<V, E>(
    _dyp_zero : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// no_op_var
/// [ForwardVar](crate::op::info::ForwardVar) function
fn no_op_var<V, E>(
    _dyp_zero : &Vec<E>     ,
    _var_zero : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// no_op_one
/// [ForwardOne](crate::op::info::ForwardOne) or
/// [ReverseOne](crate::op::info::ReverseOne) function
fn no_op_one<V, E>(
    _var_zero : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { }
//
// no_op_arg_var_index
/// [ArgVarIndex](crate::op::info::ArgVarIndex) function
fn no_op_arg_var_index(
    arg_var_index  : &mut Vec<IndexT> ,
    _flag          : &Vec<bool>       ,
    _arg           : &[IndexT]        ,
) {
    let zero_t = 0 as IndexT;
    arg_var_index.resize(0, zero_t);
}
//
// no_op_rust_src
fn no_op_rust_src<V> (
    _not_used  : V             ,
    _n_domain  : usize         ,
    _flag      : &Vec<bool>    ,
    _arg       : &[IndexT]     ,
    _res       : usize         ,
) -> String
{   String::new() }
// --------------------------------------------------------------------------
// call_rust_src
//
/// Rust source code for the call operator.
fn call_rust_src<V> (
    _not_used  : V             ,
    n_domain   : usize         ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         ) -> String
where
    V : AtomEvalVec,
    AtomEval<V> : Clone,
{   // ----------------------------------------------------------------------
    let (
        call_info,
        n_dom,
        n_res,
        trace,
        is_arg_var,
        is_res_var,
        atom_eval,
    ) = extract_call_info_old(arg, flag);
    //
    // src
    let mut src = String::new();
    //
    // name
    let name = &atom_eval.name;
    //
    // call_domain
    src = src +
        "   //\n" +
        "   // call_domain\n" +
        "   let mut call_domain : Vec<&V> = " +
                "vec![&nan; " + &(n_dom.to_string()) + "];\n";
    for i_arg in 0 .. n_dom {
        let i_str = i_arg.to_string();
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            if index < n_domain {
                let j_str = index.to_string();
                src = src +
                "   call_domain[" + &i_str + "] = &domain[" + &j_str + "];\n";
            } else {
                let j_str = (index - n_domain).to_string();
                src = src +
                "   call_domain[" + &i_str + "] = &dep[" + &j_str + "];\n";
            }
        } else {
            let j_str = index.to_string();
            src = src +
                "   call_domain[" + &i_str + "] = &cop[" + &j_str + "];\n";
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
                "atom_" + &name + "(&call_domain, call_info, trace);\n";
    // dep
    assert!(n_domain <= res);
    src = src +
        "   //\n" +
        "   // dep\n" +
        "   call_range.reverse();\n";
    let j_res = 0;
    for i_res in 0 .. n_res {
        if is_res_var[i_res] {
            let j_str = (res + j_res - n_domain).to_string();
            src = src +
                "   dep[" + &j_str + "] = call_range.pop().unwrap();\n";
        } else {
            src = src +
                "   call_range.pop();\n";
        }
    }
    //
    // call_domain, call_range
    src = src +
        "   //\n" +
        "   // call_domain, call_range\n" +
        "   drop(call_domain);\n" +
        "   drop(call_range);\n" ;
    //
    src
}
