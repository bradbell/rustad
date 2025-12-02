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
use crate::adfn::optimize::Depend;
use crate::adfn::optimize::Renumber;
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
    /// Determine a new tape, with dead code removed,
    /// and the map from ADfn indices to tape indices.
    ///
    /// * Syntax :
    /// ```text
    ///     (tape, renumber) = f.dead_code(depend, trace)
    /// ```
    ///
    /// * f :
    /// is an [ADfn] object.
    ///
    /// * depend :
    /// is the [Depend] structure for *f* .
    ///
    /// * trace :
    /// if true, a trace is printed on standard output.
    ///
    /// * tape :
    /// is a [Tape] corresponding to the optimized version of *f* .
    ///
    /// * renumber :
    /// is the [Renumber] structure that maps indices in *f* to
    /// indices in *tape* .
    ///
    pub(crate) fn dead_code(&self, depend : &Depend, trace : bool,
    ) -> ( Tape<V>, Renumber) {
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
        let mut renumber = Renumber{
                cop : vec![ n_cop as IndexT; n_cop ] ,
                dyp : vec![ n_dyp as IndexT; n_dyp ] ,
                var : vec![ n_var as IndexT; n_var ] ,
        };
        //
        if trace {
            println!( "Begin Trace: dead_code");
            println!(
                "n_cop = {}, n_dyp = {}, n_var = {}", n_cop, n_dyp, n_var
            );
            if 0 < n_cop {
                println!( "old_cop_index, new_cop_index" );
            }
        }
        //
        // tape.cop, renumber.cop
        let n_cop = self.cop_len();
        for old_index in 0 .. n_cop {
            if depend.cop[old_index] {
                let value               = self.cop[old_index].clone();
                let new_index           = tape.cop.len();
                renumber.cop[old_index] = new_index as IndexT;
                tape.cop.push( value );
                println!( "{}, {}", old_index, new_index );
            }
        }
        //
        // i_op_seq
        let start_op_seq = if n_dyp > 0 { 0 } else { 1 };
        for i_op_seq in start_op_seq .. 2 {
            let old_depend  : &Vec<bool>;
            let old_op_seq  : &OpSequence;
            let new_op_seq  : &mut OpSequence;
            if i_op_seq == 0 {
                old_depend = &depend.dyp;
                old_op_seq = &self.dyp;
                new_op_seq = &mut tape.dyp;
                if trace {
                    println!( "old_dyp_index, new_dyp_index" );
                }
            } else {
                old_depend = &depend.var;
                old_op_seq = &self.var;
                new_op_seq = &mut tape.var;
                if trace {
                    println!( "old_var_index, new_var_index" );
                }
            };
            //
            // new_op_seq.n_dom, renumber.dyp
            let n_dom        = old_op_seq.n_dom;
            new_op_seq.n_dom = n_dom;
            for old_index in 0 .. n_dom {
                if i_op_seq == 0 {
                    renumber.dyp[old_index] = old_index as IndexT;
                } else {
                    renumber.var[old_index] = old_index as IndexT;
                }
            }
            //
            // op_index, first_op
            let mut op_index = 0;
            while op_index < old_op_seq.n_dep {
                //
                // res, op_id
                let res    = op_index + old_op_seq.n_dom;
                let op_id  = old_op_seq.id_seq[op_index];
                //
                if old_depend[res] {
                    //
                    if is_binary_op(op_id) {
                        //
                        // arg, arg_type
                        let start = old_op_seq.arg_seq[op_index] as usize;
                        let end   = old_op_seq.arg_seq[op_index + 1] as usize;
                        let arg       = &old_op_seq.arg_all[start .. end];
                        let arg_type  = &old_op_seq.arg_type_all[start .. end];
                        assert!( arg.len() == 2 );
                        //
                        // new_op_seq: id_seq, arg_seq, n_dep
                        new_op_seq.n_dep += 1;
                        new_op_seq.id_seq.push( op_id );
                        new_op_seq.arg_seq.push(
                            new_op_seq.arg_all.len() as IndexT
                        );
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
                        // renumber
                        let new_op_index = new_op_seq.id_seq.len() - 1;
                        let new_index    = new_op_index + new_op_seq.n_dom;
                        let old_index    = op_index     + old_op_seq.n_dom;
                        if i_op_seq == 0 {
                            renumber.dyp[old_index] = new_index as IndexT;
                        } else {
                            renumber.var[old_index] = new_index as IndexT;
                        }
                        if trace {
                            println!( "{}, {}", old_index, new_index );
                        }
                    } else {
                        panic!( "dead_code: op_id = {}", op_id );
                    }
                } // if old_depend[res]
                if is_binary_op(op_id ) {
                    op_index += 1;
                } else {
                    panic!( "dead_code: op_id = {}", op_id );
                }
            } // while op_index <
        } // for i_op_seq in 0 .. 2
        if trace {
            println!( "End Trace: dead_code" );
        }
        return (tape, renumber);
    } // fn dead_code
} // impl
