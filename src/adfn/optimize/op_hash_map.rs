// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement [OpHashMap]
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
use rustc_hash::FxHashMap;
//
use crate::{
    IndexT,
};
use crate::ad::ADType;
use crate::tape::OpSequence;
use crate::op::binary::is_binary_op;
use crate::op::id::CALL_OP;
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
        op_id_in    : u8          ,
        arg_in      : [IndexT; 2] ,
        arg_type_in : &[ADType]   ,
    ) -> Self {
        debug_assert!( is_binary_op(op_id_in) );
        debug_assert!( arg_type_in.len() == 2 );
        Self {
            op_id    : op_id_in                                           ,
            arg      : arg_in                                             ,
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
        arg_in          : Vec<IndexT> ,
        arg_type_in     : &[ADType]   ,
        flag_in         : &[bool]     ,
    ) -> Self {
        debug_assert!( arg_in.len() == arg_type_in.len() );
        Self {
            arg         : arg_in               ,
            arg_type    : arg_type_in.to_vec() ,
            flag        : flag_in.to_vec()     ,
        }
    }
}
// ---------------------------------------------------------------------------
/// A hash map that identifies identical operator uses; i.e.,
/// operators uses that will always yield the same results.
///
pub(crate) struct OpHashMap {
    binary_hash_map : FxHashMap<BinaryOp, IndexT> ,
    call_hash_map   : FxHashMap<CallOp, IndexT> ,
}
//
impl OpHashMap {
    //
    // OpHashMap::new
    pub(crate) fn new() -> Self {
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
    ///     option = op_hash_map.try_insert(
    ///         op_seq, op_seq_type, op_index, &first_match, map_value_in
    ///     )
    /// ```
    ///
    /// * op_seq :
    /// is the operation sequence that this operator appears in.
    ///
    /// * op_seq_type :
    /// is the type of this operation sequence. Must be one of
    /// ADType::DynamicP, ADType::VariableP.
    ///
    /// * op_index :
    /// is the index of this operator in the operation sequence.
    ///
    /// * first_match :
    /// For each operator index i_op < op_index,
    /// first_match\[i_op\] is the index of the first operator
    /// that is known to be equivalent to the operator with index i_op.
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
    pub(crate) fn try_insert(
        &mut self                   ,
        op_seq       : &OpSequence  ,
        op_seq_type  : ADType       ,
        op_index     : usize        ,
        first_match  : &Vec<IndexT> ,
        map_value_in : IndexT       ,
    ) -> Option<IndexT> {
        let n_dom_indext = op_seq.n_dom as IndexT;
        let op_id        = op_seq.id_all[op_index];
        let start        = op_seq.arg_start[op_index] as usize;
        let end          = op_seq.arg_start[op_index + 1] as usize;
        let arg          = &op_seq.arg_all[start .. end];
        let arg_type     = &op_seq.arg_type_all[start .. end];
        if is_binary_op(op_id) {
            //
            // arg_0
            let match_0 = arg_type[0] == op_seq_type && n_dom_indext <= arg[0];
            let arg_0   = if match_0 {
                let dep_index = (arg[0] - n_dom_indext) as usize;
                first_match[dep_index] + n_dom_indext
            } else {
                arg[0]
            };
            //
            // arg_1
            let match_1 = arg_type[1] == op_seq_type && n_dom_indext <= arg[1];
            let arg_1   = if match_1 {
                let dep_index = (arg[1] - n_dom_indext) as usize;
                first_match[dep_index] + n_dom_indext
            } else {
                arg[1]
            };
            let arg_match     = [arg_0, arg_1];
            let key           = BinaryOp::new(op_id, arg_match, arg_type);
            let map_value_out =
                self.binary_hash_map.entry(key).or_insert(map_value_in);
            return Some(*map_value_out);
        } else if op_id == CALL_OP {
            let n_rng         = arg[NUMBER_RNG] as usize;
            let start         = arg[BEGIN_FLAG] as usize;
            let end           = start + 1 + n_rng;
            let flag          = &op_seq.flag_all[start .. end];
            let mut arg_match = arg.to_vec();
            for i_arg in 0 .. arg_match.len() {
                let match_i = arg_type[i_arg] == op_seq_type &&
                    n_dom_indext <= arg[i_arg];
                if match_i {
                    let dep_index    = (arg[i_arg] - n_dom_indext) as usize;
                    arg_match[i_arg] = first_match[dep_index] + n_dom_indext;
                }
            }
            // position where flags start does not matter.
            arg_match[BEGIN_FLAG] = 0;
            let key   = CallOp::new(arg_match, arg_type, flag);
            let map_value_out =
                self.call_hash_map.entry(key).or_insert(map_value_in);
            return Some(*map_value_out);
        }
        return None;
    }
}
