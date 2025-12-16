// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] normalize_call method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    IndexT,
};
use crate::tape::OpSequence;
use crate::adfn::optimize::Depend;
use crate::op::id::{
    NO_OP,
    CALL_OP,
    CALL_RES_OP,
};
use crate::op::call::{
    NUMBER_RNG,
    BEGIN_FLAG,
};
//
// group_call
fn group_call(
    op_seq : &mut OpSequence ,
    depend : &mut Vec<bool>  ,
) -> Vec<IndexT>  // old2new
{   //
    // n_dep, n_dom
    let n_dep = op_seq.n_dep;
    let n_dom = op_seq.n_dom;
    //
    // old2new
    let mut old2new : Vec<IndexT> = Vec::new();
    //
    // op_index
    let mut op_index = 0;
    while op_index < n_dep {
        //
        // op_index, res, op_id
        let res = op_index + n_dom;
        if ! depend[res] {
            op_index += 1
        } else {
            let mut op_id = op_seq.id_all[op_index];
            if op_id != CALL_OP && op_id != CALL_RES_OP {
                op_index += 1
            } else {
                // This call gets used.
                //
                // old2new
                if old2new.len() == 0 {
                    old2new.reserve(n_dep + n_dom);
                    for index in 0 .. n_dep + n_dom {
                        old2new.push( index as IndexT );
                    }
                }
                //
                // op_id, op_index
                if op_id != CALL_OP {
                    //
                    debug_assert!( op_id == CALL_RES_OP );
                    let start  = op_seq.arg_start[op_index] as usize;
                    let offset = op_seq.arg_all[start] as usize;
                    debug_assert!( offset < op_index );
                    //
                    op_index -= offset;
                    op_id     = op_seq.id_all[op_index];
                }
                debug_assert!( op_id == CALL_OP );
                //
                // n_rng, trace_this_op, old_rng_id_dep
                let start = op_seq.arg_start[op_index] as usize;
                let n_rng = op_seq.arg_all[start + NUMBER_RNG] as usize;
                let begin = op_seq.arg_all[start + BEGIN_FLAG] as usize;
                let flag  = &mut op_seq.flag_all[begin .. begin + n_rng + 1];
                //
                //
                // new_flag, old2new, new_n_dep
                let mut old_i_dep = 0;
                let mut new_i_dep = 0;
                let mut new_flag  = vec![false; n_rng + 1];
                new_flag[0]       = flag[0];
                for i_rng in 0 .. n_rng {
                    if flag[i_rng + 1] {
                        let old_index = op_index + old_i_dep;
                        let new_index = op_index + new_i_dep;
                        if depend[old_index + n_dom] {
                            let old_res      = old_index + n_dom;
                            let new_res      = (new_index + n_dom) as IndexT;
                            old2new[old_res] = new_res;
                            new_flag[i_rng + 1]        = true;
                            new_i_dep                 += 1;
                        }
                        old_i_dep += 1;
                    }
                }
                let old_n_dep = old_i_dep;
                let new_n_dep = new_i_dep;
                //
                // op_seq.id_all, old2new
                old_i_dep            = 0;
                let mut n_no_op      = 0;
                for i_rng in 0 .. n_rng {
                    if flag[i_rng + 1] {
                        let old_index = op_index + old_i_dep;
                        let new_index = op_index + new_n_dep + n_no_op;
                        let old_res   = old_index + n_dom;
                        let new_res   = (new_index + n_dom) as IndexT;
                        if ! depend[old_res] {
                            op_seq.id_all[new_index] = NO_OP;
                            old2new[old_res]         = new_res;
                            n_no_op                 += 1;
                        }
                        old_i_dep += 1;
                    }
                }
                //
                // depend
                for i_dep in 0 .. old_n_dep {
                    let new_index = op_index + i_dep;
                    depend[new_index + n_dom] = i_dep < new_n_dep;
                }
                //
                // flag
                for i in 0 .. n_rng + 1 {
                    flag[i] = new_flag[i];
                }
                //
                // op_index
                op_index += old_n_dep;
            }
        }
    }
    old2new
}
//
// ADfn::normalize_call
impl<V> ADfn<V>
{   //
    // normalize_call
    /// For each call that is used,
    /// remove the call result operators that are not used.
    ///
    /// * Syntax :
    /// ```text
    ///     f.normalize_call(depend, trace)
    /// ```
    ///
    /// * Notation :
    /// A call in the dyp (var) operation sequence is used if its CALL_OP,
    /// or one of its CALL_RES_OP operators,
    /// correspponds to a depend.dyp (depend.var) true element.
    ///
    /// * f :
    /// The [ADFun] object for which the calls are compressed.
    /// The input and output f represent the same domain to range map.
    /// The following fields are modified :
    /// f.dyp.id_all, f.dyp.arg_all, f.var.id_all, f.var_arg_all, f.rng_index .
    ///
    /// * depend :
    /// On input and output, this is the [Depend] structure for the input f .
    /// The depend.dyp and depend.var fields are modified.
    /// This is because all the call operators that are used are
    /// are grouped together (the ones that are not used are
    /// changed to NO_OP operators).
    ///
    /// * trace :
    /// if true, a trace of the normalization is printed on std::out.
    pub(crate) fn normalize_call(
        &mut self            ,
        depend : &mut Depend ,
        trace  : bool        )
    {
        if trace {
            println!("Begin Trace normalize_call");
        }
        //
        // i_op_seq, op_seq, op_depend
        for i_op_seq in 0 .. 2 {
            let op_seq    : &mut OpSequence;
            let op_depend : &mut Vec<bool>;
            if i_op_seq == 0 {
                op_seq    = &mut self.dyp;
                op_depend = &mut depend.dyp;
                if trace {
                    println!("dyp_old_index, dyp_new_index");
                }
            } else {
                op_seq    = &mut self.var;
                op_depend = &mut depend.var;
                if trace {
                    println!("var_old_index, var_new_index");
                }
            }
            let old2new = group_call(op_seq, op_depend);
            if old2new.len() != 0 {
                if trace {
                    for op_index in 0 .. op_seq.n_dep {
                        let old = (op_index + op_seq.n_dom) as usize;
                        if old2new[old] as usize != old {
                            println!( "{}, {}", old, old2new[old] );
                        }
                    }
                }
                for i_arg in 0 .. op_seq.arg_all.len() {
                    if op_seq.arg_type_all[i_arg].is_dynamic() {
                        let old = op_seq.arg_all[i_arg] as usize;
                        op_seq.arg_all[i_arg] = old2new[old];
                    }
                }
            }
        }
        if trace {
            println!("End Trace normalize_call");
        }
    }
}
