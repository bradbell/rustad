// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Define tape objects: [parent module](super)
//
use crate::Index;
use crate::Float;
use std::sync::Mutex;
//
#[cfg(doc)]
use crate::operator;
// ---------------------------------------------------------------------------
// GTape
///
/// GTape<F, U> is the type used to record a GAD<F, U> function evaluation
pub(crate) struct GTape<F, U> {
    //
    // recording
    /// if false (true) a recording is currently in progress on this tape.
    /// If recording is false, all of the Tape's Index values are zero
    /// and all of its vectors are empty.
    pub recording      : bool,
    //
    // tape_id
    /// a different tape_id is chosen for each recording.
    pub tape_id        : usize,
    //
    // n_domain
    /// is the dimension of the domain space for the operation being recorded.
    pub n_domain       : usize,
    //
    // n_var
    /// is the number of variables currently in the recording.
    pub n_var          : usize,
    //
    // id_all
    /// For each index in the operation sequence, id_all\[index\]
    /// is the corresponding [operator::id] .
    pub id_all         : Vec<u8>,
    //
    // op2arg
    /// For each op_index in the operation sequence, op2arg\[op_index\]
    /// is the index in arg_all of the first argument for the operator.
    pub op2arg         : Vec<U>,
    //
    // arg_all
    /// For each op_index in the operation sequence,
    /// the arguments for the operator are a slice of arg_all
    /// starting at op2arg\[index\] .
    pub arg_all        : Vec<U>,
    //
    // con_all
    /// is a vector containing the constant values used by the
    /// operation sequence
    pub con_all        : Vec<F>,
    //
    // flag_all
    /// is a vector containing boolean flags that are part of some
    /// operator definitions.
    pub flag_all       : Vec<bool>,
}
//
// Tape
/// Tape is the GTape that corresponds to AD
pub type Tape = GTape<Float, Index>;
// ---------------------------------------------------------------------------
// GTape::new
//
impl<F, U> GTape<F, U> {
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
            id_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
            flag_all      : Vec::new() ,
        }
    }
}
// ---------------------------------------------------------------------------
// NEXT_TAPE_ID
/// The tape_id values that have been used are 1 .. NEXT_TAPE_ID
/// (0 is not used for a recording).
pub(crate) static NEXT_TAPE_ID : Mutex<usize> = Mutex::new(1);
// ---------------------------------------------------------------------------
// this_thread_tape!
//
/// Create the tape for the current thread.
///
/// * f1 : is the floating point type for calculating values.
/// we use F1 to denote the upper case version of f1.
/// * u2 : is the unsigned integer type for indices in the tape.
/// we use U2 to denote the upper case version of u2.
///
/// THIS_THREAD_TAPE_F1_U2 :
/// is the name of the tape created by this macro call.
///
macro_rules! this_thread_tape { ($f1:ident, $u2:ident) => { paste::paste! {
    thread_local! {
        #[doc = concat!(
            "The thread local tape where ",
            "GAD<", stringify!( $f1 ), ",", stringify!( $u2 ), "> " ,
            "operations are stored"
        ) ]
        pub(crate) static
        [< THIS_THREAD_TAPE_ $f1:upper _ $u2:upper >] :
            std::cell::RefCell< GTape<$f1, $u2> > =
                std::cell::RefCell::new( GTape::new() );
    }
} } }
this_thread_tape!(f64, u32);
