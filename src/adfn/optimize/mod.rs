// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] optimization methods.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::{
    FConst,
    ADfn,
    IndexT,
    GlobalAtomCallbackVecPublic,
    GlobalOpFnsVecPublic,
};
//
use crate::ad::ADType;
use crate::tape::AGraph;
//
// -----------------------------------------------------------------------
// mod
mod op_hash_map;
mod reverse_depend;
mod compress_cop;
mod compress_dyp;
mod compress_var;
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
// renumber_agraph
/// Renumber an acyclic graph using the the first equivalent operator map.
///
/// * equal_type *
///   Type of operator that first_equal refers to.
///
/// * first_equal :
///   If first_equal\[op_index\] is not equal to op_index,
///   depend\[op_index\] is true and the operator with index op_index
///   is equivalent to the operator with index first_equal\[op_index\].
///   In addition, this is the first operator that is known to be equivalent.
///
/// * depend :
///   This identifies which operators, in the acyclic graph,
///   are necessary to compute the results
///   for the function this acyclic graph appears in.
///
/// * agraph :
///   This is the operator sequence that we are renumbering.
///   Only the field agraph.arg_all is modified.
///
///
pub(crate) fn renumber_agraph(
    equal_type  : ADType           ,
    first_equal : &[IndexT]        ,
    depend      : &[bool]          ,
    agraph      : &mut AGraph  ,
) {
    //
    // n_dep
    let n_dep = agraph.n_dep;
    //
    // n_dom
    let n_dom        = agraph.n_dom;
    let n_dom_indext = n_dom as IndexT;
    //
    // agraph.arg_all
    for op_index in 0 .. n_dep {
        //
        if depend[op_index + n_dom] {
            let start      = agraph.arg_start[op_index] as usize;
            let end        = agraph.arg_start[op_index + 1] as usize;
            let arg        = &mut agraph.arg_all[start .. end];
            let arg_type   = &agraph.arg_type_all[start .. end];
            for i_arg in 0 .. arg.len() {
                if n_dom_indext <= arg[i_arg]
                    && arg_type[i_arg] == equal_type {
                    let both_index = arg[i_arg] as usize;
                    let dep_index  = both_index - n_dom;
                    arg[i_arg]     = first_equal[dep_index] + n_dom_indext;
                }
            }
        }
    }
}
// --------------------------------------------------------------------------
// ADfn::optimize
/// Reduce the number of operations in an ADfn object.
///
/// * Syntax :
/// ```text
///     f.optimize(arg_vec)
/// ```
///
/// * V : see [doc_generic_v](crate::doc_generic_v)
/// * f : is an [ADfn] object
///
/// * arg_vec :
///   is an [arg_vec](crate::doc_arg_vec) with the following possible keys:
///
///   * trace
///     The corresponding value must be true of false (default is false).
///     If it is true, a trace of forward_der is printed on stdout.
///
/// # Example
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// type V       = rustad::AzFloat<f32>;
/// let  x      = vec![ V::from(2.0), V::from(3.0) ];
/// let (_, ax) = start_recording(None, x.clone());
/// let _atimes = &ax[0] * &ax[0]; // second dependent variable (not used)
/// let asum    = &ax[0] + &ax[0]; // first dependent variable (used)
/// let ay      = vec![ asum ];
/// let mut f   = stop_recording(ay);
/// //
/// // f.var_dep_len
/// // x[0] + x[0] and x[0] * x[0] are in original version of f.
/// assert_eq!( f.var_dep_len(), 2 );
/// //
/// let  arg_vec : Vec<[&str; 2]> = Vec::new();
/// f.optimize(&arg_vec);
/// //
/// // f.var_dep_len
/// // only x[0] + x[0] is in optimized version of f.
/// assert_eq!( f.var_dep_len(), 1 );
/// //
/// // check
/// let trace   = false;
/// let (y, _v) = f.forward_var_value(None, x.clone(), &arg_vec);
/// assert_eq!( y[0], &x[0] + &x[0] );
/// ```
impl<V> ADfn<V>
where
    V : Clone + FConst + Eq + std::fmt::Display + std::hash::Hash,
    V : GlobalAtomCallbackVecPublic + GlobalOpFnsVecPublic,
{   //
    // optimize
    pub fn optimize(&mut self, arg_vec : &Vec<[&str; 2]> )
    {   //
        // trace
        let mut trace = false;
        for arg in arg_vec {
            match arg[0] {
                "trace" => {
                    match arg[1] {
                        "true"  => { trace = true; },
                        "false" => { trace = false; },
                        _ => { panic!(
                        "forward_der arg_vec: invalid value for trace"
                        ); }
                    }
                },
                _ => panic!("forward_der arg_vec: invalid key"),
            }
        }
        //
        // depend
        let mut depend = self.reverse_depend(trace);
        //
        // self, depend
        self.compress_cop(&mut depend, trace);
        //
        // self, depend
        self.compress_dyp(&mut depend, trace);
        //
        // self, depend
        self.compress_var(&mut depend, trace);
        //
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
