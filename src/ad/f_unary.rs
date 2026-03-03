// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module implements FUnary for AD types
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
use crate::{
    FUnary,
    AD,
    IndexT,
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::op::id;
// ---------------------------------------------------------------------------
macro_rules! impl_float_unary{ ($name:ident) => { paste::paste! {
    #[doc = concat!( "`AD<V>.`", stringify!( $name ), "()")]
    fn $name(self) -> AD<V> {
        //
        // record
        fn record<V>(
            tape      : &mut Tape<V> ,
            arg       : &AD<V>       ,
            new_value : V            ,
        ) -> AD<V>
        {
            //
            // new_tape_id, new_index, new_ad_type
            let mut new_tape_id   = 0;
            let mut new_index     = 0;
            let mut new_ad_type   = ADType::ConstantP;
            if (! tape.recording) || (arg.tape_id != tape.tape_id) {
                return AD::new(new_tape_id, new_index, new_ad_type, new_value);
            }
            debug_assert!( arg.ad_type != ADType::ConstantP );
            //
            // new_tape_id, new_ad_type
            new_tape_id = tape.tape_id;
            new_ad_type = arg.ad_type;
            //
            // op_seq
            let op_seq = if new_ad_type == ADType::Variable {
                &mut tape.var
            } else {
                &mut tape.dyp
            };
            //
            // new_index
            new_index = op_seq.n_dep + op_seq.n_dom;
            //
            // op_seq: n_dep, arg_start, arg_type, id_all
            op_seq.id_all.push( id::[< $name:upper _OP >] );
            op_seq.n_dep += 1;
            op_seq.arg_start.push( op_seq.arg_all.len() as IndexT );
            op_seq.arg_all.push( arg.index as IndexT );
            op_seq.arg_type_all.push( new_ad_type );
            //
            AD::new(new_tape_id, new_index, new_ad_type, new_value)

        }
        //
        // new_value
        let new_value = self.value.$name();
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        let result = local_key.with_borrow_mut(
            |tape| record( tape, self, new_value)
        );
        result
    }
} } }
//
/// Implements the FUnary trait for AD types
impl<V> FUnary for &AD<V>
where
    V : Clone + ThisThreadTape,
    for<'a> &'a V : FUnary<Output=V>,
{
    type Output = AD<V>;
    //
    // unary functions
    impl_float_unary!(ln);
    impl_float_unary!(sqrt);
    impl_float_unary!(tanh);
    impl_float_unary!(tan);
    impl_float_unary!(sinh);
    impl_float_unary!(cosh);
    impl_float_unary!(abs);
    impl_float_unary!(signum);
    impl_float_unary!(exp);
    impl_float_unary!(minus);
    impl_float_unary!(cos);
    impl_float_unary!(sin);
    //
    // powi
    /// `AD<V>`.powi(`i32`)
    fn powi(self, rhs : i32) -> AD<V> {
        //
        // record
        fn record<V>(
            tape      : &mut Tape<V> ,
            arg       : &AD<V>       ,
            rhs       : i32          ,
            new_value : V            ,
        ) -> AD<V>
        {
            //
            // new_tape_id, new_index, new_ad_type
            let mut new_tape_id   = 0;
            let mut new_index     = 0;
            let mut new_ad_type   = ADType::ConstantP;
            if (! tape.recording) || (arg.tape_id != tape.tape_id) {
                return AD::new(new_tape_id, new_index, new_ad_type, new_value);
            }
            debug_assert!( arg.ad_type != ADType::ConstantP );
            //
            // new_tape_id, new_ad_type
            new_tape_id = tape.tape_id;
            new_ad_type = arg.ad_type;
            //
            // op_seq
            let op_seq = if new_ad_type == ADType::Variable {
                &mut tape.var
            } else {
                &mut tape.dyp
            };
            //
            // new_index
            new_index = op_seq.n_dep + op_seq.n_dom;
            //
            // op_seq: n_dep, arg_start, arg_type, id_all
            op_seq.id_all.push( id::POWI_OP );
            op_seq.n_dep += 1;
            op_seq.arg_start.push( op_seq.arg_all.len() as IndexT );
            op_seq.arg_all.push( arg.index as IndexT );
            if rhs >= 0 {
                op_seq.arg_all.push( rhs as IndexT );
                op_seq.arg_all.push( 0 );
            } else {
                op_seq.arg_all.push( - rhs as IndexT );
                op_seq.arg_all.push( 1 );
            }
            op_seq.arg_type_all.push( new_ad_type );
            op_seq.arg_type_all.push( ADType::Empty );
            op_seq.arg_type_all.push( ADType::Empty );
            //
            AD::new(new_tape_id, new_index, new_ad_type, new_value)
        }
        //
        // new_value
        let new_value = self.value.powi(rhs);
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        local_key.with_borrow_mut(
            |tape| record(tape, self, rhs, new_value)
        )
    }
}
