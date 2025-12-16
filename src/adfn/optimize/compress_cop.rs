// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] compress_cop method.
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
use rustc_hash::FxHashMap;
//
//
// ADfn::compress_cop
impl<V> ADfn<V>
where
    V : Clone + Eq + std::fmt::Display + std::hash::Hash,
{   //
    // compress_cop
    /// For each constant, replace its use by the first equal constant.
    ///
    /// * Syntax :
    /// ```text
    ///     f.compress_cop(depend, trace)
    /// ```
    ///
    /// * f :
    /// The [ADfn] object for which the constants are compressed.
    /// The input and output f represent the same domain to range map.
    /// The fields f.dyp.arg_all, f.var.arg_all, and f.rng_index
    /// are modified.
    ///
    /// * depend :
    /// On input and output,
    /// this is the [Depend] structure for the input f.
    /// The depend.cop is modified because
    /// only the first of the equal constants is used.
    ///
    /// * trace :
    /// if true, a trace of the compression is printed on std::out.
    pub(crate) fn compress_cop(
        &mut self            ,
        depend : &mut Depend ,
        trace  : bool        )
    {
        //
        // n_cop
        let n_cop = self.cop_len();
        //
        // hash_map
        let mut hash_map : FxHashMap<V, IndexT> = FxHashMap::default();
        //
        if trace {
            println!("Begin Trace compress_cop");
            println!("cop_value, original_index, compressed_index");
        }
        //
        // hash_map, depend.cop,
        for i_cop in 0 .. n_cop {
            if depend.cop[i_cop] {
                let key = &self.cop[i_cop];
                if ! hash_map.contains_key(key) {
                    hash_map.insert(key.clone(), i_cop as IndexT );
                } else {
                    depend.cop[i_cop] = false;
                }
                if trace {
                    let index = hash_map.get(key).unwrap();
                    println!("{}, {}, {}", key, i_cop, index);
                }
            }
        }
        //
        // self.rng_index
        for i in 0 .. self.rng_index.len() {
            if self.rng_ad_type[i].is_constant() {
                let key           = &self.cop[ self.rng_index[i] as usize ];
                let index         = hash_map.get(key).unwrap();
                self.rng_index[i] = *index;
            }
        }
        //
        // op_seq, op_depend
        for i_op_seq in 0 .. 2 {
            let op_seq    : &mut OpSequence;
            let op_depend : &Vec<bool>;
            if i_op_seq == 0 {
                op_seq    = &mut self.dyp;
                op_depend = &depend.dyp;
            } else {
                op_seq    = &mut self.var;
                op_depend = &depend.var;
            }
            //
            // op_seq.arg_all
            for op_index in 0 .. op_seq.n_dep {
                let res = op_index + op_seq.n_dom;
                if op_depend[res] {
                    //
                    let start      = op_seq.arg_start[op_index] as usize;
                    let end        = op_seq.arg_start[op_index + 1] as usize;
                    let arg        = &mut op_seq.arg_all[start .. end];
                    let arg_type   = &op_seq.arg_type_all[start .. end];
                    for i_arg in 0 .. arg.len() {
                        if arg_type[i_arg].is_constant() {
                            let key    = &self.cop[ arg[i_arg] as usize ];
                            let index  = hash_map.get(key).unwrap();
                            arg[i_arg] = *index;
                        }
                    }
                }
            }
        }
        if trace {
            println!("End Trace compress_cop");
        }
    }
}
