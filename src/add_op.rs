// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use crate::Float;
use crate::Index;
use crate::TapeInfo;
use crate::AD;
use crate::ADD_VC_OP;
use crate::ADD_VV_OP;
use crate::THIS_THREAD_RECORDER;
//
// eval_add_vv_fn
pub fn eval_add_vv_fn(
    vec: &mut Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    vec[ res ] = vec[ arg[0] ] + vec[ arg[1] ];
}
//
// eval_add_vc_fn
pub fn eval_add_vc_fn(
    vec: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    vec[ res ] = vec[ arg[0] ] + con[ arg[1] ];
}
//
// std::ops::ADD for AD
fn record_add(tape : &mut TapeInfo, lhs : &AD, rhs : &AD) -> (Index, Index) {
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
                tape.op_vec.push(ADD_VV_OP);
                tape.op2arg.push( tape.arg_vec.len() );
                tape.arg_vec.push( lhs.var_index );
                tape.arg_vec.push( rhs.var_index );
            } else if var_lhs {
                if rhs.value == 0.0 {
                    new_var_index = lhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.op_vec.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_vec.len() );
                    tape.arg_vec.push( lhs.var_index );
                    tape.arg_vec.push( tape.con_vec.len() );
                    tape.con_vec.push( rhs.value );
                }
            } else {
                if lhs.value == 0.0 {
                    new_var_index = rhs.var_index;
                } else {
                    new_var_index = tape.n_var;
                    tape.n_var   += 1;
                    tape.op_vec.push(ADD_VC_OP);
                    tape.op2arg.push( tape.arg_vec.len() );
                    tape.arg_vec.push( rhs.var_index );
                    tape.arg_vec.push( tape.con_vec.len() );
                    tape.con_vec.push( lhs.value );
                }
            }
        }
    }
    (new_tape_id, new_var_index)
}
impl std::ops::Add for AD {
    type Output = AD;
    fn add(self, rhs : AD) -> AD
    {   let new_value                     = self.value + rhs.value;
        let ( new_tape_id, new_var_index) = THIS_THREAD_RECORDER.with_borrow_mut(
            |tape| record_add(tape, &self, &rhs)
        );
        AD {
            tape_id   : new_tape_id,
            var_index : new_var_index,
            value     : new_value,
        }
    }
}
