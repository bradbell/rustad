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
    FloatValue,
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::op::id;
use crate::tape::sealed::ThisThreadTape;
// ---------------------------------------------------------------------------
thread_local! {
    static ZERO_ONE_MESSAGE :
        RefCell< Vec<String> > = const { RefCell::new( Vec::new() ) };
}
//
// push_zero_one_message
pub(crate) fn push_zero_one_message(message : String)
{   let local_key = &ZERO_ONE_MESSAGE;
    local_key.with_borrow_mut( |vec_str| {
        vec_str.push( message.to_string() );
     } );
}
//
// pop_zero_one_message
pub fn pop_zero_one_message()->Option<String>
{   let local_key = &ZERO_ONE_MESSAGE;
    local_key.with_borrow_mut(
        |vec_str| vec_str.pop()
     )
}
//
// panic_fn
fn panic_fn(check_one : bool, message : &str) {
    if check_one {
        panic!( "is_one: {}", message);
    } else {
        panic!( "is_zero: {}", message);
    }
}
// ---------------------------------------------------------------------------
impl<V> AD<V>
where
    V : FloatValue + ThisThreadTape,
{
    //
    pub fn is_zero(&self, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   let check_one = false;
        self.zero_one(check_one, opt_vec)
    }
    //
    pub fn is_one(&self, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   let check_one = true;
        self.zero_one(check_one, opt_vec)
    }
    //
    fn zero_one(&self, check_one : bool, opt_vec : &Vec< [&str; 2] > ) -> bool
    {   //
        let mut ignore   = false;
        let mut panic    = true;
        let mut message  = if check_one {
            "is_one: value is different than during recording"
        } else {
            "is_zero: value is different than during recording"
        };
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
