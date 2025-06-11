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
/// A [domain] call is used to start a recording an operatioin sequence.
/// A [range] call is used to stop recording and create an ADFun object
/// that can evaluate the function and its derivatives.
/// The operation sequence is a single assignment representation of
/// the function; i.e., a variable is only assigned once.
pub struct ADFun {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain       : Index,
    /// The total number of variables in this function.
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub(crate) n_var          : Index,
    //
    // range
    /// The variable index for each of the range variables in this function.
    /// The dimension of its range spase is reange.len().
    pub(crate) range          : Vec<Index>,
    //
    // op_all
    /// This maps an operators index in the operation sequence
    /// to its [operator_id](crate::operator_id) .
    pub(crate) op_all         : Vec<Index>,
    //
    // op2arg
    /// This maps an operators index in the operation sequence to its
    /// the index of its first argument in arg_all.
    pub(crate) op2arg         : Vec<Index>,
    //
    // arg_all
    /// This contains the arguments for all the opereators in the
    /// operatioon sequence.
    pub(crate) arg_all        : Vec<Index>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_all        : Vec<Float>,
}
impl ADFun {
    //
    // new
    /// This creates an empty operation sequence; i.e,
    /// its domain and range vectors have length zero.
    /// # Example
    /// ```
    /// let f = rustad::function::ADFun::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain      : 0,
            n_var         : 0,
            op_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
            range         : Vec::new() ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> Index { self.n_domain }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> Index { self.range.len() }
    //
    // forward
    /// zero order forward mode; i.e.,  function values
    pub fn forward(&self, x : &[Float] ) -> Vec<Float> {
        let op_info_vec = &*OP_INFO_VEC;
        let mut var_vec = vec![ Float::NAN; self.n_var ];
        for j in 0 .. self.n_domain {
            var_vec[j] = x[j];
        }
        for i_op in 0 .. self.op_all.len() {
            let op    = self.op_all[i_op];
            let start = self.op2arg[i_op];
            let end   = self.op2arg[i_op + 1];
            let arg   = &self.arg_all[start .. end];
            let res   = self.n_domain + i_op;
            let fun   = op_info_vec[op].fun;
            fun(&mut var_vec, &self.con_all, &arg, res );
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
/// Calling this function starts a new recording.
/// There must not currently be a recording in process using
/// [THIS_THREAD_TAPE] .
///
/// # x
/// This vector determines the number of domain (independent) variables
/// and their value during the recording.
///
/// # domain
/// The return value is the vector of domain space variables.
/// It has the same length and values as x.
pub fn domain( x : &[Float] ) -> Vec<AD> {
    let mut new_tape_id = 0;
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.op_all.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_all.len(), 0 );
        assert_eq!( tape.con_all.len(), 0 );
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
/// Calling thjis function stops a recordng.
/// There must currently be a recording in process using
/// [THIS_THREAD_TAPE] .
///
/// # ay
/// This is the vector of range space variables.
///
/// # range
/// The returreturn value is an ADFun that contains the sequence of operations
/// that determine the range space variables as a function of the
/// domain space variables.
pub fn range( ay : &[AD] ) -> ADFun {
    let mut result = ADFun::new();
    THIS_THREAD_TAPE.with_borrow_mut( |tape| {
        tape.op2arg.push( tape.arg_all.len() );
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        std::mem::swap( &mut result.n_domain, &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.op_all,        &mut tape.op_all );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut result.con_all,       &mut tape.con_all );
    } );
    for i in 0 .. ay.len() {
        result.range.push( ay[i].var_index );
    }
    result
}
