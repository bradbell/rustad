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
use crate::tape::OpSequence;
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
struct OpHashMap {
    binary_hash_map : FxHashMap<BinaryOp, IndexT> ,
}
//
impl OpHashMap {
    //
    // OpHashMap::new
    fn new() -> Self {
        Self {
            binary_hash_map : FxHashMap::default() ,
        }
    }
    //
    // OpHashMap::try_insert
    /// Try to insert an operator in this hash map.
    ///
    /// * Syntax :
    /// ```text
    ///     option = op_hash_map.try_insert(op_seq, op_index, map_value)
    /// ```
    ///
    /// * op_seq :
    /// is the operation sequence that this operator appears in.
    ///
    /// * op_index :
    /// is the index of this operator in the operation sequence.
    ///
    /// * map_value :
    /// If the hash map does not already contain this operator,
    /// it is inserted with this value.
    ///
    /// * option :
    ///     * None : OpHashMap does not handle this operator.
    ///     * Some(true) : this operator is inserted in the hash map
    ///         with the specified value.
    ///     * Some(false) : this operator is already in the hash map
    ///         and its corresponding is not changed.
    ///
    fn try_insert(
        &mut self                  ,
        op_seq       : &OpSequence ,
        op_index     : usize       ,
        map_value    : IndexT      ,
    ) -> Option<bool> {
        let op_id    = op_seq.id_all[op_index];
        let start    = op_seq.arg_start[op_index] as usize;
        let end      = op_seq.arg_start[op_index + 1] as usize;
        let arg      = &op_seq.arg_all[start .. end];
        let arg_type = &op_seq.arg_type_all[start .. end];
        if is_binary_op(op_id) {
            let key = BinaryOp::new(op_id, arg, arg_type);
            if self.binary_hash_map.contains_key(&key) {
                return Some(false);
            }
            self.binary_hash_map.insert(key, map_value);
            return Some(true);
        }
        None
    }
    //
    // OpHashMap::get
    /// Get the map value corresponding to an operator.
    ///
    /// * Syntax :
    /// ```text
    ///     option = op_hash_map.get(op_seq, op_index)
    /// ```
    ///
    /// * return :
    /// If this operator is not in the hash map, or this operator is not
    /// handled by OpHashMap, None is returned.
    /// Otherwise it is assumed that this operator is in the hash map and
    /// Some(map_value) for this operator is returned.
    fn get(
        &self                   ,
        op_seq    : &OpSequence ,
        op_index  : usize       ,
    ) -> Option<IndexT> {
        let op_id    = op_seq.id_all[op_index];
        let start    = op_seq.arg_start[op_index] as usize;
        let end      = op_seq.arg_start[op_index + 1] as usize;
        let arg      = &op_seq.arg_all[start .. end];
        let arg_type = &op_seq.arg_type_all[start .. end];
        if is_binary_op(op_id) {
            let key       = BinaryOp::new(op_id, arg, arg_type);
            let map_value = *self.binary_hash_map.get(&key).unwrap();
            return Some(map_value);
        }
        return None;
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
        // op_hash_map
        let mut op_hash_map : OpHashMap = OpHashMap::new();
        //
        // new_arg
        let mut new_arg : Vec<IndexT> = Vec::new();
        //
        if trace {
            println!("Begin Trace compress_dyp");
            println!("original_index, compressed_index");
        }
        //
        // op_hash_map, depend.dyp
        for op_index in 0 .. n_dep {
            let res = op_index + self.dyp.n_dom;
            if depend.dyp[res] {
                let map_value = res as IndexT;
                let option   =
                    op_hash_map.try_insert(&self.dyp, op_index, map_value);
                if option.is_some() {
                    let new_op = option.unwrap();
                    if ! new_op {
                        depend.dyp[res] = false;
                    }
                    if trace {
                        let option = op_hash_map.get(&self.dyp, op_index);
                        let index  = option.unwrap();
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
                    let option    = op_hash_map.get(&self.dyp, op_index);
                    if option.is_some() {
                        self.rng_index[i] = option.unwrap();
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
                new_arg.clear();
                //
                let start      = self.dyp.arg_start[op_index] as usize;
                let end        = self.dyp.arg_start[op_index + 1] as usize;
                let arg        = &self.dyp.arg_all[start .. end];
                let arg_type   = &self.dyp.arg_type_all[start .. end];
                for i_arg in 0 .. arg.len() {
                    let res_   = arg[i_arg] as usize;
                    let dep    = self.dyp.n_dom <=  res_;
                    let map    = arg_type[i_arg].is_dynamic() && dep ;
                    if map {
                        let op_index_ = res_ - self.dyp.n_dom;
                        let option    = op_hash_map.get(&self.dyp, op_index_);
                        if option.is_some() {
                            // self.dyp.arg_all
                            new_arg.push( option.unwrap() );
                        } else {
                            new_arg.push( arg[i_arg] );
                        }
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
                    let res_     = arg[i_arg] as usize;
                    let mut skip = arg_type[i_arg] != ADType::DynamicP;
                    skip         = skip || res_ < self.dyp.n_dom;
                    if ! skip {
                        let op_index_ = res_ - self.dyp.n_dom;
                        let option    = op_hash_map.get(&self.dyp, op_index_);
                        if option.is_some() {
                            // self.dyp.arg_all
                            arg[i_arg] = option.unwrap();
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
