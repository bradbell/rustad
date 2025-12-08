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
use rustc_hash::FxHashMap;
//
use crate::{
    ADfn,
    IndexT,
};
use crate::ad::ADType;
use crate::adfn::optimize::Depend;
use crate::op::binary::is_binary_op;
//
// ---------------------------------------------------------------------------
#[derive(Eq, Hash, PartialEq)]
struct BinaryOp {
    op_id     : u8          ,
    arg       : [IndexT; 2] ,
    arg_type  : [ADType; 2] ,
}
impl BinaryOp {
    pub fn new(
        op_id_in    : u8        ,
        arg_in      : &[IndexT] ,
        arg_type_in : &[ADType] ,
    ) -> Self {
        debug_assert!( is_binary_op(op_id_in) );
        debug_assert!( arg_in.len() ==  2 && arg_type_in.len() == 2 );
        Self {
            op_id    : op_id_in                                           ,
            arg      : [ arg_in[0],              arg_in[1]              ] ,
            arg_type : [ arg_type_in[0].clone(), arg_type_in[1].clone() ] ,
        }
    }
}
// ---------------------------------------------------------------------------
//
// ADfn::compress_dyp
impl<V> ADfn<V>
where
    V : Clone + Eq + std::fmt::Display + std::hash::Hash,
{   //
    // compress_dyp
    /// Determine and compress a set of identical dynamic parameters
    /// so that only one element of the set is used.
    ///
    /// * Syntax :
    /// ```text
    ///     f.compress_dyp(depend, trace)
    /// ```
    ///
    /// * f :
    /// The [ADfn] object for which the constants are compressed.
    ///
    /// * depend :
    /// On input, this is the [Depend] structure for the input f .
    /// only the constants for which depend.cop is true are included.
    ///
    /// Upon return, if two or more dynamic parameters are identical,
    /// depend.cop, f.dyp.arg_all, and f.var.arg_all are modified so that
    /// only one of the equal dynamic parameters is used by f.
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
        // binary_hash_map
        let mut binary_hash_map : FxHashMap<BinaryOp, IndexT> =
            FxHashMap::default();
        //
        if trace {
            println!("Begin Trace compress_dyp");
            println!("original_index, compressed_index");
        }
        //
        // binary_hash_map, depend.dyp
        for op_index in 0 .. n_dep {
            let res = op_index + self.dyp.n_dom;
            if depend.dyp[res] {
                let op_id     = self.dyp.id_all[op_index];
                let start     = self.dyp.arg_start[op_index] as usize;
                let end       = self.dyp.arg_start[op_index + 1] as usize;
                let arg       = &self.dyp.arg_all[start .. end];
                let arg_type  = &self.dyp.arg_type_all[start .. end];
                if is_binary_op( op_id ) {
                    let key = BinaryOp::new( op_id, arg, arg_type);
                    if ! binary_hash_map.contains_key(&key) {
                        binary_hash_map.insert(key, res as IndexT );
                    } else {
                        depend.dyp[res] = false;
                    }
                    if trace {
                        let key   = BinaryOp::new( op_id, arg, arg_type);
                        let index = binary_hash_map.get(&key).unwrap();
                        println!("{}, {}", res, index);
                    }
                }
            }
        }
        //
        // self.rng_index
        for i in 0 .. self.rng_index.len() {
            if self.rng_ad_type[i].is_dynamic() {
                let res      = self.rng_index[i] as usize;
                if self.dyp.n_dom <= res {
                    let op_index  = res - self.dyp.n_dom;
                    let op_id     = self.dyp.id_all[op_index];
                    let start     = self.dyp.arg_start[op_index] as usize;
                    let end       = self.dyp.arg_start[op_index + 1] as usize;
                    let arg       = &self.dyp.arg_all[start .. end];
                    let arg_type  = &self.dyp.arg_type_all[start .. end];
                    if is_binary_op( op_id ) {
                        let key   = BinaryOp::new( op_id, arg, arg_type);
                        let index = binary_hash_map.get(&key).unwrap();
                        self.rng_index[i] = *index;
                    }
                }
            }
        }
        //
        // self.dyp.arg_all
        for op_index in 0 .. self.dyp.n_dep {
            //
            // res
            let res = op_index + self.dyp.n_dom;
            if depend.dyp[res] {
                //
                let start         = self.dyp.arg_start[op_index] as usize;
                let end           = self.dyp.arg_start[op_index + 1] as usize;
                let (left, right) = self.dyp.arg_all.split_at_mut(start);
                //
                let arg        = &mut right[0 .. end - start];
                let arg_type   = &self.dyp.arg_type_all[start .. end];
                for i_arg in 0 .. arg.len() {
                    let res_     = arg[i_arg] as usize;
                    let mut skip = arg_type[i_arg] != ADType::DynamicP;
                    skip         = skip || res_ < self.dyp.n_dom;
                    if ! skip {
                        let op_index_ = res_ - self.dyp.n_dom;
                        let op_id_ = self.dyp.id_all[op_index_];
                        let start_ = self.dyp.arg_start[op_index_] as usize;
                        let end_   = self.dyp.arg_start[op_index_ + 1] as usize;
                        let arg_   = &left[start_ .. end_];
                        let arg_type_ = &self.dyp.arg_type_all[start_ .. end_];
                        if is_binary_op( op_id_ ) {
                            let key   = BinaryOp::new( op_id_, arg_, arg_type_);
                            let index = binary_hash_map.get(&key).unwrap();
                            //
                            // self.dyp.arg_all
                            arg[i_arg] = *index;
                        }
                    }
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
                    let res_     = arg[i_arg] as usize;
                    let mut skip = arg_type[i_arg] != ADType::DynamicP;
                    skip         = skip || res_ < self.dyp.n_dom;
                    if ! skip {
                        let op_index_ = res_ - self.dyp.n_dom;
                        let op_id_ = self.dyp.id_all[op_index_];
                        let start_ = self.dyp.arg_start[op_index_] as usize;
                        let end_   = self.dyp.arg_start[op_index_ + 1] as usize;
                        let arg_      = &self.dyp.arg_all[start_ .. end_];
                        let arg_type_ = &self.dyp.arg_type_all[start_ .. end_];
                        if is_binary_op( op_id_ ) {
                            let key   = BinaryOp::new( op_id_, arg_, arg_type_);
                            let index = binary_hash_map.get(&key).unwrap();
                            //
                            // self.dyp.arg_all
                            arg[i_arg] = *index;
                        }
                    }
                }
            }
        }
        if trace {
            println!("End Trace compress_dyp");
        }
    }
}
