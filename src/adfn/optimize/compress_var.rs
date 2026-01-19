// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] compress_var method.
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
        renumber_op_seq,
        Depend,
        op_hash_map::first_equal_op,
};
// ---------------------------------------------------------------------------
//
// ADfn::compress_var
impl<V> ADfn<V>
where
    V : Clone + Eq + std::fmt::Display + std::hash::Hash,
{   //
    // compress_var
    /// For each variable, replace its use by the first
    /// identical variable.
    ///
    /// * Syntax :
    ///   ```text
    ///     f.compress_var(depend, trace)
    ///   ```
    ///
    /// * Assumption :
    ///   The constants have already been compressed using compress_cop.
    ///
    /// * f :
    ///   The [ADfn] object for which the variables are compressed.
    ///   The input and output f represent the same domain to range map.
    ///   The fields f.var.arg_all and f.rng_index are modified.
    ///
    /// * depend :
    ///   On input and output, this is the [Depend] structure for the input f .
    ///   The depend.var field is modified because only the first
    ///   of identical variables is used.
    ///
    /// * trace :
    ///   if true, a trace of the compression is printed on std::out.
    pub(crate) fn compress_var(
        &mut self            ,
        depend : &mut Depend ,
        trace  : bool        )
    {
        // n_dep
        let n_dep = self.var.n_dep;
        //
        // n_dom, n_dom_i
        let n_dom        = self.var.n_dom;
        let n_dom_indext = n_dom as IndexT;
        //
        // first_equal
        let op_seq_type = ADType::DynamicP;
        let first_equal = first_equal_op(
            op_seq_type, &depend.var, &self.var
        );
        debug_assert!( first_equal.len() == n_dep );
        //
        // depend.var
        for (op_index, equal_index) in first_equal.iter().enumerate() {
            if *equal_index as usize != op_index {
                depend.var[op_index + n_dom] = false;
            }
        }
        //
        if trace {
            println!("Begin Trace compress_var");
            println!("original_index, compressed_index");
            for (op_index, equal_index) in first_equal.iter().enumerate() {
                if *equal_index as usize != op_index {
                    let dep_index   = op_index + n_dom;
                    let match_index = *equal_index + n_dom_indext;
                    println!( "{}, {}", dep_index, match_index );
                }
            }
        }
        //
        // self.rng_index
        for i in 0 .. self.rng_index.len() {
            let variable  = self.rng_ad_type[i].is_variable();
            let dependent = variable && n_dom_indext <= self.rng_index[i];
            if dependent {
                let var_index     = self.rng_index[i] as usize;
                let op_index      = var_index - n_dom;
                self.rng_index[i] = first_equal[op_index] + n_dom_indext;
            }
        }
        //
        // self.var.arg_all
        let equal_type = ADType::DynamicP;
        renumber_op_seq(equal_type, &first_equal, &depend.var, &mut self.var);
        //
        if trace {
            println!("End Trace compress_var");
        }
    }
}
