// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Define tape objects
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use std::cell::RefCell;
use std::thread::LocalKey;
use std::sync::Mutex;
//
use crate::numvec::NumVec;
//
/// The type used for vectors of indices in the tape and function objects
pub(crate) type Tindex = u32;
// ---------------------------------------------------------------------------
// Tape
///
/// `Tape` < *V* > is the type used to record a `AD` < *V* > function evaluation
///
/// * V : is the value type used ruing the recording.
///
pub struct Tape<V> {
    //
    // recording
    /// if false (true) a recording is currently in progress on this tape.
    /// If recording is false, all of the tape's index values are zero
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
    /// is the corresponding operator id.
    pub id_all         : Vec<u8>,
    //
    // op2arg
    /// For each op_index in the operation sequence, op2arg\[op_index\]
    /// is the index in arg_all of the first argument for the operator.
    pub op2arg         : Vec<Tindex>,
    //
    // arg_all
    /// For each op_index in the operation sequence,
    /// the arguments for the operator are a slice of arg_all
    /// starting at op2arg\[index\] .
    pub arg_all        : Vec<Tindex>,
    //
    // con_all
    /// is a vector containing the constant values used by the
    /// operation sequence
    pub con_all        : Vec<V>,
    //
    // flag_all
    /// is a vector containing boolean flags that are part of some
    /// operator definitions.
    pub flag_all       : Vec<bool>,
}
// ---------------------------------------------------------------------------
// GTape::new
//
impl<V> Tape<V> {
    //
    // Tape::new
    /// Sets recording to false, tape_id, n_domain, n_var to zero,
    /// and the vectors to empty.
    pub fn new() -> Self {
        Self {
            recording     : false,
            tape_id       : 0,
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
/// The tape_id values that have been used are 1 .. NEXT_TAPE_ID.
/// (The tape_id 0 is not used for a recording.)
/// 2DO: Change this to be pub(crate) once it is used.
pub static NEXT_TAPE_ID : Mutex<usize> = Mutex::new(1);
// ---------------------------------------------------------------------------
//
// sealed::ThisThreadTape
pub (crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package.
    //
    use super::Tape;
    use std::cell::RefCell;
    use std::thread::LocalKey;
    //
    /// ```text
    ///     local_key : &LocalKey< RefCell< Tape<V> > > = ThisThreadTape::get()
    //      local_key.with_borrow_mut( |tape| { ... } )
    /// ```
    /// Sets `tape` to a reference to the tape for recording `AD` < *V* >
    /// operations.
    ///
    pub trait ThisThreadTape
        where
        Self : Sized + 'static ,
    {
        fn get() -> &'static LocalKey< RefCell< Tape<Self> > >;
    }
}
//
/// Get reference to the tape for this thread.
///
/// * Value : is the type used for values calculations.
///
/// This macro must be executed once for any type *Value*  where
/// `AD` < *Value* > is used. It is execute one by this package for the
/// following types: f32, f64, `NumVec<f32>`, `NumVec<f64>`.
macro_rules! impl_this_thread_tape{ ($Value:ty) => {
    #[doc = concat!(
        "This threads tape for recording ",
        "`AD<" , stringify!($Value), ">` operations"
    ) ]
    impl sealed::ThisThreadTape for $Value {
        fn get() -> &'static LocalKey< RefCell< Tape<$Value> > > {
            thread_local! {
                pub(crate) static THIS_THREAD_TAPE :
                    RefCell< Tape<$Value> > = RefCell::new( Tape::new() );
            }
            &THIS_THREAD_TAPE
        }
    }
} }
impl_this_thread_tape!(f32);
impl_this_thread_tape!( NumVec<f32> );
impl_this_thread_tape!(f64);
impl_this_thread_tape!( NumVec<f64> );
