// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD add operators.
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::AD;
use crate::ADD_VC_OP;
use crate::ADD_VV_OP;
use crate::Float;
use crate::Index;
use crate::OpInfo;
use crate::ad_tape::THIS_THREAD_TAPE;
use crate::ad_tape::Tape;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator_id;
//
// ---------------------------------------------------------------------------
// forward_0_add_vc_fn
/// Stores the result of a zero order variable + constant
/// operation in the tape.
///
/// # con
/// is the vector of constants in the tape.
///
/// # arg
/// is a slice of size two containing the arguments for this addition.
/// We use the notation
/// <pre>
///     lhs = arg[0]
///     rhs = arg[1]
/// </pre>
///
/// # res
/// is the index in *var* where the result for this addition is placed.
///
/// # var
/// is the vector of the zero order values for all the variables.
/// It is changed in the following way
/// <pre>
///     var[res] = var[lhs] + con[rhs]
/// </pre>
fn forward_0_add_vc_fn(
    var: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var[ res ] = var[ arg[0] ] + con[ arg[1] ];
}
// ---------------------------------------------------------------------------
// forward_0_add_vv_fn
/// Stores the result of a zero order variable + variable
/// operation in the tape.
///
/// # arg
/// is a slice of size two containing the arguments for this addition.
/// We use the notation
/// <pre>
///     lhs = arg[0]
///     rhs = arg[1]
/// </pre>
///
/// # res
/// is the index in *var* where the result for this addition is placed.
///
/// # var
/// is the vector of the zero order values for all the variables.
/// It is changed in the following way
/// <pre>
///     var[res] = var[lhs] + var[rhs]
/// </pre>
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
/// is a map from [operator_id] to operator information.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
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
                tape.op_all.push(ADD_VV_OP);
                tape.op2arg.push( tape.arg_all.len() );
                tape.arg_all.push( lhs.var_index );
                tape.arg_all.push( rhs.var_index );
            } else if var_lhs {
                if rhs.value == 0.0 {
                    new_var_index = lhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.op_all.push(ADD_VC_OP);
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
                    tape.op_all.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_all.len() );
                    tape.arg_all.push( rhs.var_index );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( lhs.value );
                }
            }
        }
    }
    (new_tape_id, new_var_index)
}
// ---------------------------------------------------------------------------
impl std::ops::Add<AD> for AD {
    type Output = AD;
    //
    /// compute AD + AD
    fn add(self, rhs : AD) -> AD {
        let new_value                     = self.value + rhs.value;
        let ( new_tape_id, new_var_index) =
            THIS_THREAD_TAPE.with_borrow_mut(
                |tape| record_add(tape, &self, &rhs)
        );
        AD {
            tape_id   : new_tape_id,
            var_index : new_var_index,
            value     : new_value,
        }
    }
}
//
impl std::ops::Add<AD> for Float {
    type Output = AD;
    //
    /// compute Float + AD
    fn add(self, rhs : AD) -> AD {
        AD::from(self) + rhs
    }
}
//
impl std::ops::Add<Float> for AD {
    type Output = AD;
    //
    /// compute AD + Float
    fn add(self, rhs : Float) -> AD {
        self + AD::from(rhs)
    }
}
