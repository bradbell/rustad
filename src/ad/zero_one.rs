// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub module defines the is_zero and is_one `AD<V>` member functions.
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// use
use std::thread::LocalKey;
use std::cell::RefCell;
//
use crate::{
    AD,
    IndexT,
    FValue,
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::op::id;
use crate::tape::sealed::ThisThreadTape;
//
// panic_fn
fn panic_fn(check_one : bool, message : &str) {
    if check_one {
        panic!( "is_one: {}", message);
    } else {
        panic!( "is_zero: {}", message);
    }
}
// -------------------------------------------------------------------------
// doc_zero_one
/// The is_zero and is_one `AD<V>` member functions
///
/// These are similar to the is_zero and is_one functions in [FValue].
///
/// * Syntax :
///   ```text
///     bval = aval.is_zero(opt_vec)
///     bval = aval.is_one(opt_vec)
///   ```
///
/// * Prototype : see [AD::is_zero], [AD::is_one] .
///
/// * aval :
///   an `AD<V>` value usually created by one of the numerical comparisons;
///   see [doc_f_binary_ad](crate::ad::f_binary::doc_f_binary_ad).
///
/// * bval :
///
///   * is_zero :
///     ```text
///         bval = aval.to_value().is_zero()
///     ```
///     In this case fn_name is is_zero.
///
///   * is_one :
///     ```text
///         bval = aval.to_value().is_one()
///     ```
///     In this case fn_name is is_one.
///
/// * domain_set_by :
///
///   The value bval during a recording may determine what operations are
///   recorded and placed in a corresponding function.
///   If aval depends on the domain parameters or variables.
///   bval might have been different.
///   The places that the domain values can be set are:
///   [forward_dyp_value](crate::ADfn::forward_dyp_value) ,
///   [forward_var_value](crate::ADfn::forward_var_value).
///
/// * opt_vec :
///   controls what should happen when bval would have changed.
///   The [opt_vec](crate::doc_opt_vec) argument
///   has the following possible keys:
///
///   * ignore :
///     the corresponding value must be true of false (default is false).
///     If it is true, the change is ignored (nothing is done).
///
///   * panic :
///     the corresponding value must be true of false (default is true).
///     If it is true and the change occurs, the current thread will panic
///     with an error message describing the problem.
///
///   * message :
///     the corresponding value is a message to use when the changes occurs
///     (default is unspecified).
///     If panic is true, the message is used in the panic.
///     Otherwise the message is stored in a thread static variable.
///     The messages stored in this thread static variable can be retrieve
///     using [pop_this_thread_message](crate::pop_this_thread_message).
///     This retrieval is on a last in first out basis.
///
/// * option = pop_this_thread_message() :
///
///   * None :
///     if option is None, no messages are in the zero_one message stack.
///
///   * Some( total_message ) :
///     If a message was pushed by use of an is_zero (is_one) function,
///     total_message is
///     ```text
///         domain_set_by + ": " + fn_name + ": " + message
///     ```
///     where domain_set_by is forward_dyp_value or forward_var_value.
///
/// * example : see examples/zero_one.rs.
///
#[cfg(doc)]
pub fn doc_zero_one() { }
//
impl<V> AD<V>
where
    V : FValue + ThisThreadTape,
{
    //
    /// see [doc_zero_one]
    pub fn is_zero(&self, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   let check_one = false;
        self.zero_one(check_one, opt_vec)
    }
    //
    /// see [doc_zero_one]
    pub fn is_one(&self, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   let check_one = true;
        self.zero_one(check_one, opt_vec)
    }
    //
    fn zero_one(&self, check_one : bool, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   //
        let mut ignore   = false;
        let mut panic    = true;
        let mut message  = "value was different during recording";
        for opt in opt_vec {
            match opt[0] {
                "ignore" => {
                    match opt[1] {
                        "true"  => { ignore = true; },
                        "false" => { ignore = false; },
                        _ => { panic_fn(
                            check_one, "opt_vec: invalid value for ignore"
                        ) },
                    }
                },
                "panic" => {
                    match opt[1] {
                        "true"  => { panic = true; },
                        "false" => { panic = false; },
                        _ => { panic_fn(
                            check_one, "opt_vec: invalid value for panic"
                        ) },
                    }
                },
                "message" => {
                    message = opt[1];
                },
                _ => panic_fn(check_one, "opt_vec: invalid key")
            }
        }
        //
        // result
        let result = if check_one {
            self.value.is_one()
        } else {
            self.value.is_zero()
        };
        if ignore {
            return result
        };
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // tape
        local_key.with_borrow_mut( |tape|
            record_zero_one(tape, self, check_one, panic, message, result)
        );
        //
        // result
        result
    }
}
//
// record_zero_one
fn record_zero_one<V>(
    tape       : &mut Tape<V>  ,
    arg        : &AD<V>        ,
    check_one  : bool          ,
    panic      : bool          ,
    message    : &str          ,
    result     : bool          ,
)
{   //
    if (! tape.recording) || (arg.tape_id != tape.tape_id) {
        return;
    }
    debug_assert!( arg.ad_type != ADType::ConstantP );
    //
    // agraph
    let agraph = if arg.ad_type.is_variable() {
        &mut tape.var
    } else {
        debug_assert!( arg.ad_type.is_dynamic() );
        &mut tape.dyp
    };
    //
    // agraph: id_all, n_dep, arg_start
    agraph.id_all.push( id::ZERO_ONE_OP );
    agraph.n_dep += 1; // This value is never used
    agraph.arg_start.push( agraph.arg_all.len() as IndexT );
    //
    // agraph: arg_all, bool_all
    agraph.arg_all.push( agraph.bool_all.len() as IndexT );
    agraph.bool_all.push( check_one );
    agraph.bool_all.push( panic );
    agraph.bool_all.push( result );
    //
    // agraph: arg_all, str_all
    agraph.arg_all.push( agraph.str_all.len() as IndexT );
    agraph.str_all += message;
    agraph.arg_all.push( agraph.str_all.len() as IndexT );
    //
    // agraph: arg_all, arg_type_all
    agraph.arg_all.push( arg.index as IndexT );
    for _ in 0 .. 3 {
        agraph.arg_type_all.push( ADType::Empty );
    }
    agraph.arg_type_all.push( arg.ad_type );
}
