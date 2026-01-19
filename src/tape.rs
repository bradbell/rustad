// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! This pub module defines tape objects and functions
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use std::cell::RefCell;
use std::thread::LocalKey;
use std::sync::Mutex;
//
use crate::ad::ADType;
use crate::{
    AD,
    ADfn,
    FloatCore,
};
//
#[cfg(doc)]
use crate::doc_generic_v;
//
/// The type is used, instead of usize, to save space in vectors of indices.
pub type IndexT = u32;
// ---------------------------------------------------------------------------
// OpSequence
/// An operation sequence is a single assignment representation of
/// a function; i.e., each dependent value is only assigned once.
pub(crate) struct OpSequence {
    //
    // n_dom
    /// is the number of independent values in the operation sequence.
    pub(crate) n_dom : usize,
    //
    // n_dep
    /// is the number of dependent values currently in the operation sequence.
    pub(crate) n_dep : usize,
    //
    // id_all
    /// For each index in the operation sequence, id_all\[op_index\]
    /// is the corresponding operator [id](crate::op::id) .
    pub(crate) id_all : Vec<u8>,
    //
    // arg_start
    /// For each index in the operation sequence, arg_start\[op_index\]
    /// is the index in arg_all of the first argument for the operator.
    pub(crate) arg_start : Vec<IndexT>,
    //
    // arg_all
    /// For each index in the operation sequence,
    /// the arguments for the corresponding operator are a slice of arg_all
    /// starting at arg_start\[index\] and ending with arg_start\[index + 1\] .
    pub(crate) arg_all : Vec<IndexT>,
    //
    // arg_type_all
    /// For each *index* in arg_all, if the value arg_type\[ *index* \] is
    /// ADType::ConstantP ( ADType::DynamicP ) { ADtype::Variable } ,
    /// the value arg_all\[ *index * \] is a constant parameter
    /// ( dynamic parameter ) { variable }.
    pub(crate) arg_type_all : Vec<ADType>,
    //
    // flag_all
    /// is a vector containing all the flags for all the operators.
    /// If an operator has flags, one of its arguments in
    /// arg_all is the index in flag_all of its first flag.
    pub(crate) flag_all : Vec<bool>,
}
// VarTape::new
impl OpSequence {
    //
    // OpSequence::new
    /// Sets n_dom, n_dep to zero and all the vectors to empty.
    pub fn new() -> Self {
        Self {
            n_dom         : 0,
            n_dep         : 0,
            id_all        : Vec::new(),
            arg_start     : Vec::new(),
            arg_all       : Vec::new() ,
            arg_type_all  : Vec::new() ,
            flag_all      : Vec::new() ,
        }
    }
}
// ---------------------------------------------------------------------------
// Tape
///
/// `Tape` < *V* > is the type were to an `AD<V>`
/// operation sequence is recorded.
///
/// * V : see [doc_generic_v]
///
pub struct Tape<V> {
    //
    // dyp
    /// dynamic parameter specific tape information
    pub(crate) dyp : OpSequence,
    //
    /// variable specific tape information
    pub(crate) var : OpSequence,
    //
    // recording
    /// if false (true) a recording is currently in progress on this tape.
    /// If recording is false, all of the tape's index values are zero
    /// and all of its vectors are empty.
    pub(crate) recording      : bool,
    //
    // tape_id
    /// a different tape_id is chosen for each recording.
    pub(crate) tape_id        : usize,
    //
    // cop
    /// is the vector of constant parameters used by both operation sequences.
    pub(crate) cop : Vec<V>,
}
// ---------------------------------------------------------------------------
// Tape::default
//
impl<V> Default for Tape<V> {
    //
    // Tape::default
    /// Sets dyp, var to new, recording to false, and tape_id to zero.
    /// (The tape with tape_id zero never has recording true.)
    fn default() -> Self {
        Self {
            dyp           : OpSequence::new(),
            var           : OpSequence::new(),
            recording     : false,
            tape_id       : 0,
            cop           : Vec::new(),
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
    /// Sets `tape` to a reference to the tape for recording `AD<V>`
    /// operations.
    ///
    pub trait ThisThreadTape
    where
        Self : Sized + 'static ,
    {
        fn get() -> &'static LocalKey< RefCell< Tape<Self> > >;
    }
}
// impl_this_thread_tape!
/// Implement ThisThreadTape for the value type V
///
/// * V : see [doc_generic_v]
///
/// This macro must be executed once for any type *V*  where
/// `AD<V>` is used. The rustad package automatically executes it
/// for the following types: `f32` , `f64` , `NumVec<f32>`, `NumVec<f64>`.
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
/// ```
macro_rules! impl_this_thread_tape{ ($V:ty) => {
    #[doc = concat!(
        "This threads tape for recording ",
        "`AD<" , stringify!($V), ">` operations"
    ) ]
    impl crate::tape::sealed::ThisThreadTape for $V {
        fn get() -> &'static LocalKey<
                RefCell< crate::tape::Tape<$V> >
            > {
            thread_local! {
                pub(crate) static THIS_THREAD_TAPE : RefCell<
                    crate::tape::Tape<$V>
                > = RefCell::new( crate::tape::Tape::default() );
            }
            &THIS_THREAD_TAPE
        }
    }
} }
pub(crate) use impl_this_thread_tape;
// ----------------------------------------------------------------------------
//
// start_recording
/// This starts recording a new `AD<V>` operation sequence with
/// dynamic parameters.
///
/// * Syntax :
///   ```text
///     (adyp_dom, avar_dom) = start_recording(dyp_dom, var_dom)
///   ```
///
/// * V : see [doc_generic_v]
///
/// * Recording :
///   There must not currently be a recording in process on the current thread
///   when start_recording is called.
///   The recording is stopped when [stop_recording] is called.
///
/// * dyp_dom :
///   If this is None or an empty vector, there must be no dynamic parameters.
///   Otherwise, vector contains the value of the domain dynamic parameters
///   used during the recording.
///
/// * var_dom :
///   This vector contains the value of the domain variables
///   used during the recording.  It can't be empty.
///
/// *adyp_dom :
///   This return is the vector of domain dynamic parameters.
///   It has the same length and values as *dyp_dom* .
///   Dependencies with respect to these parameters will be recorded on the
///   tape for this thread.
///
/// *avar_dom :
///   This return is the vector of domain variables.
///   It has the same length and values as *var_dom* .
///   Dependencies with respect to these variables will be recorded on the
///   tape for this thread.
///
/// * Example : see [stop_recording]
///
pub fn start_recording<V>(
    dyp_dom : Option< Vec<V> >  ,
    var_dom : Vec<V>            ,
) -> ( Vec< AD<V> >, Vec< AD<V> > )
where
    V : FloatCore + Clone + Sized + 'static + sealed::ThisThreadTape ,
{
    assert_ne!( var_dom.len(), 0 );
    //
    // dyp_dom
    let dyp_dom = dyp_dom.unwrap_or_else(|| Vec::new() );
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
            "start_recording_var: This thread's tape is already recording"
        );
        //
        assert_eq!( tape.dyp.id_all.len(),  0 );
        assert_eq!( tape.var.id_all.len(),  0 );
        //
        assert_eq!( tape.dyp.arg_start.len(),  0 );
        assert_eq!( tape.var.arg_start.len(),  0 );
        //
        assert_eq!( tape.dyp.arg_all.len(),  0 );
        assert_eq!( tape.var.arg_all.len(),  0 );
        //
        assert_eq!( tape.dyp.flag_all.len(), 0 );
        assert_eq!( tape.var.flag_all.len(), 0 );
        //
        assert_eq!( tape.cop.len(),          0 );
        //
        // tape: tape_id, recording
        tape.tape_id     = tape_id;
        tape.recording   = true;
        //
        // tape.cop:
        // initialize with NAN at index zero
        tape.cop.push( V::nan() );
        //
        // tape.dyp: n_dom, n_dep
        tape.dyp.n_dom   = dyp_dom.len();
        tape.dyp.n_dep   = 0;
        //
        // tape.var: n_dom, n_dep
        tape.var.n_dom  = var_dom.len();
        tape.var.n_dep  = 0;
    } );
    //
    // adyp_dom
    let adyp_dom = dyp_dom.into_iter().enumerate().map(
        | (index, value) | {
            let ad_type  = ADType::DynamicP;
            AD::new(tape_id, index, ad_type, value)
        }
    ).collect();
    //
    // avar_dom
    let avar_dom = var_dom.into_iter().enumerate().map(
        | (index, value) | {
            let ad_type  = ADType::Variable;
            AD::new(tape_id, index , ad_type, value)
        }
    ).collect();
    //
    (adyp_dom, avar_dom)
}
// ----------------------------------------------------------------------------
// stop_recording
//
/// Stops a recordng and moves it to an ADfn object.
///
/// * Syntax :
///   ```text
///     ad_fn = stop_recording(arange)
///   ```
///
/// * V : see [doc_generic_v]
///
/// * Recording :
///   There must currently be a recording in process on the current thread
///   when stop_recording is called.
///
/// * arange :
///   This is a `Vec< AD<V> >` vector that specifies
///   the range space variables.
///
/// * ad_fn :
///   The return value is an `ADfn<V>` containing the operation sequence
///   that computed arange as a function of the domain variables and
///   dynamic parameters specified by [start_recording] .
///   It can be used to compute the values for the function and its derivatives.
///
/// # Example
/// ```
/// use rustad::start_recording;
/// use rustad::stop_recording;
/// type V       = rustad::AzFloat<f64>;
/// let x        = vec![ V::from(1.0), V::from(2.0) ];
/// let (_, ax)  = start_recording(None, x);
/// let sum      = &ax[0] + &ax[1];
/// let diff     = &ax[0] - &ax[1];
/// let times    = &ax[0] * &ax[1];
/// let ay       = vec![ sum, diff, times ];
/// let ad_fn    = stop_recording(ay);
/// assert_eq!( ad_fn.var_dom_len(), 2);
/// assert_eq!( ad_fn.rng_len(), 3);
/// ```
pub fn stop_recording<V>( arange : Vec< AD<V> > ) -> ADfn<V>
where
    IndexT : TryFrom<usize> ,
    V : Clone + Sized + 'static + sealed::ThisThreadTape ,
{
    // ad_fn
    let mut ad_fn : ADfn<V> = ADfn::default();
    //
    // tape
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
        // index_t_limit
        let index_t_limit : usize = IndexT::MAX as usize;
        //
        // check documented assumptions
        if index_t_limit < tape.tape_id {
            panic!( "tape.tape_id > IndexT::MAX" );
        }
        if index_t_limit < tape.dyp.arg_all.len() {
            panic!( "tape.dyp.arg_all.len() > IndexT::MAX" );
        }
        if index_t_limit < tape.var.arg_all.len() {
            panic!( "tape.var.arg_all.len() > IndexT::MAX" );
        }
        let par_len = tape.cop.len()
            + tape.dyp.n_dom + tape.dyp.n_dep + arange.len();
        if index_t_limit < par_len  {
            panic!( "par_len > IndexT::MAX" );
        }
        //
        // more checks
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
        // end marker for arguments to the last operation
        tape.var.arg_start.push( tape.var.arg_all.len() as IndexT );
        tape.dyp.arg_start.push( tape.dyp.arg_all.len() as IndexT );
        //
        // ad_fn, tape
        std::mem::swap(&mut ad_fn.dyp,  &mut tape.dyp);
        std::mem::swap(&mut ad_fn.var,  &mut tape.var);
        std::mem::swap(&mut ad_fn.cop,  &mut tape.cop);
        //
        // tape_id
        tape.tape_id
    } );
    //
    // rng_ad_type, rng_index, cop
    // TODO: figure out how to do this without any cloning of values.
    for ay_i in &arange {
        if ay_i.tape_id == tape_id {
            ad_fn.rng_ad_type.push( ay_i.ad_type.clone() );
            ad_fn.rng_index.push( ay_i.index as IndexT );
        } else {
            ad_fn.rng_ad_type.push( ADType::ConstantP );
            ad_fn.rng_index.push( ad_fn.cop.len() as IndexT  );
            ad_fn.cop.push( ay_i.value.clone() );
        }
    }
    ad_fn
}
