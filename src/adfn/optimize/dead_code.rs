// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] dead_code method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    IndexT,
};
use crate::adfn::optimize;
use crate::tape::Tape;
use crate::tape::OpSequence;
use crate::op::binary::is_binary_op;
use crate::ad::ADType;
// -----------------------------------------------------------------------
//
// ADfn::dead_code
impl<V> ADfn<V>
where
    V : Clone,
{   //
    // dead_code
    /// Determine a new tape and map from ADfn indices to tape indices.
    pub(crate) fn dead_code(&self, depend : &optimize::Depend) -> Tape<V> {
        //
        // tape
        let mut tape : Tape<V> = Tape::new();
        //
        // n_cop, n_dyp, n_var
        let n_cop = self.cop_len();
        let n_dyp = self.dyp_len();
        let n_var = self.var_len();
        //
        // renumber
        // initialize as an invalid value
        let mut renumber = optimize::Renumber{
                cop : vec![ n_cop as IndexT; n_cop ] ,
                dyp : vec![ n_dyp as IndexT; n_dyp ] ,
                var : vec![ n_var as IndexT; n_var ] ,
        };
        //
        // tape.cop, renumber.cop
        let n_cop = self.cop_len();
        for old_index in 0 .. n_cop {
            if depend.cop[old_index] {
                let value               = self.cop[old_index].clone();
                let new_index           = tape.cop.len();
                renumber.cop[old_index] = new_index as IndexT;
                tape.cop.push( value );
            }
        }
        //
        // i_op_seq
        for i_op_seq in 0 .. 2 {
            let old_depend  : &Vec<bool>;
            let old_op_seq  : &OpSequence;
            let new_op_seq  : &mut OpSequence;
            if i_op_seq == 0 {
                old_depend = &depend.dyp;
                old_op_seq = &self.dyp;
                new_op_seq = &mut tape.dyp;
            } else {
                old_depend = &depend.var;
                old_op_seq = &self.var;
                new_op_seq = &mut tape.var;
            };
            //
            // new_op_seq.n_dom, renumber.dyp
            let n_dom        = old_op_seq.n_dom;
            new_op_seq.n_dom = n_dom;
            for old_index in 0 .. n_dom {
                renumber.dyp[old_index] = old_index as IndexT;
            }
            //
            // op_index, first_op
            let mut op_index = 0;
            let mut first_op = true;
            while op_index < old_op_seq.n_dep {
                if first_op {
                    first_op = false;
                } else {
                    op_index += 1;
                }
                //
                // res
                let res  = op_index + old_op_seq.n_dom;
                if old_depend[res] {
                    //
                    // op_id
                    let op_id     = self.dyp.id_seq[op_index];
                    if is_binary_op(op_id) {
                        //
                        // arg, arg_type
                        let start = old_op_seq.arg_seq[op_index] as usize;
                        let end   = old_op_seq.arg_seq[op_index + 1] as usize;
                        let arg       = &old_op_seq.arg_all[start .. end];
                        let arg_type  = &old_op_seq.arg_type_all[start .. end];
                        assert!( arg.len() == 2 );
                        //
                        // op_sed.id_seq
                        new_op_seq.id_seq.push( op_id );
                        //
                        for i_arg in 0 .. 2 {
                            let arg_type_i = arg_type[i_arg].clone();
                            let old_index = arg[i_arg] as usize;
                            let new_index : IndexT;
                            match arg_type_i {
                                ADType::ConstantP => {
                                    new_index = renumber.cop[old_index];
                                    assert_ne!(
                                        new_index as usize, renumber.cop.len()
                                    );
                                },
                                ADType::DynamicP  => {
                                    new_index = renumber.dyp[old_index];
                                    assert_ne!(
                                        new_index as usize, renumber.dyp.len()
                                    );
                                },
                                ADType::Variable => {
                                    new_index = renumber.var[old_index];
                                    assert_ne!(
                                        new_index as usize, renumber.var.len()
                                    );
                                    assert_ne!( i_op_seq, 0 );
                                },
                                _  => {
                                    panic!("dead_code: binary operator error")
                                },
                            }
                            new_op_seq.arg_all.push( new_index );
                            new_op_seq.arg_type_all.push( arg_type_i );
                        }
                    } else {
                        // CALL_OP not yet implemented
                        assert!( false );
                    }
                } // if old_depend[res]
            } // while op_index < n_dep
        } // for i_op_seq in 0 .. 2
        return tape;
    } // fn dead_code
} // impl
