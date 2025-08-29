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
use crate::numvec::AD;
use crate::numvec::ADFn;
//
#[cfg(doc)]
use crate::numvec::ad::doc_generic_v;
//
/// The type used for vectors of indices in the tape and function objects
pub(crate) type Tindex = u32;
// ---------------------------------------------------------------------------
// Tape
///
/// `Tape` < *V* > is the type were to an `AD` < *V* >
/// operation sequence is recorded.
///
/// * V : see [doc_generic_v]
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
pub(crate) static NEXT_TAPE_ID : Mutex<usize> = Mutex::new(1);
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
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD` < *V* > is used. The rustad package automatically executes it
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
macro_rules! impl_this_thread_tape{ ($V:ty) => {
    #[doc = concat!(
        "This threads tape for recording ",
        "`AD<" , stringify!($V), ">` operations"
    ) ]
    impl sealed::ThisThreadTape for $V {
        fn get() -> &'static LocalKey< RefCell< Tape<$V> > > {
            thread_local! {
                pub(crate) static THIS_THREAD_TAPE :
                    RefCell< Tape<$V> > = RefCell::new( Tape::new() );
            }
            &THIS_THREAD_TAPE
        }
    }
} }
impl_this_thread_tape!(f32);
impl_this_thread_tape!( NumVec<f32> );
impl_this_thread_tape!(f64);
impl_this_thread_tape!( NumVec<f64> );
// ----------------------------------------------------------------------------
// start_recording
//
/// This starts recording a new ''AD`` < *V* > operation sequence on
/// this thead's tape.
///
/// * Syntax :
/// ```text
///     adomain = start_recording(domain)
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Recording :
/// There must not currently be a recording in process on the current thread
/// when start_recording is called.
/// The recording is stopped when [stop_recording] is called.
///
/// * domain :
/// This vector contains the value of the domain variables used during
/// the recording.
///
/// * adomain :
/// The return is a vector of variables
/// with the same length and values as domain.
/// Dependencies with respect to these variables will be recorded on
/// the tape for this thread.
///
/// * Example : see [stop_recording]
///
pub fn start_recording<V>(domain : Vec<V> ) -> Vec< AD<V> >
where
    V : Clone + Sized + 'static + sealed::ThisThreadTape ,
{
    //
    // tape_id
    let tape_id : usize;
    {   let mut next_tape_id = NEXT_TAPE_ID.lock().unwrap();
        //
        // The rest of this block has a lock, so it is fast and can't fail.
        tape_id        = *next_tape_id;
        *next_tape_id += 1;
    }
    let local_key : &LocalKey< RefCell< Tape<V> > > =
        sealed::ThisThreadTape::get();
    local_key.with_borrow_mut( |tape| {
        assert_ne!( tape_id, 0);
        assert!( ! tape.recording ,
            "start_recording: This thread's tape is already recording"
        );
        //
        assert_eq!( tape.id_all.len(),   0 );
        assert_eq!( tape.op2arg.len(),   0 );
        assert_eq!( tape.arg_all.len(),  0 );
        assert_eq!( tape.con_all.len(),  0 );
        assert_eq!( tape.flag_all.len(), 0 );
        //
        tape.tape_id     = tape_id;
        tape.recording   = true;
        tape.n_domain    = domain.len();
        tape.n_var       = domain.len();
    } );
    //
    // adomain
    let adomain = domain.into_iter().enumerate().map(
        | (index, value) | AD::new(tape_id, index, value)
    ).collect();
    //
    adomain
}
// ----------------------------------------------------------------------------
// stop_recording
//
/// Stops a recordng and moves it to an ADFn object.
///
/// * Syntax :
/// ```test
///     ad_fn = stop_recording(arange)
/// ```
///
/// * V : see [doc_generic_v]
///
/// * Recording :
/// There must currently be a recording in process on the current thread
/// when stop_recording is called.
///
/// * arange :
/// This is a `Vec< AD<` *V* `> >` vector that specifies
/// the range space variables.
///
/// * ad_fn :
/// The return value is an `ADFn` < *V* > containing the operation sequence
/// that computed arange as a function of the adomain returned by
/// [start_recording] .
/// It can compute the values for the function and its derivative.
///
/// * Assumptions :
/// The following assumptions are checked for the tape for this thread:
/// ```text
///     1. tape.arg_all.len()                <= [Tindex]::Max
///     2. tape.tape_id                      <= Tindex::Max
///     3. tape.con_all.len() + arange.len() <= Tindex::Max
/// ```
/// # Example
/// ```
/// use rustad::numvec::tape::start_recording;
/// use rustad::numvec::tape::stop_recording;
/// let domain  : Vec<f32>  = vec![ 1.0, 2.0 ];
/// let adomain             = start_recording( domain );
/// let sum                 = &adomain[0] + &adomain[1];
/// let diff                = &adomain[0] - &adomain[1];
/// let times               = &adomain[0] * &adomain[1];
/// let arange              = vec![ sum, diff, times ];
/// let ad_fn               = stop_recording( arange );
/// assert_eq!( ad_fn.domain_len(), 2);
/// assert_eq!( ad_fn.range_len(), 3);
/// ```
pub fn stop_recording<V>( arange : Vec< AD<V> > ) -> ADFn<V>
where
    Tindex : TryFrom<usize> ,
    V : Clone + Sized + 'static + sealed::ThisThreadTape ,
{
    let mut ad_fn : ADFn<V> = ADFn::new();
    let local_key : &LocalKey< RefCell< Tape<V> > > =
        sealed::ThisThreadTape::get();
    let tape_id : usize = local_key.with_borrow_mut( |tape| {
        //
        // tape.recording
        assert!( tape.recording ,
            "stop_recording: This thread's tape is not recording"
        );
        tape.recording = false;
        //
        // check documented assumptions
        match Tindex::try_from( tape.arg_all.len() ) {
            Err(_) => panic!( "tape.arg_all.len() > Tindex::MAX" ),
            Ok(_)  => (),
        }
        match Tindex::try_from( tape.tape_id ) {
            Err(_) => panic!( "tape.tape_id > Tindex::MAX" ),
            Ok(_)  => (),
        }
        let con_all_len = tape.con_all.len() + arange.len();
        match Tindex::try_from( con_all_len ) {
            Err(_) => panic!(
                "tape.con_all.len() + arange.len() > Tindex::MAX"
            ),
            Ok(_)  => (),
        }
        //
        // more checks
        assert_eq!( tape.op2arg.len() , tape.id_all.len() );
        assert_eq!( tape.n_var , tape.n_domain + tape.id_all.len() );
        //
        // tape.op2arg
        // end marker for arguments to the last operation
        tape.op2arg.push( tape.arg_all.len() as Tindex );
        //
        // swap fields in ad_fn and tape
        std::mem::swap( &mut ad_fn.n_domain,      &mut tape.n_domain );
        std::mem::swap( &mut ad_fn.n_var,         &mut tape.n_var );
        std::mem::swap( &mut ad_fn.id_all,        &mut tape.id_all );
        std::mem::swap( &mut ad_fn.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut ad_fn.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut ad_fn.con_all,       &mut tape.con_all );
        std::mem::swap( &mut ad_fn.flag_all,      &mut tape.flag_all );
        //
        // tape_id
        tape.tape_id
    } );
    //
    // range_is_var, range2tape_index, con_all
    // 2DO: figure out how to do this without any cloning of values.
    for i in 0 .. arange.len() {
        if arange[i].tape_id == tape_id {
            ad_fn.range_is_var.push( true );
            ad_fn.range2tape_index.push( arange[i].var_index as Tindex );
        } else {
            ad_fn.range_is_var.push( false );
            ad_fn.range2tape_index.push( ad_fn.con_all.len() as Tindex  );
            ad_fn.con_all.push( arange[i].value.clone() );
        }
    }
    ad_fn
}
