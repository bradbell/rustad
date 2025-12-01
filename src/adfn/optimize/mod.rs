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
//
// -----------------------------------------------------------------------
// mod
mod reverse_depend;
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
// Renumber
/// Mapping from old (ADfn) indices to new (tape) indices
/// for constants, dynamics and variables.
///
/// If an old constant, (dynamic), {variable} index does not get
/// used in the new tape, the new index value is
/// cop_len, (dyp_len), {var_len} .
/// These are invalid values because the new tape does not have more
/// constants, dynamic parameters, or variables.
///
pub(crate) struct Renumber {
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
// ADfn::optimize
impl<V> ADfn<V>
where
    V : Clone + AtomInfoVecPublic + GlobalOpInfoVecPublic,
{   //
    // optimize
    pub fn optimize(&mut self, trace : bool)
    {   //
        let depend               = self.reverse_depend(trace);
        let (mut tape, renumber) = self.dead_code(&depend, trace);
        //
        // checks
        assert_eq!( tape.dyp.arg_seq.len()  , tape.dyp.id_seq.len() );
        assert_eq!( tape.var.arg_seq.len()  , tape.var.id_seq.len() );
        //
        assert_eq!( tape.dyp.arg_all.len()  , tape.dyp.arg_type_all.len() );
        assert_eq!( tape.var.arg_all.len()  , tape.var.arg_type_all.len() );
        //
        assert_eq!( tape.dyp.n_dep , tape.dyp.id_seq.len());
        assert_eq!( tape.var.n_dep , tape.var.id_seq.len());
        //
        // tape.*.var.arg_seq
        // End marker for arguments to the last operation
        tape.var.arg_seq.push( tape.var.arg_all.len() as IndexT );
        tape.dyp.arg_seq.push( tape.dyp.arg_all.len() as IndexT );
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
                    let value = tape.cop[old_index].clone();
                    self.rng_index[i_rng] = self.cop.len() as IndexT;
                    self.cop.push( value );
                },
                ADType::DynamicP => {
                    let new_index = renumber.dyp[old_index];
                    assert!( (new_index as usize) < renumber.dyp.len() );
                    self.rng_index[i_rng] = new_index;
                },
                ADType::Variable => {
                    let new_index = renumber.var[old_index];
                    assert!( (new_index as usize) < renumber.var.len() );
                    self.rng_index[i_rng] = new_index;
                },
                _ => { panic!("optimize: rng_ad_type error"); },
            }
        }
    }
}
