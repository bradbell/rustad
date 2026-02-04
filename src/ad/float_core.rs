// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
// ---------------------------------------------------------------------------
//! This pub(crate) module implements FloatCore for AD types
//!
//! Link to [parent module](super)
//!
//!
// ---------------------------------------------------------------------------
use std::thread::LocalKey;
use std::cell::RefCell;
use crate::{
    FloatCore,
    AD,
    IndexT,
};
use crate::ad::ADType;
use crate::tape::Tape;
use crate::tape::sealed::ThisThreadTape;
use crate::op::id;
// ---------------------------------------------------------------------------
macro_rules! impl_unary_float_core{ ($name:ident) => { paste::paste! {
    #[doc = concat!( "& `AD<V>.`", stringify!( $name ), "()")]
    fn $name(self) -> Self {
        //
        // record
        fn record<V : FloatCore>(
            tape : &mut Tape<V> ,
            arg  : AD<V>        ,
        ) -> AD<V> {
            //
            // tape_id, ad_type, value
            let AD::<V> { tape_id, ad_type, value, .. } = arg;
            //
            // new_value
            let new_value = value.$name();
            //
            // new_tape_id, new_index, new_ad_type
            let mut new_tape_id   = 0;
            let mut new_index     = 0;
            let mut new_ad_type   = ADType::ConstantP;
            if (! tape.recording) || (tape_id != tape.tape_id) {
                return AD::new(new_tape_id, new_index, new_ad_type, new_value);
            }
            debug_assert!( ad_type != ADType::ConstantP );
            //
            // new_tape_id, new_ad_type
            new_tape_id = tape.tape_id;
            new_ad_type = ad_type.clone();
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
            op_seq.n_dep += 1;
            op_seq.arg_start.push( op_seq.arg_all.len() as IndexT );
            op_seq.arg_type_all.push( new_ad_type.clone() );
            op_seq.id_all.push( id::[< $name:upper _OP >] );
            //
            AD::new(new_tape_id, new_index, new_ad_type, new_value)

        }
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // new_tape_id, new_index, new_ad_type
        let new_ad = local_key.with_borrow_mut(
            |tape| record( tape, self)
        );
        new_ad
    }
} } }
//
/// Implements the FloatCore trait for AD types
impl<V> FloatCore for AD<V>
where
    V : Clone + FloatCore + ThisThreadTape,
{
        fn nan()  -> Self { AD::<V>::from( V::nan() ) }
        fn zero() -> Self { AD::<V>::from( V::zero() ) }
        fn one()  -> Self { AD::<V>::from( V::one() ) }
        //
        // unary functions
        impl_unary_float_core!(sin);
}
