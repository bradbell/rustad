// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Define tape objects
//
use crate::Index;
use crate::Float;
use std::sync::Mutex;
//
#[cfg(doc)]
use crate::operators;
//
// Tape
/// is the type used to represent one tape; i.e., o
/// one operation sequence recording
pub(crate) struct Tape {
    //
    // recording
    /// if false (true) a recording is currently in progress on this tape.
    /// If recording is false, all of the Tape's Index values are zero
    /// and all of its vectors are empty.
    pub recording      : bool,
    //
    // tape_id
    /// a different tape_id is chosen for each recording.
    pub tape_id        : Index,
    //
    // n_domain
    /// is the dimension of the domain space for the operation being recorded.
    pub n_domain       : Index,
    //
    // n_var
    /// is the number of variables currently in the recording.
    pub n_var          : Index,
    //
    // op_all
    /// For each index in the operation sequence, op_all\[index\]
    /// is the corresponding [operators::id] .
    pub op_all         : Vec<Index>,
    //
    // op2arg
    /// For each index in the operation sequence, op2arg\[index\]
    /// is the index in arg_all of the first argument for the operator.
    pub op2arg         : Vec<Index>,
    //
    // arg_all
    /// For each index in the operation sequence,
    /// the arguments for the operator are a slice of arg_all
    /// starting at op2arg\[index\] .
    pub arg_all        : Vec<Index>,
    //
    // con_all
    /// is a vector containing the constant values used by the
    /// operation sequence
    pub con_all        : Vec<Float>,
}
impl Tape {
    //
    // Tape::new
    /// Sets recording to false, all the Index values to zero,
    /// and the vectors to empty.
    pub fn new() -> Self {
        Self {
            tape_id       : 0,
            recording     : false,
            n_domain      : 0,
            n_var         : 0,
            op_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
        }
    }
}
//
// NEXT_TAPE_ID
/// The tape_id values that have been used are 1 .. NEXT_TAPE_ID
/// (0 is not used for a recording).
pub(crate) static NEXT_TAPE_ID : Mutex<Index> = Mutex::new(1);
//
thread_local! {
    //
    // THIS_THREAD_TAPE
    /// is thread local storage used to record one operation sequence
    pub(crate) static THIS_THREAD_TAPE: std::cell::RefCell<Tape> =
        std::cell::RefCell::new( Tape::new() );
}
