// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//! Renumber an operation sequence so it uses the first of equivalent operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    IndexT,
};
use crate::ad::ADType;
use crate::tape::OpSequence;
// ---------------------------------------------------------------------------
pub(crate) fn renumber(
    equal_type  : ADType           ,
    first_equal : &Vec<IndexT>     ,
    depend      : &Vec<bool>       ,
    op_seq      : &mut OpSequence  ,
) {
    //
    // n_dep
    let n_dep = op_seq.n_dep;
    //
    // n_dom
    let n_dom        = op_seq.n_dom;
    let n_dom_indext = n_dom as IndexT;
    //
    // new_arg
    let mut new_arg : Vec<IndexT> = Vec::new();
    //
    // op_seq.arg_all
    for op_index in 0 .. n_dep {
        //
        // both_index
        let both_index = op_index + n_dom;
        if depend[both_index] {
            //
            // new_arg
            new_arg.clear();
            //
            let start      = op_seq.arg_start[op_index] as usize;
            let end        = op_seq.arg_start[op_index + 1] as usize;
            let arg        = &op_seq.arg_all[start .. end];
            let arg_type   = &op_seq.arg_type_all[start .. end];
            for i_arg in 0 .. arg.len() {
                if n_dom_indext <= arg[i_arg] {
                    if arg_type[i_arg] == equal_type {
                        let both_index  = arg[i_arg] as usize;
                        let dep_index   = both_index - n_dom;
                        new_arg.push( first_equal[dep_index] + n_dom_indext );
                    } else {
                        new_arg.push( arg[i_arg] );
                    }
                } else {
                    new_arg.push( arg[i_arg] );
                }
            }

            let arg  = &mut op_seq.arg_all[start .. end];
            for i_arg in 0 .. arg.len() {
                arg[i_arg] = new_arg[i_arg];
            }
        }
    }
}
