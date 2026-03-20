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
        let mut mode  = "panic";
        let mut text  = "is_zero or is_one is different from recording";
        for opt in opt_vec {
            match opt[0] {
                "mode" => {
                    mode = opt[1];
                },
                "text" => {
                    text = opt[1];
                },
                _ => panic!("is_zero or is_one: opt_vec: invalid key")
            }
        }
        //
        // result
        let result = if check_one {
            self.value.is_one()
        } else {
            self.value.is_zero()
        };
        if mode == "ignore" {
            return result
        };
        //
        // panic_mode
        if mode != "panic" && mode != "print" {
            panic!( "is_zero or is_one: opt_vec: invalid value for mode" );
        }
        let panic_mode = mode == "panic";
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // tape
        local_key.with_borrow_mut( |tape|
            record_zero_one(tape, self, check_one, panic_mode, text, result)
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
    panic_mode : bool          ,
    text       : &str          ,
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
    agraph.bool_all.push( panic_mode );
    agraph.bool_all.push( result );
    //
    // agraph: arg_all, str_all
    agraph.arg_all.push( agraph.str_all.len() as IndexT );
    agraph.str_all += text;
    agraph.arg_all.push( agraph.str_all.len() as IndexT );
    //
    // agraph: arg_all, arg_type_all
    agraph.arg_all.push( arg.index as IndexT );
    for _ in 0 .. 3 {
        agraph.arg_type_all.push( ADType::Empty );
    }
    agraph.arg_type_all.push( arg.ad_type );
}
