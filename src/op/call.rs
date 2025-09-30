// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// --------------------------------------------------------------------------
//! Operator that calls an atomic function
//!
//! Link to [parent module](super)
//!
//! # Operator Id
//!  CALL_OP
//!
//! # Operator Arguments
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index that identifies the atomic function; i.e., atom_id |
//! | 1        | Extra information about this call; i.e. call_info        |
//! | 2        | Number of arguments to the function being called (n_arg) |
//! | 3        | Number of results for the function being called  (n_res) |
//! | 4        | Index of the first boolean for this operator             |
//! | 5        | Variable or constant index for first argument to call    |
//! | 6        | Variable or constant index for second argument to call   |
//! | ...      | ... |
//! | 4+n_arg  | Variable or constant index for last argument to call     |
//!
//! # Operator Booleans
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | is the value of the trace argument of this call           |
//! | 1        | true (false) if first argument is a variable (constant)   |
//! | 2        | true (false) if second argument is a variable (constant)  |
//! | ...      | ... |
//! | n_arg    | true (false) if last argument is a variable (constant)    |
//! | n_arg+1  | true (false) if first result is a variable (constant)     |
//! | n_arg+2  | true (false) if second result is a variable (constant)    |
//! | n_arg+n_res | true (false) if last result is a variable (constant)   |
//!
//! # Operator Results
//! We use n_var_res for the number of results that are variables.
//! There are n_var_res - 1 CALL_RES_OP operators directly after each CALL_OP
//! operator in the sequence of operations. These are place holders so that
//! there is a direct correpondence between variable and operator indices.
// --------------------------------------------------------------------------
// use
//
use std::sync::RwLock;
//
use crate::op::info::{
    OpInfo,
    panic_zero,
    panic_one,
};
use crate::atom::{
    AtomForwardZeroValue,
    AtomForwardOneValue,
    AtomReverseOneValue,
    sealed::AtomEvalVec,
};
use crate::op::id::{
        CALL_OP,
        CALL_RES_OP,
};
use crate::{
    AD,
    IndexT,
    AtomEval,
};
// ----------------------------------------------------------------------
fn extract_call_arg<'a, 'b, V>(
    var_zero   : &'a Vec<V>    ,
    con        : &'a Vec<V>    ,
    flag       : &'b Vec<bool> ,
    arg        : &'b [IndexT]  ,
) -> (
    usize      , // atom_id
    IndexT     , // call_info
    usize      , // call_n_arg
    usize      , // call_n_res
    bool       , // trace
    &'b [bool] , // is_arg_var
    &'b [bool] , // is_res_var
    Vec<&'a V> , // call_domain_zero
) {
    let atom_id    = arg[0] as usize;
    let call_info  = arg[1];
    let call_n_arg = arg[2] as usize;
    let call_n_res = arg[3] as usize;
    let trace      = flag[ arg[4] as usize ];
    //
    let mut begin  = (arg[4] as usize) + 1;
    let mut end     = begin + call_n_arg;
    let is_arg_var  = &flag[begin .. end];
    begin           = end;
    end             = begin + call_n_res;
    let is_res_var  = &flag[begin .. end];
    //
    let mut call_domain_zero : Vec<&V> = Vec::with_capacity( call_n_arg );
    for i_arg in 0 .. call_n_arg {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_domain_zero.push( &var_zero[index] );
        } else {
            call_domain_zero.push( &con[index] );
        }
    }
    //
    (
        atom_id,
        call_info,
        call_n_arg,
        call_n_res,
        trace,
        is_arg_var,
        is_res_var,
        call_domain_zero
    )
}
// --------------------------------------------------------------------------
// call_forward_0_value
//
/// zero order forward call operator for atomic functions
fn call_forward_0_value<V> (
    var_zero   : &mut Vec<V>   ,
    con        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    V : AtomEvalVec,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        _call_n_arg,
        call_n_res,
        trace,
        _is_arg_var,
        is_res_var,
        call_domain_zero,
    ) = extract_call_arg(var_zero, con, flag, arg);
    // ----------------------------------------------------------------------
    //
    // forward_zero
    let forward_zero : AtomForwardZeroValue<V>;
    {   //
        // rw_lock
        let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval = &atom_eval_vec[atom_id];
        forward_zero  = atom_eval.forward_zero_value.clone();
    }
    //
    // call_range_zero
    let mut call_var_zero  : Vec<V> = Vec::new();
    let mut call_range_zero = forward_zero(
        &mut call_var_zero, call_domain_zero, call_info, trace
    );
    //
    // var_zero
    let mut j_res = 0;
    call_range_zero.reverse();
    for i_res in (0 .. call_n_res).rev() {
        let range_i = call_range_zero.pop();
        debug_assert!( range_i.is_some() );
        if is_res_var[i_res] {
            var_zero[res + j_res] = range_i.unwrap();
            j_res += 1;
        }
    }
}
// --------------------------------------------------------------------------
// call_forward_1_value
//
/// first order forward call operator for atomic functions
fn call_forward_1_value<V> (
    var_zero   : &Vec<V>       ,
    var_one    : &mut Vec<V>   ,
    con        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    V : AtomEvalVec + From<f32>,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        call_n_arg,
        call_n_res,
        trace,
        is_arg_var,
        is_res_var,
        call_domain_zero,
    ) = extract_call_arg(var_zero, con, flag, arg);
    // ----------------------------------------------------------------------
    //
    // forward_zero, forward_one
    let forward_zero : AtomForwardZeroValue<V>;
    let forward_one  : AtomForwardOneValue<V>;
    {   //
        // rw_lock
        let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval     = &atom_eval_vec[atom_id];
        forward_zero      = atom_eval.forward_zero_value.clone();
        forward_one       = atom_eval.forward_one_value.clone();
    }
    //
    // call_var_zero
    let mut call_var_zero : Vec<V> = Vec::new();
    forward_zero(&mut call_var_zero, call_domain_zero, call_info, trace);
    //
    // call_domain_one
    let zero_v : V = 0f32.into();
    let mut call_domain_one : Vec<&V> = Vec::with_capacity( call_n_arg );
    for i_arg in 0 .. call_n_arg {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_domain_one.push( &var_one[index] );
        } else {
            call_domain_one.push( &zero_v );
        }
    }
    // call_range_one
    let mut call_range_one = forward_one(
        &call_var_zero, call_domain_one, call_info, trace
    );
    //
    // var_one
    let mut j_res = 0;
    call_range_one.reverse();
    for i_res in (0 .. call_n_res).rev() {
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
/// first order reverse call operator for atomic functions
fn call_reverse_1_value<V> (
    var_zero   : &Vec<V>       ,
    var_one    : &mut Vec<V>   ,
    con        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    for<'a> V : AtomEvalVec + std::ops::AddAssign<&'a V>  + From<f32>,
{   // ----------------------------------------------------------------------
    let (
        atom_id,
        call_info,
        call_n_arg,
        call_n_res,
        trace,
        is_arg_var,
        is_res_var,
        call_domain_zero,
    ) = extract_call_arg(var_zero, con, flag, arg);
    // ----------------------------------------------------------------------
    //
    // forward_zero, reverse_one
    let forward_zero : AtomForwardZeroValue<V>;
    let reverse_one  : AtomReverseOneValue<V>;
    {   //
        // rw_lock
        let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
        //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        let atom_eval     = &atom_eval_vec[atom_id];
        forward_zero      = atom_eval.forward_zero_value.clone();
        reverse_one       = atom_eval.reverse_one_value.clone();
    }
    //
    // call_var_zero
    let mut call_var_zero : Vec<V> = Vec::new();
    forward_zero(&mut call_var_zero, call_domain_zero, call_info, trace);
    //
    // call_range_one
    let zero_v : V = 0f32.into();
    let mut call_range_one : Vec<&V> = Vec::with_capacity( call_n_res );
    let mut j_res = 0;
    for i_res in 0 .. call_n_res {
        if is_res_var[i_res] {
            call_range_one.push( &var_one[res + j_res] );
            j_res += 1;
        } else {
            call_range_one.push( &zero_v );
        }
    }
    // call_domain_one
    let call_domain_one = reverse_one(
        &call_var_zero, call_range_one, call_info, trace
    );
    //
    // var_one
    for i_arg in 0 .. call_n_arg {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            var_one[index] += &call_domain_one[i_arg];
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
    // call_n_arg
    let call_n_arg = arg[2] as usize;
    //
    // is_var
    let begin    = arg[3] as usize;
    let end      = begin + call_n_arg;
    let is_var   = &flag[begin .. end];
    //
    // arg_var_index
    let zero_t = 0 as IndexT;
    arg_var_index.resize(0, zero_t);
    for call_i_arg in 0 .. call_n_arg {
        if is_var[call_i_arg] {
            arg_var_index.push( arg[5 + call_i_arg]  );
        }
    }
    assert_ne!( arg_var_index.len() , 0 );
}
//
// set_op_info
/// Set the operator information for call.
///
/// * op_info_vec :
/// The map from operator id to operator information [OpInfo] .
/// The map results for CALL_OP and CALL_RES_OP are set.
pub(crate) fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> V : AtomEvalVec + std::ops::AddAssign<&'a V> + From<f32> ,
{
    op_info_vec[CALL_OP as usize] = OpInfo{
        name              : "call" ,
        forward_0_value   : call_forward_0_value::<V>,
        forward_0_ad      : panic_zero::<V, AD<V> >,
        forward_1_value   : call_forward_1_value::<V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : call_reverse_1_value::<V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : call_arg_var_index,
    };
    op_info_vec[CALL_RES_OP as usize] = OpInfo{
        name              : "call_res" ,
        forward_0_value   : no_op_zero::<V, V>,
        forward_0_ad      : no_op_zero::<V, AD<V> >,
        forward_1_value   : no_op_one::<V, V>,
        forward_1_ad      : no_op_one::<V, AD<V> >,
        reverse_1_value   : no_op_one::<V, V>,
        reverse_1_ad      : no_op_one::<V, AD<V> >,
        arg_var_index     : no_op_arg_var_index,
    };
}
// ---------------------------------------------------------------------------
//
// no_op_zero
/// [ForwardZero](crate::op::info::ForwardZero) function
fn no_op_zero<V, E>(
    _var_zero : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { }
//
// no_op_one
/// [ForwardOne](crate::op::info::ForwardOne) or
/// [ReverseOne](crate::op::info::ReverseOne) function
fn no_op_one<V, E>(
    _var_zero : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _con      : &Vec<V>     ,
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
