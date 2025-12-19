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
use crate::tape::OpSequence;
use crate::op::binary::is_binary_op;
use crate::op::id::{
    CALL_OP,
    CALL_RES_OP
};
use crate::op::call::{
    BEGIN_FLAG,
    NUMBER_RNG,
};
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
#[derive(Eq, Hash, PartialEq)]
struct CallOp {
    arg         : Vec<IndexT> ,
    arg_type    : Vec<ADType> ,
    flag        : Vec<bool>   ,
}
impl CallOp {
    pub fn new(
        arg_in          : &[IndexT]  ,
        arg_type_in     : &[ADType]  ,
        flag_in         : &[bool]    ,
    ) -> Self {
        debug_assert!( arg_in.len() == arg_type_in.len() );
        Self {
            arg         : arg_in.to_vec()      ,
            arg_type    : arg_type_in.to_vec() ,
            flag        : flag_in.to_vec()     ,
        }
    }
}
// ---------------------------------------------------------------------------
/// A hash map that identifies identical operator uses; i.e.,
/// operators uses that will always yield the same results.
///
struct OpHashMap {
    binary_hash_map : FxHashMap<BinaryOp, IndexT> ,
    call_hash_map   : FxHashMap<CallOp, IndexT> ,
}
//
impl OpHashMap {
    //
    // OpHashMap::new
    fn new() -> Self {
        Self {
            binary_hash_map : FxHashMap::default() ,
            call_hash_map   : FxHashMap::default() ,
        }
    }
    //
    // OpHashMap::try_insert
    /// Try to insert an operator in this hash map.
    ///
    /// * Syntax :
    /// ```text
    ///     option = op_hash_map.try_insert(op_seq, op_index, map_value_in)
    /// ```
    ///
    /// * op_seq :
    /// is the operation sequence that this operator appears in.
    ///
    /// * op_index :
    /// is the index of this operator in the operation sequence.
    ///
    /// * map_value_in :
    /// If the hash map does not contain this operator,
    /// it is inserted with this value.
    /// This value is different for each call to try_insert.
    ///
    /// * option :
    ///     * None : try_insert only handles the following operators:
    ///         binary operators, CALL_OP operators.
    ///         If this is not one of these, try_insert returns None.
    ///     * Some(map_value_out) : If map_value_out is equal to map_value_in,
    ///         this operator was inserted in the hash map
    ///         with the specified value. Otherwise,
    ///         this operation is equivalent to a previous operator and
    ///         is map_value_out is its map value.
    ///
    fn try_insert(
        &mut self                  ,
        op_seq       : &OpSequence ,
        op_index     : usize       ,
        map_value_in : IndexT      ,
    ) -> Option<IndexT> {
        let op_id    = op_seq.id_all[op_index];
        let start    = op_seq.arg_start[op_index] as usize;
        let end      = op_seq.arg_start[op_index + 1] as usize;
        let arg      = &op_seq.arg_all[start .. end];
        let arg_type = &op_seq.arg_type_all[start .. end];
        if is_binary_op(op_id) {
            let key           = BinaryOp::new(op_id, arg, arg_type);
            let map_value_out =
                self.binary_hash_map.entry(key).or_insert(map_value_in);
            return Some(*map_value_out);
        } else if op_id == CALL_OP {
            let n_rng = arg[NUMBER_RNG] as usize;
            let start = arg[BEGIN_FLAG] as usize;
            let end   = start + 1 + n_rng;
            let flag  = &op_seq.flag_all[start .. end];
            let key   = CallOp::new(arg, arg_type, flag);
            let map_value_out =
                self.call_hash_map.entry(key).or_insert(map_value_in);
            return Some(*map_value_out);
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
        // first_match
        let mut first_match : Vec<IndexT> = Vec::with_capacity(n_dep);
        for op_index in 0 .. n_dep {
            first_match.push( op_index as IndexT );
        }
        //
        // op_hash_map
        let mut op_hash_map : OpHashMap = OpHashMap::new();
        //
        // new_arg
        let mut new_arg : Vec<IndexT> = Vec::new();
        //
        // id_all
        let id_all = &self.dyp.id_all;
        //
        if trace {
            println!("Begin Trace compress_dyp");
            println!("original_index, compressed_index");
        }
        //
        // op_index, increment, op_hash_map, depend.dyp
        let mut op_index = 0;
        while op_index < n_dep {
            let mut increment = 1;
            //
            if depend.dyp[op_index + n_dom] {
                //
                // op_index
                if id_all[op_index] == CALL_RES_OP {
                    let start    = self.dyp.arg_start[op_index] as usize;
                    let offset   = self.dyp.arg_all[start] as usize;
                    op_index     = op_index - offset;
                    debug_assert!( id_all[op_index] == CALL_OP );
                }
                if id_all[op_index] == CALL_OP {
                    let mut n_call = 1;
                    while op_index + n_call < n_dep &&
                        id_all[op_index + n_call] == CALL_RES_OP {
                        n_call += 1;
                    }
                    let map_value_in = op_index as IndexT;
                    let option = op_hash_map.try_insert(
                        &self.dyp, op_index, map_value_in
                    );
                    let map_value_out = option.unwrap();
                    let new_op        = map_value_out == map_value_in;
                    //
                    for i_call in 0 .. n_call {
                        let dep_index = op_index + i_call;
                        let dep_match = map_value_out + (i_call as IndexT);
                        first_match[dep_index] = dep_match;
                    }
                    //
                    if ! new_op { for i_call in 0 .. n_call {
                        depend.dyp[op_index + i_call] = false;
                    } }
                    //
                    // increment
                    increment = n_call;
                } else {
                    let map_value_in = op_index as IndexT;
                    let option = op_hash_map.try_insert(
                        &self.dyp, op_index, map_value_in
                    );
                    if option.is_some() {
                        let map_value_out = option.unwrap();
                        let new_op        = map_value_out == map_value_in;
                        if ! new_op {
                            depend.dyp[op_index + n_dom] = false;
                        }
                        let dyp_index = op_index;
                        first_match[dyp_index] = map_value_out;
                    }
                }
            }
            // op_index
            op_index += increment;
        }
        //
        // self.rng_index
        for i in 0 .. self.rng_index.len() {
            let dynamic   = self.rng_ad_type[i].is_dynamic();
            let dependent = dynamic && n_dom_indext <= self.rng_index[i];
            if dependent {
                let dyp_index     = self.rng_index[i] as usize;
                let op_index      = dyp_index - n_dom;
                self.rng_index[i] = first_match[op_index] + n_dom_indext;
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
                        new_arg.push( first_match[dep_index] + n_dom_indext );
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
                            arg[i_arg] = first_match[dep_index] + n_dom_indext;
                    }
                }
            }
        }
        if trace {
            println!("End Trace compress_dyp");
        }
    }
}
