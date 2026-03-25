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
// doc_f_unary_ad
/// `AD<V>` unary functions
///
/// * Syntax : ``ay = ax.Name()``
///
/// * V : see [doc_generic_v](crate::doc_generic_v)
///
/// * Name : is the name of one of the [FUnary] functions.
///
/// * ax : is an `AD<V>` or `&AD<V>` object.
///
/// # Example
/// ```
/// use rustad::{
///     AD,
///     AzFloat,
///     FConst,
///     FUnary,
///     nearly_eq,
/// };
/// type V = AzFloat<f64>;
/// let opt_vec : Vec<[&str; 2]> = Vec::new();
/// //
/// let pi          = AD::<V>::pi();
/// let pi_4        = pi / AD::from( V::from(4.0) );
/// let y           = (&pi_4).tan();
/// nearly_eq::<V>(&y.to_value(), &V::from(1.0), &opt_vec);
/// let y           = pi_4.clone().tan();
/// nearly_eq::<V>(&y.to_value(), &V::from(1.0), &opt_vec);
/// let y           = FUnary::tan( pi_4 );
/// nearly_eq::<V>(&y.to_value(), &V::from(1.0), &opt_vec);
/// ```
pub fn doc_f_unary_ad() {}
// ---------------------------------------------------------------------------
macro_rules! unary_self_borrowed{ ($name:ident) => { paste::paste! {
    #[doc = "see doc_f_unary_ad" ]
    fn $name(self) -> AD<V> {
        //
        // new_value
        let new_value = self.value.$name();
        //
        // op_id
        let op_id = id::[< $name:upper _OP >];
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        let result = local_key.with_borrow_mut(
            |tape| record_unary( tape, self, new_value, op_id)
        );
        result
    }
} } }
macro_rules! unary_self_owned{ ($name:ident) => { paste::paste! {
    #[doc = "see doc_f_unary_ad" ]
    fn $name(self) -> AD<V> {
        //
        // new_value
        let new_value = self.value.$name();
        //
        // op_id
        let op_id = id::[< $name:upper _OP >];
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        let result = local_key.with_borrow_mut(
            |tape| record_unary( tape, &self, new_value, op_id)
        );
        result
    }
} } }
//
/// Implements the FUnary trait `&AD<V>`
impl<V> FUnary for &AD<V>
where
    V : Clone + ThisThreadTape,
    for<'a> &'a V : FUnary<Output=V>,
{
    type Output = AD<V>;
    //
    // use unary_self_borrowed
    unary_self_borrowed!(square);
    unary_self_borrowed!(ln_1p);
    unary_self_borrowed!(exp_m1);
    unary_self_borrowed!(ln);
    unary_self_borrowed!(sqrt);
    unary_self_borrowed!(tanh);
    unary_self_borrowed!(tan);
    unary_self_borrowed!(sinh);
    unary_self_borrowed!(cosh);
    unary_self_borrowed!(abs);
    unary_self_borrowed!(signum);
    unary_self_borrowed!(exp);
    unary_self_borrowed!(minus);
    unary_self_borrowed!(cos);
    unary_self_borrowed!(sin);
    //
    // powi
    /// `AD<V>`.powi(`i32`)
    fn powi(self, rhs : i32) -> AD<V> {
        //
        // new_value
        let new_value = self.value.powi(rhs);
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        local_key.with_borrow_mut(
            |tape| record_powi(tape, self, rhs, new_value)
        )
    }
}
//
/// Implements the FUnary trait `AD<V>`
impl<V> FUnary for AD<V>
where
    V : Clone + ThisThreadTape,
    for<'a> &'a V : FUnary<Output=V>,
{
    type Output = AD<V>;
    //
    // use unary_self_owned
    unary_self_owned!(square);
    unary_self_owned!(ln_1p);
    unary_self_owned!(exp_m1);
    unary_self_owned!(ln);
    unary_self_owned!(sqrt);
    unary_self_owned!(tanh);
    unary_self_owned!(tan);
    unary_self_owned!(sinh);
    unary_self_owned!(cosh);
    unary_self_owned!(abs);
    unary_self_owned!(signum);
    unary_self_owned!(exp);
    unary_self_owned!(minus);
    unary_self_owned!(cos);
    unary_self_owned!(sin);
    //
    // powi
    /// `AD<V>`.powi(`i32`)
    fn powi(self, rhs : i32) -> AD<V> {
        //
        //
        // new_value
        let new_value = self.value.powi(rhs);
        //
        // local_key
        let local_key : &LocalKey<RefCell< Tape<V> >> = ThisThreadTape::get();
        //
        // result
        local_key.with_borrow_mut(
            |tape| record_powi(tape, &self, rhs, new_value)
        )
    }
}
//
// record_unary
fn record_unary<V>(
    tape      : &mut Tape<V> ,
    arg       : &AD<V>       ,
    new_value : V            ,
    op_id     : u8           ,
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
    // agraph
    let agraph = if new_ad_type == ADType::Variable {
        &mut tape.var
    } else {
        &mut tape.dyp
    };
    //
    // new_index
    new_index = agraph.n_dep + agraph.n_dom;
    //
    // agraph: n_dep, arg_start, arg_type, id_all
    agraph.id_all.push( op_id );
    agraph.n_dep += 1;
    agraph.arg_start.push( agraph.arg_all.len() as IndexT );
    agraph.arg_all.push( arg.index as IndexT );
    agraph.arg_type_all.push( new_ad_type );
    //
    AD::new(new_tape_id, new_index, new_ad_type, new_value)

}
//
// record_powi
fn record_powi<V>(
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
    // pow(x, 0) = 1
    if rhs == 0 {
        return AD::new(new_tape_id, new_index, new_ad_type, new_value);
    }
    //
    // pow(x, 1) = x
    if rhs == 1 {
        return AD::new(arg.tape_id, arg.index, arg.ad_type, new_value);
    }
    //
    // new_tape_id, new_ad_type
    new_tape_id = tape.tape_id;
    new_ad_type = arg.ad_type;
    //
    // agraph
    let agraph = if new_ad_type == ADType::Variable {
        &mut tape.var
    } else {
        &mut tape.dyp
    };
    //
    // new_index
    new_index = agraph.n_dep + agraph.n_dom;
    //
    // agraph: n_dep, arg_start, arg_type, id_all
    agraph.id_all.push( id::POWI_OP );
    agraph.n_dep += 1;
    agraph.arg_start.push( agraph.arg_all.len() as IndexT );
    agraph.arg_all.push( arg.index as IndexT );
    if rhs >= 0 {
        agraph.arg_all.push( rhs as IndexT );
        agraph.arg_all.push( 0 );
    } else {
        agraph.arg_all.push( - rhs as IndexT );
        agraph.arg_all.push( 1 );
    }
    agraph.arg_type_all.push( new_ad_type );
    agraph.arg_type_all.push( ADType::Empty );
    agraph.arg_type_all.push( ADType::Empty );
    //
    AD::new(new_tape_id, new_index, new_ad_type, new_value)
}
