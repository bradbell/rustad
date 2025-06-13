// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD add operator.
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::AD;
use crate::Float;
use crate::Index;
use crate::ad_tape::THIS_THREAD_TAPE;
use crate::ad_tape::Tape;
use crate::operator::OpInfo;
use crate::operator::id::{ADD_CV_OP, ADD_VC_OP, ADD_VV_OP};
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::ForwardZeroBinary;
//
// ---------------------------------------------------------------------------
// forward_0_add_cv_fn
/// [ForwardZeroBinary] were op is +, left is variable, right is constant.
fn forward_0_add_cv_fn(
    var: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var[ res ] = con[ arg[0] ] + var[ arg[1] ];
}
// ---------------------------------------------------------------------------
// forward_0_add_vc_fn
/// [ForwardZeroBinary] were op is +, left is variable, right is constant.
fn forward_0_add_vc_fn(
    var: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var[ res ] = var[ arg[0] ] + con[ arg[1] ];
}
// ---------------------------------------------------------------------------
// forward_0_add_vv_fn
/// [ForwardZeroBinary] where op is +, left is variable, right is variable.
fn forward_0_add_vv_fn(
    var: &mut Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var[ res ] = var[ arg[0] ] + var[ arg[1] ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the add operators.
///
/// # op_info_vec
/// is a map from [operator::id] to operator information.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[ADD_CV_OP] =
        OpInfo{ name : "add_cv".to_string() , forward_0 : forward_0_add_cv_fn };
    op_info_vec[ADD_VC_OP] =
        OpInfo{ name : "add_vc".to_string() , forward_0 : forward_0_add_vc_fn };
    op_info_vec[ADD_VV_OP] =
        OpInfo{ name : "add_vv".to_string() , forward_0 : forward_0_add_vv_fn };
}
// ---------------------------------------------------------------------------
// record_add
/// This records addition operations that result in a variable.
///
/// # lhs
/// is the left (first) operand for this operation.
///
/// # rhs
/// is the right (second) operand for this operation.
///
/// # tape
/// is the tape for this thread.
/// If both *lhs* and *rhs* are constants, the tape is not modified.
/// If both *lhs* and *rhs* are variables, a variable + variable
/// operator is added to the end of the tape.
/// Otherwise a variable + constant operator s added to the tape.
/// Note that additionm is commutative so we do not need a
/// constant + variable operator.
///
/// # record_add
/// The return value is the ( *tape_id* , *var_index* ) fields for the AD
/// object corresponding to the summation.
fn record_add(tape : &mut Tape, lhs : &AD, rhs : &AD) -> (Index, Index) {
    let mut new_tape_id   = 0;
    let mut new_var_index = 0;
    if tape.recording {
        let var_lhs    = lhs.tape_id == tape.tape_id;
        let var_rhs    = rhs.tape_id == tape.tape_id;
        if var_lhs || var_rhs {
            new_tape_id = tape.tape_id;
            if var_lhs && var_rhs {
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.id_all.push(ADD_VV_OP);
                tape.op2arg.push( tape.arg_all.len() );
                tape.arg_all.push( lhs.var_index );
                tape.arg_all.push( rhs.var_index );
            } else if var_lhs {
                if rhs.value == 0.0 {
                    new_var_index = lhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.id_all.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_all.len() );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( rhs.value );
                }
            } else {
                if lhs.value == 0.0 {
                    new_var_index = rhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.id_all.push(ADD_CV_OP);
                    tape.op2arg.push( tape.arg_all.len() );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.arg_all.push( rhs.var_index );
                    tape.con_all.push( lhs.value );
                }
            }
        }
    }
    (new_tape_id, new_var_index)
}
impl_binary_operator!( Add, + );
