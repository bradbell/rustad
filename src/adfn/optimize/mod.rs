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
// -----------------------------------------------------------------------
// mod
mod reverse_depend;
mod dead_code;
//
#[cfg(test)]
mod tests;
// -----------------------------------------------------------------------
// Depend
/// Which constants, dynamic parameters, and variables the
/// range for an [ADfn] depends on.
///
pub(crate) struct Depend {
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
// ADfn::optimize
impl<V> ADfn<V>
where
    V : Clone + AtomInfoVecPublic + GlobalOpInfoVecPublic,
{   //
    // optimize
    pub fn optimize(&mut self, trace : bool)
    {   //
        let depend   = self.reverse_depend(trace);
        let mut tape = self.dead_code(&depend);
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
    }
}
