// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] compress_dyp method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    IndexT,
};
use crate::ad::ADType;
use crate::adfn::optimize::{
        Depend,
        op_hash_map::first_equal_op,
};
// ---------------------------------------------------------------------------
//
// ADfn::compress_dyp
impl<V> ADfn<V>
where
    V : Clone + Eq + std::fmt::Display + std::hash::Hash,
{   //
    // compress_dyp
    /// For each dynamic parameter, replace its use by the first
    /// identical dynamic parameter.
    ///
    /// * Syntax :
    /// ```text
    ///     f.compress_dyp(depend, trace)
    /// ```
    ///
    /// * Assumption :
    /// The constants have already been compressed using compress_cop.
    ///
    /// * f :
    /// The [ADfn] object for which the dynamic parameters are compressed.
    /// The input and output f represent the same domain to range map.
    /// The fields f.dyp.arg_all and f.var.arg_all are modified.
    ///
    /// * depend :
    /// On input and output, this is the [Depend] structure for the input f .
    /// The depend.dyp field is modified because only the first
    /// of identical dynamic parameters is used.
    ///
    /// * trace :
    /// if true, a trace of the compression is printed on std::out.
    pub(crate) fn compress_dyp(
        &mut self            ,
        depend : &mut Depend ,
        trace  : bool        )
    {
        // n_dep
        let n_dep = self.dyp.n_dep;
        //
        // n_dom, n_dom_i
        let n_dom        = self.dyp.n_dom;
        let n_dom_indext = n_dom as IndexT;
        //
        // first_equal
        let mut first_equal : Vec<IndexT> = Vec::with_capacity(n_dep);
        for op_index in 0 .. n_dep {
            first_equal.push( op_index as IndexT );
        }
        //
        // new_arg
        let mut new_arg : Vec<IndexT> = Vec::new();
        //
        // first_equal
        let op_seq_type = ADType::DynamicP;
        let first_equal = first_equal_op(
            op_seq_type, &mut depend.dyp, &self.dyp
        );
        //
        if trace {
            println!("Begin Trace compress_dyp");
            println!("original_index, compressed_index");
            for op_index in 0 .. n_dep {
                if first_equal[op_index] != op_index as IndexT {
                    let dep_index   = op_index + n_dom;
                    let match_index = first_equal[op_index] + n_dom_indext;
                    println!( "{}, {}", dep_index, match_index );
                }
            }
        }
        //
        // self.rng_index
        for i in 0 .. self.rng_index.len() {
            let dynamic   = self.rng_ad_type[i].is_dynamic();
            let dependent = dynamic && n_dom_indext <= self.rng_index[i];
            if dependent {
                let dyp_index     = self.rng_index[i] as usize;
                let op_index      = dyp_index - n_dom;
                self.rng_index[i] = first_equal[op_index] + n_dom_indext;
            }
        }
        //
        // self.dyp.arg_all
        for op_index in 0 .. self.dyp.n_dep {
            //
            // res
            let res = op_index + self.dyp.n_dom;
            if depend.dyp[res] {
                new_arg.clear();
                //
                let start      = self.dyp.arg_start[op_index] as usize;
                let end        = self.dyp.arg_start[op_index + 1] as usize;
                let arg        = &self.dyp.arg_all[start .. end];
                let arg_type   = &self.dyp.arg_type_all[start .. end];
                for i_arg in 0 .. arg.len() {
                    let dynamic   = arg_type[i_arg].is_dynamic();
                    let dependent = dynamic && n_dom_indext <= arg[i_arg];
                    if dependent {
                        let dyp_index  = arg[i_arg] as usize;
                        let dep_index  = dyp_index - n_dom;
                        new_arg.push( first_equal[dep_index] + n_dom_indext );
                    } else {
                        new_arg.push( arg[i_arg] );
                    }
                }
                let arg  = &mut self.dyp.arg_all[start .. end];
                for i_arg in 0 .. arg.len() {
                    arg[i_arg] = new_arg[i_arg];
                }
            }
        }
        //
        // self.var.arg_all
        for op_index in 0 .. self.var.n_dep {
            //
            // res
            let res = op_index + self.var.n_dom;
            if depend.var[res] {
                //
                let start      = self.var.arg_start[op_index] as usize;
                let end        = self.var.arg_start[op_index + 1] as usize;
                let arg        = &mut self.var.arg_all[start .. end];
                let arg_type   = &self.var.arg_type_all[start .. end];
                for i_arg in 0 .. arg.len() {
                    let dynamic   = arg_type[i_arg].is_dynamic();
                    let dependent = dynamic && n_dom_indext <= arg[i_arg];
                    if dependent {
                            let dyp_index  = arg[i_arg] as usize;
                            let dep_index  = dyp_index - n_dom;
                            arg[i_arg] = first_equal[dep_index] + n_dom_indext;
                    }
                }
            }
        }
        if trace {
            println!("End Trace compress_dyp");
        }
    }
}
