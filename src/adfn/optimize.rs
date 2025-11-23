// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] optimize method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn
};
use crate::op::info::sealed::GlobalOpInfoVec;
use crate::ad::ADType;
use crate::tape::OpSequence;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
//
// -----------------------------------------------------------------------
// OptimizeDepend
/// Which constants, dynamic parameters, and variables the
/// range for an [ADfn] depends on.
///
/// TODO: change to private when reverse_depend gets changes to private.
pub struct OptimizeDepend {
    // cop
    /// Constant parameters dependency; length [ADfn::cop_len].
    pub(crate) cop : Vec<bool> ,
    //
    // dyp
    /// Dynamic parameters dependency; length [ADfn::dyp_len].
    pub(crate) dyp : Vec<bool> ,
    //
    // var
    /// Variable dependency; length [ADfn::var_len].
    pub(crate) var : Vec<bool> ,
}
//
// ADfn::reverse_depend
impl<V> ADfn<V>
where
    V : GlobalOpInfoVec ,
{   //
    // reverse_depend
    /// Determine [OptimizeDepend] for this [ADfn].
    /// TODO: change to privae when this gets used by a public function.
    pub fn reverse_depend(&self) -> OptimizeDepend {
        //
        // n_cop, n_dyp, n_var, range_ad_type, range_index
        let n_cop         = self.cop_len();
        let n_dyp         = self.dyp_len();
        let n_var         = self.var_len();
        let range_ad_type = &self.range_ad_type;
        let range_index   = &self.range_index;
        //
        // depend
        let mut depend = OptimizeDepend {
            cop : vec![false; n_cop ],
            dyp : vec![false; n_dyp ],
            var : vec![false; n_var ],
        };
        //
        // depend
        for i in 0 .. self.range_ad_type.len() {
            let index = range_index[i] as usize;
            match range_ad_type[i] {
                ADType::ConstantP => {},
                ADType::DynamicP  => { depend.dyp[index] = true; },
                ADType::Variable  => { depend.var[index] = true; },
                _ => panic!( "reverse_depend: expected an AD type."),
            }
        }
        //
        // op_info_vec
        let op_info_vec = &*<V as GlobalOpInfoVec>::get();
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
            // op_index
            for op_index in (0 .. n_dep).rev() {
                let op_id     = op_seq.id_seq[op_index] as usize;
                let start     = op_seq.arg_seq[op_index] as usize;
                let end       = op_seq.arg_seq[op_index + 1] as usize;
                let arg       = &op_seq.arg_all[start .. end];
                let arg_type  = &op_seq.arg_type_all[start .. end];
                let res       = n_dom + op_index;

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
        }
        depend
    }
}
