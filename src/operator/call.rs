// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Operator that calls an ADFun (Under Construction):  [parent module](super)
//!
//! # Operator Id
//!  CALL_OP
//!
//! # Operator Arguments:
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index that identifies the ADFun object being called |
//! | 1        | Number of arguments to the function being called (n_arg) |
//! | 2        | Number of results for the function being called  |
//! | 3        | Index of the first boolean for this operator |
//! | 4        | Variable or constant index for first argument to call |
//! | 5        | Variable or constant index for second argument to call |
//! | ...      | ... |
//! | 3+n_arg  | Variable or constant index for last argument to call |
//!
//! # Operator Booleans:
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | true (false) if first call argument is a variable (constant) |
//! | 1        | true (false) if second call argument is a variable (constant) |
//! | ...      | ... |
//! | n_arg-1  | true (false) if last call argument is a variable (constant) |
//!
//
use crate::{Index, Float};
use crate::function::ADFun;
use crate::operator::id::CALL_OP;
use crate::operator::OpInfo;
//
// float_forward_0_call
/// Float zero order forward for call operator
fn float_forward_0_call(
    var_zero:    &mut Vec<Float>,
    con:         &Vec<Float>,
    flag_all:    &Vec<bool>,
    arg:         &[Index],
    res:         Index)
{   //
    // call_index, n_arg, n_res
    let call_index  = arg[0];
    let n_arg       = arg[1];
    let n_res       = arg[2];
    //
    // is_arg_var, is_res_var
    let mut begin   = arg[3];
    let mut end     = begin + n_arg;
    let is_arg_var  = &flag_all[begin .. end];
    begin           = end;
    end             = begin + n_res;
    let is_res_var  = &flag_all[begin .. end];
    //
    // call_domain_zero
    let mut call_domain_zero : Vec<Float> = Vec::new();
    for i_arg in 0 .. n_arg {
        if is_arg_var[i_arg] {
            call_domain_zero.push( var_zero[ arg[i_arg + 4] ] );
        } else {
            call_domain_zero.push( con[ arg[i_arg + 4] ] );
        }
    }
    //
    // call_range_zero
    let call_range_zero = THIS_THREAD_CHECKPOINT_VEC.with_borrow( |adfun_vec| {
        let trace = false;
        let (range_zero, _call_var_zero) =
            adfun_vec[call_index].forward_zero(&call_domain_zero, trace);
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
// set_op_info
/// Set the operator information for call.
 pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[CALL_OP] = OpInfo{
        name         : "call".to_string() ,
        forward_0    : float_forward_0_call,
        forward_1    : super::panic_one,
        reverse_1    : super::panic_one,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
     };
}
//
thread_local! {
    // THIS_THREAD_CHECKPOINT_VEC
    /// is thread local storage holding a vector of checkpoint functions.
    pub(crate) static THIS_THREAD_CHECKPOINT_VEC:
        std::cell::RefCell< Vec<ADFun> > =
            std::cell::RefCell::new( Vec::new() );
}
