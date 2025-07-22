// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Define tape objects
//! : [parent module](super)
//
use std::cell::RefCell;
use std::thread::LocalKey;
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
///
/// * F : is the floating point type use for value calculations.
///
/// * U :
/// is the unsigned integer type used indices in the tape.
/// It must be able to represent the maximum:
/// tape id, operator index, constant index, operator argument index.
pub struct GTape<F, U> {
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
pub (crate) static NEXT_TAPE_ID : Mutex<usize> = Mutex::new(1);
// ---------------------------------------------------------------------------
//
// ThisThreadTape
/// ```text
///     < F as ThisThreadTape >::get()
/// ```
/// returns a reference to the tape for recording GAD<F,U> using this thread.
///
pub trait ThisThreadTape<U>
where
    Self : Sized + 'static ,
    U    : Sized + 'static ,
{
    fn get() -> &'static LocalKey< RefCell< GTape<Self, U> > >;
}
//
/// Get reference to the tape for this thread.
///
/// * f1 : is the floating point type used for values calculations.
/// * u2 : is the unsigned integer type used for tape indices.
///
macro_rules! impl_this_thread_tape{ ($f1:ident, $u2:ident) => {
    #[doc = concat!(
        "This threads tape for recording ",
        "GAD<" , stringify!($f1), ", ", stringify!($u2), "> operations"
    ) ]
    impl ThisThreadTape<$u2> for $f1 {
        fn get() -> &'static LocalKey< RefCell< GTape<$f1, $u2> > > {
            thread_local! {
                pub(crate) static THIS_THREAD_TAPE :
                    RefCell< GTape<$f1, $u2> > = RefCell::new( GTape::new() );

            }
            &THIS_THREAD_TAPE
        }
    }
} }
impl_this_thread_tape!(f32, u32);
impl_this_thread_tape!(f32, u64);
impl_this_thread_tape!(f64, u32);
impl_this_thread_tape!(f64, u64);
