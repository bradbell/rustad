// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Operator that calls an ADFun
//! : [parent module](super)
//!
//! # Operator Id
//!  CALL_OP
//!
//! # Operator Arguments
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index that identifies the ADFun object being called |
//! | 1        | Number of arguments to the function being called (n_arg) |
//! | 2        | Number of results for the function being called  (n_res) |
//! | 3        | Index of the first boolean for this operator |
//! | 4        | Variable or constant index for first argument to call |
//! | 5        | Variable or constant index for second argument to call |
//! | ...      | ... |
//! | 3+n_arg  | Variable or constant index for last argument to call |
//!
//! # Operator Booleans
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | true (false) if first call argument is a variable (constant) |
//! | 1        | true (false) if second call argument is a variable (constant) |
//! | ...      | ... |
//! | n_arg-1  | true (false) if last call argument is a variable (constant) |
//! | n_arg    | true (false) if first result is a variable (constant) |
//! | n_arg+1  | true (false) if second result is a variable (constant) |
//! | n_arg+n_res-1 | true (false) if last result is a variable (constant) |
//!
//! # Operator Results
//! We use n_var_res for the number of results that are variables.
//! There are n_var_res - 1 CALL_RES_OP operators directly after each CALL_OP
//! operator in the sequence of operations. These are place holders so that
//! there is a direct correpondence between variable and operator indices.
//
use crate::ad::GAD;
use crate::ptrait::GenericAs;
use crate::{Index, Float};
use crate::checkpoint::sealed::ThisThreadCheckpointAll;
use crate::operator::id::{CALL_OP, CALL_RES_OP};
use crate::operator::GlobalOpInfoVec;
use crate::operator::OpInfo;
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::{
    ForwardZero,
    ForwardOne,
    ReverseOne,
    ArgVarIndex,
};
//
// forward_0_call
/// Float zero order forward for call operator.
///
/// Note that checkpoints do not allow for ad_forward because
/// it would just record the evaluation inside of the checkpoint function.
fn forward_0_call<F,U>(
    var_zero:    &mut Vec<F>,
    con:         &Vec<F>,
    flag_all:    &Vec<bool>,
    arg:         &[U],
    res:         usize)
where
    U : 'static + Copy + GenericAs<usize> + std::fmt::Debug,
    F : Copy +
        From<F> +
        From<f32> +
        GlobalOpInfoVec<U> +
        std::fmt::Display +
        ThisThreadCheckpointAll<U>,
    GAD<F,U>: From<F>,
{   //
    // call_index, n_arg, n_res
    let call_index  = GenericAs::gas(arg[0]);
    let n_arg       = GenericAs::gas(arg[1]);
    let n_res       = GenericAs::gas(arg[2]);
    //
    // is_arg_var, is_res_var
    let mut begin : usize = GenericAs::gas(arg[3]);
    let mut end     = begin + n_arg;
    let is_arg_var  = &flag_all[begin .. end];
    begin           = end;
    end             = begin + n_res;
    let is_res_var  = &flag_all[begin .. end];
    //
    // call_domain_zero
    let mut call_domain_zero : Vec<F> = Vec::new();
    for i_arg in 0 .. n_arg {
        if is_arg_var[i_arg] {
            call_domain_zero.push( var_zero[ GenericAs::gas(arg[i_arg + 4]) ] );
        } else {
            let c = con[ GenericAs::gas(arg[i_arg + 4]) ];
            call_domain_zero.push( c );
        }
    }
    //
    // call_range_zero
    let local_key       = < F as ThisThreadCheckpointAll<U> >::get();
    let call_range_zero = local_key.with_borrow( |all| {
        let checkpoint_info = &all.vec[call_index];
        let adfun           = &checkpoint_info.adfun;
        let trace           = false;
        let (range_zero, _call_var_zero) =
            adfun.forward_zero(&call_domain_zero, trace);
        range_zero
    } );
    //
    // var_zero
    let mut j_res = 0;
    for i_res in 0 .. n_res {
        if is_res_var[i_res] {
            var_zero[res + j_res] = call_range_zero[i_res];
            j_res += 1;
        }
    }
}
//
// call_arg_var_index
/// vector of variable indices that are arguments to this call operator
fn call_arg_var_index<U>(
    arg_var_index : &mut Vec<U>, flag_all : &Vec<bool>, arg: &[U]

)
where
    U     : Copy + GenericAs<usize> ,
    usize : GenericAs<U> ,
{
    //
    // call_n_arg
    let call_n_arg : usize = GenericAs::gas(arg[1]);
    //
    // is_var
    let begin : usize = GenericAs::gas(arg[3]);
    let end           = begin + call_n_arg;
    let is_var        = &flag_all[begin .. end];
    //
    // arg_var_index
    let zero_u : U = GenericAs::gas(0);
    arg_var_index.resize(0, zero_u);
    for call_i_arg in 0 .. call_n_arg {
        if is_var[call_i_arg] {
            arg_var_index.push( arg[4 + call_i_arg]  );
        }
    }
    assert_ne!( arg_var_index.len() , 0 );
}
//
// set_op_info
/// Set the operator information for call.
///
/// * op_info_vec :
/// The map from [operator::id] to operator information.
/// The map results for CALL_OP are set.
 pub(crate) fn set_op_info( op_info_vec : &mut Vec< OpInfo<Float,Index> > ) {
    op_info_vec[CALL_OP as usize] = OpInfo{
        name           : "call".to_string() ,
        forward_0      : forward_0_call,
        forward_1      : super::panic_one,
        reverse_1      : super::panic_one,
        ad_forward_0   : super::panic_zero,
        ad_forward_1   : super::panic_one,
        ad_reverse_1   : super::panic_one,
        arg_var_index  : call_arg_var_index,
     };
    op_info_vec[CALL_RES_OP as usize] = OpInfo{
        name           : "call_res".to_string() ,
        forward_0      : no_op_zero,
        forward_1      : no_op_one,
        reverse_1      : no_op_one,
        ad_forward_0   : no_op_zero,
        ad_forward_1   : no_op_one,
        ad_reverse_1   : no_op_one,
        arg_var_index  : no_op_arg_var_index,
     };
}
// ---------------------------------------------------------------------------
//
// no_op_zero
/// [ForwardZero] function
fn no_op_zero<F, U, E>( _var_zero: &mut Vec<E>,
    _con_all: &Vec<F>, _flag_all : &Vec<bool>, _arg: &[U], _res: usize)
{ }
//
// no_op_one
/// [ForwardOne] or [ReverseOne] function
fn no_op_one<F, U, E>( _var_one: &mut Vec<E>, _var_zero : &Vec<E>,
    _con_all: &Vec<F>, _arg: &[U], _res: usize)
{ }
//
// no_op_arg_var_index
/// [ArgVarIndex] function
fn no_op_arg_var_index<U>(
    arg_var_index: &mut Vec<U>, _flag_all: &Vec<bool>, _arg: &[U])
where
    U     : Copy ,
    usize : GenericAs<U> ,
{
    let zero_u : U = GenericAs::gas(0);
    arg_var_index.resize(0, zero_u);
}
