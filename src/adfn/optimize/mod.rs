// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] optimization methods.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    ADfn,
    IndexT,
    AtomInfoVecPublic,
    GlobalOpInfoVecPublic,
};
//
use crate::ad::ADType;
use crate::tape::OpSequence;
//
// -----------------------------------------------------------------------
// mod
mod op_hash_map;
mod reverse_depend;
mod compress_cop;
mod compress_dyp;
mod dead_code;
// -----------------------------------------------------------------------
// Depend
/// Which constants, dynamic parameters, and variables the
/// range for an [ADfn] depends on.
///
pub(crate) struct Depend {
    // cop
    /// Constant parameters dependency; length [ADfn::cop_len].
    pub cop : Vec<bool> ,
    //
    // dyp
    /// Dynamic parameters dependency; length [ADfn::dyp_len].
    pub dyp : Vec<bool> ,
    //
    // var
    /// Variable dependency; length [ADfn::var_len].
    pub var : Vec<bool> ,
}
//
// Old2New
/// Mapping from old (ADfn) indices to new (tape) indices
/// for constants, dynamics and variables.
///
/// If an old constant, (dynamic), {variable} index does not get
/// used in the new tape, the new index value is
/// cop_len, (dyp_len), {var_len} .
/// These are invalid values because the new tape does not have more
/// constants, dynamic parameters, or variables.
///
pub(crate) struct Old2New {
    // cop
    /// Constant parameters; length [ADfn::cop_len].
    pub cop : Vec<IndexT> ,
    //
    // dyp
    /// Dynamic parameters; length [ADfn::dyp_len].
    pub dyp : Vec<IndexT> ,
    //
    // var
    /// Variables; length [ADfn::var_len].
    pub var : Vec<IndexT> ,
}
// --------------------------------------------------------------------------
// renumber_op_seq
/// Renumber an operation sequence using the the first equivalent operator map.
///
/// * equal_type *
/// Type of operator that first_equal refers to.
///
/// * first_equal :
/// If first_equal\[op_index\] is not equal to op_index,
/// depend\[op_index\] is true and the operator with index op_index
/// is equivalent to the operator with index first_equal\[op_index\].
/// In addition, this is the first operator that is known to be equivalent.
///
/// * depend :
/// This identifies which operators, in the operation sequence,
/// are necessary to compute the results
/// for the function this operation sequence appears in.
///
/// * op_seq :
/// This is the operator sequence that we are renumbering.
/// Only the field op_seq.arg_all is modified.
///
///
pub(crate) fn renumber_op_seq(
    equal_type  : ADType           ,
    first_equal : &Vec<IndexT>     ,
    depend      : &Vec<bool>       ,
    op_seq      : &mut OpSequence  ,
) {
    //
    // n_dep
    let n_dep = op_seq.n_dep;
    //
    // n_dom
    let n_dom        = op_seq.n_dom;
    let n_dom_indext = n_dom as IndexT;
    //
    // new_arg
    let mut new_arg : Vec<IndexT> = Vec::new();
    //
    // op_seq.arg_all
    for op_index in 0 .. n_dep {
        //
        // both_index
        let both_index = op_index + n_dom;
        if depend[both_index] {
            //
            // new_arg
            new_arg.clear();
            //
            let start      = op_seq.arg_start[op_index] as usize;
            let end        = op_seq.arg_start[op_index + 1] as usize;
            let arg        = &op_seq.arg_all[start .. end];
            let arg_type   = &op_seq.arg_type_all[start .. end];
            for i_arg in 0 .. arg.len() {
                if n_dom_indext <= arg[i_arg] {
                    if arg_type[i_arg] == equal_type {
                        let both_index  = arg[i_arg] as usize;
                        let dep_index   = both_index - n_dom;
                        new_arg.push( first_equal[dep_index] + n_dom_indext );
                    } else {
                        new_arg.push( arg[i_arg] );
                    }
                } else {
                    new_arg.push( arg[i_arg] );
                }
            }

            let arg  = &mut op_seq.arg_all[start .. end];
            for i_arg in 0 .. arg.len() {
                arg[i_arg] = new_arg[i_arg];
            }
        }
    }
}
// --------------------------------------------------------------------------
// ADfn::optimize
/// Reduce the number of operations in an ADfn object.
///
/// # Example
/// ```
/// use rustad::start_recording_var;
/// use rustad::stop_recording;
/// type V      = rustad::AzFloat<f32>;
/// let  trace  = false;
/// let  x      = vec![ V::from(2.0), V::from(3.0) ];
/// let ax      = start_recording_var(x.clone());
/// let _atimes = &ax[0] * &ax[0]; // second dependent variable (not used)
/// let asum    = &ax[0] + &ax[0]; // first dependent variable (used)
/// let ay      = vec![ asum ];
/// let mut f   = stop_recording(ay);
/// //
/// // f.var_dep_len
/// // x[0] + x[0] and x[0] * x[0] are in original version of f.
/// assert_eq!( f.var_dep_len(), 2 );
/// //
/// f.optimize(trace);
/// //
/// // f.var_dep_len
/// // only x[0] + x[0] is in optimized version of f.
/// assert_eq!( f.var_dep_len(), 1 );
/// //
/// // check
/// let (y, _v) = f.forward_zero_value(x.clone(), trace);
/// assert_eq!( y[0], &x[0] + &x[0] );
/// ```
impl<V> ADfn<V>
where
    V : Clone + From<f32> + Eq + std::fmt::Display + std::hash::Hash +
        AtomInfoVecPublic + GlobalOpInfoVecPublic,
{   //
    // optimize
    pub fn optimize(&mut self, trace : bool)
    {   //
        // depend
        let mut depend = self.reverse_depend(trace);
        //
        // self, depend
        self.compress_cop(&mut depend, trace);
        //
        // self, depend
        self.compress_dyp(&mut depend, trace);
        //
        // tape, old2new
        let (mut tape, old2new) = self.dead_code(&depend, trace);
        //
        // checks
        assert_eq!( tape.dyp.arg_start.len()  , tape.dyp.id_all.len() );
        assert_eq!( tape.var.arg_start.len()  , tape.var.id_all.len() );
        //
        assert_eq!( tape.dyp.arg_all.len()  , tape.dyp.arg_type_all.len() );
        assert_eq!( tape.var.arg_all.len()  , tape.var.arg_type_all.len() );
        //
        assert_eq!( tape.dyp.n_dep , tape.dyp.id_all.len());
        assert_eq!( tape.var.n_dep , tape.var.id_all.len());
        //
        // tape.*.var.arg_start
        // End marker for arguments to the last operation
        tape.var.arg_start.push( tape.var.arg_all.len() as IndexT );
        tape.dyp.arg_start.push( tape.dyp.arg_all.len() as IndexT );
        //
        // self, tape
        std::mem::swap(&mut self.cop,  &mut tape.cop);
        std::mem::swap(&mut self.dyp,  &mut tape.dyp);
        std::mem::swap(&mut self.var,  &mut tape.var);
        //
        // self: rng_ad_type, rng_index, cop
        // TODO: figure out how to do this without any cloning of values.
        let n_rng = self.rng_index.len();
        for i_rng in 0 .. n_rng {
            let old_index = self.rng_index[i_rng] as usize;
            match self.rng_ad_type[i_rng] {
                ADType::ConstantP => {
                    let new_index = old2new.cop[old_index];
                    assert!( (new_index as usize) < old2new.cop.len() );
                    self.rng_index[i_rng] = new_index;
                },
                ADType::DynamicP => {
                    let new_index = old2new.dyp[old_index];
                    assert!( (new_index as usize) < old2new.dyp.len() );
                    self.rng_index[i_rng] = new_index;
                },
                ADType::Variable => {
                    let new_index = old2new.var[old_index];
                    assert!( (new_index as usize) < old2new.var.len() );
                    self.rng_index[i_rng] = new_index;
                },
                _ => { panic!("optimize: rng_ad_type error"); },
            }
        }
    }
}
