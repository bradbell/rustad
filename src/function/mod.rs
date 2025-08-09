// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD function objects
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// sub-modules
//
// sparsity
pub mod sparsity;
//
// sweep
pub mod sweep;
// ---------------------------------------------------------------------------
// use
//
use std::cell::RefCell;
use std::thread::LocalKey;
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::gad::GAD;
use crate::gas::as_from;
use crate::gas::sealed::GenericAs;
use crate::record::sealed::ThisThreadTape;
use crate::record::{NEXT_TAPE_ID, GTape};
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::doc_generic_f_and_u;
//
#[cfg(doc)]
use crate::operator;
// -----------------------------------------------------------------------
// GADFun
//
/// This can evaluate an operation sequence function and its derivatives.
///
/// * Operation sequence :
/// An operation sequence is a single assignment representation of
/// the function; i.e., each variable is only assigned once.
///
/// * Constructor :
/// An [ad_domain] call is used to start recording an operation sequence.
/// An [ad_fun] call is used to stop recording move the operation sequence
/// to an new ADFun object.
///
/// * F, U : see [doc_generic_f_and_u]
///
pub struct GADFun<F,U> {
    //
    // n_domain
    /// The dimension of the domain space for this function.
    /// The domain variables have index 0 .. n_domain-1.
    pub(crate) n_domain            : usize,
    //
    // n_var
    /// The total number of variables in the operation sequence.
    pub(crate) n_var               : usize,
    //
    // id_all
    /// This maps an operator's index in the operation sequence
    /// to its [operator::id]
    pub(crate) id_all              : Vec<u8>,
    //
    // range_is_var
    /// The length of this vector is the dimension of the range space.
    /// If range_is_var\[i\] is true (false), the i-th range space component
    /// is a variable (constant).
    pub(crate) range_is_var        : Vec<bool>,
    //
    // flag_all
    /// This contains boolean flags that are part of some operator definitions.
    pub(crate) flag_all            : Vec<bool>,
    //
    // range2tape_index
    /// The length of this vector is also the dimension of the range space.
    /// If range_is_var\[i\] is true (false), range2tape_indx\[i\] is the
    /// variable (constant) index for the i-th component of the range space.
    pub(crate) range2tape_index    : Vec<U>,
    //
    // op2arg
    /// This maps an operator's index in the operation sequence to its
    /// the index of its first argument in arg_all.
    pub(crate) op2arg              : Vec<U>,
    //
    // arg_all
    /// This contains the arguments for all the opereators in the
    /// operatioon sequence.
    pub(crate) arg_all             : Vec<U>,
    //
    // con_all
    /// This contains the value of all the constants needed
    /// to evaluate the function.
    pub(crate) con_all             : Vec<F>,
}
//
// ---------------------------------------------------------------------------
impl<F,U> GADFun<F,U> {
    //
    // new
    /// This creates an empty operation sequence.
    ///
    /// To be more specific,
    /// its domain and range vectors have length zero.
    ///
    /// * F, U : see [doc_generic_f_and_u]
    ///
    /// # Example
    /// ```
    /// use rustad::GADFun;
    /// let f : GADFun<f32,u32> = GADFun::new();
    /// assert_eq!( f.domain_len(), 0 );
    /// assert_eq!( f.range_len(), 0 );
    /// ```
    pub fn new() -> Self {
        Self {
            n_domain         : 0,
            n_var            : 0,
            id_all           : Vec::new() ,
            range_is_var     : Vec::new() ,
            flag_all         : Vec::new() ,
            range2tape_index : Vec::new() ,
            op2arg           : Vec::new() ,
            arg_all          : Vec::new() ,
            con_all          : Vec::new() ,
        }
    }
    //
    // domain_len
    /// dimension of domain space
    pub fn domain_len(&self) -> usize { self.n_domain }
    //
    // range_len
    /// dimension of range space
    pub fn range_len(&self) -> usize { self.range_is_var.len() }
}
// ----------------------------------------------------------------------------
// ad_domain
//
/// This starts recording a new operation sequence.
///
/// * Recording :
/// There must not currently be a recording in process on the current thread
/// when ad_domain is called. The recording is stopped when [ad_fun] is called.
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * domain :
/// This vector determines the number of domain (independent) variables
/// and their value during the recording.
///
/// * ad_domain :
/// The return is a vector of variables
/// with the same length and values as domain.
/// Dependencies with respect to these variables will be recorded on
/// the tape for this thread.
///
pub fn ad_domain<F,U>( domain : &[F] ) -> Vec< GAD<F,U> >
where
    F     : Copy + Sized + 'static + ThisThreadTape<U> ,
    U     : Sized + 'static ,
    usize : Sized + 'static + GenericAs<U> ,
{
    //
    // new_tape_id
    let new_tape_id : usize;
    {   let mut next_tape_id = NEXT_TAPE_ID.lock().unwrap();
        //
        // The rest of this block has a lock, so it is fast and can't fail.
        new_tape_id   = *next_tape_id;
        *next_tape_id = new_tape_id + 1;
    }
    let local_key : &LocalKey< RefCell< GTape<F,U> > > =
        < F as ThisThreadTape<U> >::get();
    local_key.with_borrow_mut( |tape| {
        assert_ne!( new_tape_id, 0);
        assert!( ! tape.recording , "indepndent: tape is already recording");
        assert_eq!( tape.id_all.len(), 0 );
        assert_eq!( tape.op2arg.len(), 0 );
        assert_eq!( tape.arg_all.len(), 0 );
        assert_eq!( tape.con_all.len(), 0 );
        assert_eq!( tape.flag_all.len(), 0 );
        tape.tape_id        = new_tape_id;
        tape.recording      = true;
        tape.n_domain       = domain.len();
        tape.n_var          = domain.len();
        //
    } );
    let mut result : Vec< GAD<F,U> > = Vec::new();
    for j in 0 .. domain.len() {
        result.push(  GAD {
            tape_id   : as_from( new_tape_id ),
            var_index : as_from( j ),
            value     : domain[j],
        } );
    }
    result
}
// ----------------------------------------------------------------------------
// ad_fun
//
/// Stops a recordng and moves it to an ADFun object.
///
/// * Recording :
/// There must currently be a recording in process on the current thread
/// ( started by [ad_domain] ).
///
/// * F, U : see [doc_generic_f_and_u]
///
/// * ad_range :
/// This is an AD vector of range space variables.
///
/// * ad_fun :
/// The return value is an ADFun containing the sequence of operations
/// that computed ad_range as a function of [ad_domain].
/// It can compute the range space variables and derivarives
/// as a function of the domain space variables.
///
/// * Assumptions :
/// The following assumptions are checked for the tape for this thread:
///     1. tape.arg_all.len() <= U::Max
///     2. tape.tape_id       <= U::Max
///
pub fn ad_fun<F,U>( ad_range : &[ GAD<F,U> ] ) -> GADFun<F,U>
where
    F     : Copy + Sized + 'static + ThisThreadTape<U> ,
    U     : Copy + Sized + 'static + TryFrom<usize> + GenericAs<usize> ,
    usize : GenericAs<U> ,
{
    let mut result : GADFun<F,U> = GADFun::new();
    let local_key : &LocalKey< RefCell< GTape<F,U> > > =
        < F as ThisThreadTape<U> >::get();
    let tape_id : usize = local_key.with_borrow_mut( |tape| {
        //
        // tape.recording
        assert!( tape.recording , "indepndent: tape is not recording");
        tape.recording = false;
        //
        // check assumptions
        assert_eq!( tape.n_var , tape.n_domain + tape.id_all.len() );
        assert_eq!( tape.op2arg.len() , tape.id_all.len() );
        match U::try_from( tape.arg_all.len() ) {
            Err(_) => panic!( "tape.arg_all.len() > U::MAX" ),
            Ok(_)  => (),
        }
        match U::try_from( tape.tape_id ) {
            Err(_) => panic!( "tape.tape_id > U::MAX" ),
            Ok(_)  => (),
        }
        //
        // tape.op2arg
        // end marker for arguments to the last operation
        tape.op2arg.push( as_from( tape.arg_all.len() ) );
        //
        std::mem::swap( &mut result.n_domain,      &mut tape.n_domain );
        std::mem::swap( &mut result.n_var,         &mut tape.n_var );
        std::mem::swap( &mut result.id_all,        &mut tape.id_all );
        std::mem::swap( &mut result.op2arg,        &mut tape.op2arg );
        std::mem::swap( &mut result.arg_all,       &mut tape.arg_all );
        std::mem::swap( &mut result.con_all,       &mut tape.con_all );
        std::mem::swap( &mut result.flag_all,      &mut tape.flag_all );
        tape.tape_id
    } );
    //
    // range_is_var, range2tape_index
    for i in 0 .. ad_range.len() {
        if as_from( ad_range[i].tape_id ) == tape_id {
            result.range_is_var.push( true );
            result.range2tape_index.push( ad_range[i].var_index );
        } else {
            result.range_is_var.push( false );
            result.range2tape_index.push(
                as_from( result.con_all.len()  )
            );
            result.con_all.push( ad_range[i].value );
        }
    }
    result
}
