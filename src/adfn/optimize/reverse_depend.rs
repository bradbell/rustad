// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] reverse_depend method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    AtomCallback,
    IndexT,
};
use crate::op::{
    info::sealed::GlobalOpInfoVec,
    call::call_depend,
    id::CALL_OP,
    id::CALL_RES_OP,
};
use crate::ad::ADType;
use crate::tape::OpSequence;
use crate::atom::sealed::AtomInfoVec;
use crate::adfn::optimize;
//
// -----------------------------------------------------------------------
//
// ADfn::reverse_depend
impl<V> ADfn<V>
where
    V               : AtomInfoVec + GlobalOpInfoVec,
    AtomCallback<V> : Clone,
{   //
    // reverse_depend
    /// Determine [optimize::Depend] for this [ADfn].
    pub(crate) fn reverse_depend(&self, trace : bool) -> optimize::Depend {
        //
        // atom_depend, cop_depend, dyp_depend, var_depend
        // work space used to avoid reallocationg vectors
        let mut atom_depend : Vec<usize>  = Vec::new();
        let mut cop_depend  : Vec<IndexT> = Vec::new();
        let mut dyp_depend  : Vec<IndexT> = Vec::new();
        let mut var_depend  : Vec<IndexT> = Vec::new();
        //
        // n_cop, n_dyp, n_var, rng_ad_type, rng_index
        let n_cop         = self.cop_len();
        let n_dyp         = self.dyp_len();
        let n_var         = self.var_len();
        let rng_ad_type   = &self.rng_ad_type;
        let rng_index     = &self.rng_index;
        //
        // depend
        let mut depend = optimize::Depend {
            cop : vec![false; n_cop ],
            dyp : vec![false; n_dyp ],
            var : vec![false; n_var ],
        };
        //
        if trace {
            println!( "Begin Trace: reverse_depend" );
            println!(
                "n_cop = {}, n_dyp = {}, n_var = {}", n_cop, n_dyp, n_var
            );
            println!( "rng_index, type_index, type" );
        }
        //
        // depend
        for i in 0 .. self.rng_ad_type.len() {
            let index = rng_index[i] as usize;
            match rng_ad_type[i] {
                ADType::ConstantP => { depend.cop[index] = true; },
                ADType::DynamicP  => { depend.dyp[index] = true; },
                ADType::Variable  => { depend.var[index] = true; },
                _ => panic!( "reverse_depend: expected an AD type."),
            }
            //
            if trace {
                println!( "{}, {}, {:?}", i, index, &rng_ad_type[i])
            }
        }
        if trace {
            println!( "res, res_type, name, arg, arg_type" )
        }
        //
        // op_info_vec
        let op_info_vec = &*<V as GlobalOpInfoVec>::get();
        //
        //
        // i_op_seq
        for i_op_seq in 0 .. 2 {
            //
            // op_seq
            let op_seq   : &OpSequence;
            let res_type : ADType;
            if i_op_seq == 0 {
                op_seq    = &self.var;
                res_type  = ADType::Variable;
            } else {
                op_seq    = &self.dyp;
                res_type  = ADType::DynamicP;
            }
            //
            // n_dep, flag_all
            let n_dom    = op_seq.n_dom;
            let n_dep    = op_seq.n_dep;
            let flag_all = &op_seq.flag_all;
            //
            // op_index, op_id
            for op_index in (0 .. n_dep).rev() {
                let op_id     = op_seq.id_seq[op_index] as usize;
                let start     = op_seq.arg_seq[op_index] as usize;
                let end       = op_seq.arg_seq[op_index + 1] as usize;
                let arg       = &op_seq.arg_all[start .. end];
                let arg_type  = &op_seq.arg_type_all[start .. end];
                let res       = n_dom + op_index;
                //
                if op_id == CALL_OP as usize || op_id == CALL_RES_OP as usize {
                    cop_depend.clear();
                    dyp_depend.clear();
                    var_depend.clear();
                    call_depend::<V>(
                        &mut atom_depend,
                        &mut cop_depend,
                        &mut dyp_depend,
                        &mut var_depend,
                        &self.var,
                        op_index
                     );
                    for dep_index in var_depend.iter() {
                        depend.var[*dep_index as usize] = true;
                    }
                    for dep_index in dyp_depend.iter() {
                        depend.dyp[*dep_index as usize] = true;
                    }
                    for dep_index in cop_depend.iter() {
                        depend.cop[*dep_index as usize] = true;
                    }
                } else {
                    let reverse_depend = op_info_vec[op_id].reverse_depend;
                    reverse_depend(
                        &mut depend,
                        &flag_all,
                        &arg,
                        &arg_type,
                        res,
                        res_type.clone(),
                    );
                }
                if trace {
                    let name = &op_info_vec[op_id].name;
                    println!(
                        "{}, {:?}, {}, {:?}, {:?}",
                        res, res_type, name, arg, arg_type
                    )
                }
            }
        }
        if trace {
            println!( "depend.cop = {:?}", depend.cop );
            println!( "depend.dyp = {:?}", depend.dyp );
            println!( "depend.var = {:?}", depend.var );
        }
        depend
    }
}
