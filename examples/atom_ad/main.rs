// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
This atomic example uses all the possible AtomEval callback function.
The sumsq_forward_one_ad and reverse_forward_one_ad callbacks each
require defining another atomic function to evaluate their derivatives.

sumsq_forward_zero; see forward_zero.rs
z = g(x) = x[0] * x[0] + x[1] * x[1] + ...

sumsq_forward_one: see forward_one.rs
dz = g'(x) * dx = 2 * ( x[0] * dx[0] + x[1] * dx[1] + ... )

sumsq_reverse_one; see reverse_one.rs
dx^T = dz * g'(x) = 2 * dz * ( x[0], x[1], ... )

for_sumsq_forward_zero; see for_atom.rs
z = g(x, y) = 2 * ( x[0] * y[0] + x[1] * y[1] + ... )

rev_sumsq_forward_zero; see rev_atom.rs
z = g(x, y) = 2 * y * (x[0], x[1], ... )^T
*/
use std::cell::RefCell;
//
use rustad::{
    ADType,
    register_atom,
    AtomEval,
    IndexT,
};
//
// V
type V = f64;
//
// ATOM_ID_VEC
thread_local! {
    pub static ATOM_ID_VEC : RefCell< Vec<IndexT> > = RefCell::new(Vec::new());
}
//
// tests
mod tests;
//
// for_atom, rev_atom
mod for_atom;
mod rev_atom;
//
// sumsq_forward_zero_value
// sumsq_forward_zero_ad
mod forward_zero;
use forward_zero::{
    sumsq_forward_zero_value,
    sumsq_forward_zero_ad,
};
//
// sumsq_forward_one_value
// sumsq_forward_one_ad
mod forward_one;
use forward_one::{
    sumsq_forward_one_value,
    sumsq_forward_one_ad,
};
//
// sumsq_reverse_one_value
// sumsq_reverse_one_ad
mod reverse_one;
use reverse_one::{
    sumsq_reverse_one_value,
    sumsq_reverse_one_ad,
};
//
// sumsq_forward_type
fn sumsq_forward_type(
    domain_ad_type  : &[ADType]    ,
    _call_info      : IndexT       ,
    _trace          : bool         ,
) -> Vec<ADType>
{
    let mut max_ad_type = ADType::ConstantP;
    for ad_type in domain_ad_type.iter() {
        max_ad_type = std::cmp::max( max_ad_type, ad_type.clone() );
    }
    vec![ max_ad_type ]
}
//
// register_sumsq_atom
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_atom_eval
    let sumsq_atom_eval = AtomEval {
        name                 : &"sumsq",
        forward_type         :  sumsq_forward_type,
        //
        forward_zero_value   :  Some(sumsq_forward_zero_value),
        forward_zero_ad      :  Some( sumsq_forward_zero_ad ),
        //
        forward_one_value    :  Some( sumsq_forward_one_value ),
        forward_one_ad       :  Some( sumsq_forward_one_ad ),
        //
        reverse_one_value    :  Some( sumsq_reverse_one_value ),
        reverse_one_ad       :  Some( sumsq_reverse_one_ad ),
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_atom_eval );
    sumsq_atom_id
}
// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------
fn main() {
    let sumsq_atom_id     = register_sumsq_atom();
    let for_sumsq_atom_id = for_atom::register_for_sumsq_atom();
    let rev_sumsq_atom_id = rev_atom::register_rev_sumsq_atom();
    let call_info     = ATOM_ID_VEC.with_borrow_mut(|atom_id_vec| {
        let call_info = 3 * (atom_id_vec.len() as IndexT);
        atom_id_vec.push( sumsq_atom_id );
        atom_id_vec.push( for_sumsq_atom_id );
        atom_id_vec.push( rev_sumsq_atom_id );
        call_info
    } );
    let trace         = false;
    //
    tests::callback_forward_zero_value(sumsq_atom_id, call_info, trace);
    tests::callback_forward_zero_ad(sumsq_atom_id,    call_info, trace);
    //
    tests::callback_forward_one_value(sumsq_atom_id,  call_info, trace);
    tests::callback_forward_one_ad(sumsq_atom_id,  call_info, trace);
    //
    tests::callback_reverse_one_value(sumsq_atom_id,  call_info, trace);
    tests::callback_reverse_one_ad(sumsq_atom_id,  call_info, trace);
}
