// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! ADFun objects
//
use crate::Index;
use crate::Float;
use crate::OP_INFO_VEC;
use crate::AD;
use crate::THIS_THREAD_TAPE;
//
// ADFun
/// A [domain] call is used to start a recording function.
/// A [range] call is used to stop recording and create an ADFun object
/// that can evaluate the function and its derivatives.
/// The recording is a single assignment representation of the function; i.e.,
/// a variable is only assigned once.
pub struct ADFun {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain       : Index,
    /// The total number of variables in this function.
    //
    // n_var
    /// The total number of variables in the recording.
    pub(crate) n_var          : Index,
    //
    // range
    /// The variable index for each of the range variables in this function.
    /// The dimension of its range spase is reange.len().
    pub(crate) range          : Vec<Index>,
    //
    // op_vec
    /// This maps an operators index in the recording of the function
    /// to its [operator_id](crate::operator_id) .
    pub(crate) op_vec         : Vec<Index>,
    //
    // op2arg
    /// This maps an operators index in the function to its
    /// the index of its first argument in arg_vec.
    pub(crate) op2arg         : Vec<Index>,
    //
    // arg_vec
    /// This contains the arguments for all the opereators in the recording.
    pub(crate) arg_vec        : Vec<Index>,
    //
    // con_vec
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_vec        : Vec<Float>,
}
impl ADFun {
    pub fn new() -> Self {
        Self {
            n_domain      : 0,
            n_var         : 0,
            op_vec        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_vec       : Vec::new() ,
            con_vec       : Vec::new() ,
            range         : Vec::new() ,
        }
    }
    pub fn forward(&self, x : &[Float] ) -> Vec<Float> {
        let op_info_vec = &*OP_INFO_VEC;
        let mut var_vec = vec![ Float::NAN; self.n_var ];
        for j in 0 .. self.n_domain {
            var_vec[j] = x[j];
        }
        for i_op in 0 .. self.op_vec.len() {
            let op    = self.op_vec[i_op];
            let start = self.op2arg[i_op];
            let end   = self.op2arg[i_op + 1];
            let arg   = &self.arg_vec[start .. end];
            let res   = self.n_domain + i_op;
            let fun   = op_info_vec[op].fun;
            fun(&mut var_vec, &self.con_vec, &arg, res );
        }
        let mut y : Vec<Float> = Vec::new();
        for i in 0 .. self.range.len() {
            y.push( var_vec[ self.range[i] ] );
        }
        y
    }
}
//
// domain
pub fn domain( x : &[Float] ) -> Vec<AD> {
    let mut new_tape_id = 0;
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.op_vec.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_vec.len(), 0 );
        assert_eq!( tape.con_vec.len(), 0 );
        tape.tape_id       += 1;
        tape.recording      = true;
        tape.n_domain       = x.len();
        tape.n_var          = x.len();
        //
        new_tape_id         = tape.tape_id;
    } );
    let mut result : Vec<AD> = Vec::new();
    for j in 0 .. x.len() {
        result.push(
             AD { tape_id : new_tape_id, var_index : j, value : x[j] }
        );
    }
    result
}
//
// range
pub fn range( y : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        tape.op2arg.push( tape.arg_vec.len() );
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        std::mem::swap( &mut result.n_domain, &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.op_vec,        &mut tape.op_vec );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_vec,       &mut tape.arg_vec );
        std::mem::swap( &mut result.con_vec,       &mut tape.con_vec );
    } );
    for i in 0 .. y.len() {
        result.range.push( y[i].var_index );
    }
    result
}
