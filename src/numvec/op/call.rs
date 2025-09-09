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
//! | 0        | true (false) if first argument is a variable (constant)   |
//! | 1        | true (false) if second argument is a variable (constant)  |
//! | ...      | ... |
//! | n_arg-1  | true (false) if last argument is a variable (constant)    |
//! | n_arg    | true (false) if first result is a variable (constant)     |
//! | n_arg+1  | true (false) if second result is a variable (constant)    |
//! | n_arg+n_res-1 | true (false) if last result is a variable (constant) |
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
use crate::numvec::op::info::OpInfo;
use crate::numvec::atom::{
    Callback,
    sealed::AtomEvalVec,
};
use crate::numvec::op::id::{
        CALL_OP,
        CALL_RES_OP,
};
use crate::numvec::{
    AD,
    IndexT,
    AtomEval,
    panic_zero,
    panic_one,
};
// --------------------------------------------------------------------------
//
// forward_0_call_value
/// zero order forward for call operator for atomic functions
fn call_forward_0<V> (
    var_zero   : &mut Vec<V>   ,
    con        : &Vec<V>       ,
    flag       : &Vec<bool>    ,
    arg        : &[IndexT]     ,
    res        : usize         )
where
    V : AtomEvalVec,
{   //
    // atom_id, call_info, n_arg, n_res
    let atom_id    = arg[0] as usize;
    let call_info  = arg[1];
    let call_n_arg = arg[2] as usize;
    let call_n_res = arg[3] as usize;
    //
    // is_arg_var, is_res_var
    let mut begin   = arg[4] as usize;
    let mut end     = begin + call_n_arg;
    let is_arg_var  = &flag[begin .. end];
    begin           = end;
    end             = begin + call_n_res;
    let is_res_var  = &flag[begin .. end];
    //
    // domain_zero
    let mut call_domain_zero : Vec<&V> = Vec::new();
    for i_arg in 0 .. call_n_arg {
        let index = arg[i_arg + 5] as usize;
        if is_arg_var[i_arg] {
            call_domain_zero.push( &var_zero[index] );
        } else {
            call_domain_zero.push( &con[index] );
        }
    }
    //
    // rwlock
    let rw_lock : &RwLock< Vec< AtomEval<V> > > = AtomEvalVec::get();
    //
    // forward_zero
    let forward_zero : Callback<V>;
    {   //
        // read_lock
        let read_lock = rw_lock.read();
        assert!( read_lock.is_ok() );
        //
        // Rest of this block has a lock, so it should be fast and not fail.
        let atom_eval_vec = read_lock.unwrap();
        forward_zero  = atom_eval_vec[atom_id as usize].forward_zero.clone();
    }
    //
    // call_range_zero
    let mut call_var_zero  : Vec<V> = Vec::new();
    let trace = false;
    let mut call_range_zero = forward_zero(
        &mut call_var_zero, call_domain_zero, trace, call_info
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
    V : AtomEvalVec ,
{
    op_info_vec[CALL_OP as usize] = OpInfo{
        name              : "call" ,
        forward_0_value   : call_forward_0::<V>,
        forward_0_ad      : panic_zero::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
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
/// [ForwardZero](crate::numvec::op::info::ForwardZero) function
fn no_op_zero<V, E>(
    _var_zero : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { }
//
// no_op_one
/// [ForwardOne](crate::numvec::op::info::ForwardOne) or
/// [ReverseOne](crate::numvec::op::info::ReverseOne) function
fn no_op_one<V, E>(
    _var_one  : &mut Vec<E> ,
    _var_zero : &Vec<E>     ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { }
//
// no_op_arg_var_index
/// [ArgVarIndex](crate::numvec::op::info::ArgVarIndex) function
fn no_op_arg_var_index(
    arg_var_index  : &mut Vec<IndexT> ,
    _flag          : &Vec<bool>       ,
    _arg           : &[IndexT]        ,
) {
    let zero_t = 0 as IndexT;
    arg_var_index.resize(0, zero_t);
}
