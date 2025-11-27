// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
This atomic example uses all the possible AtomCallback function.
The sumsq_forward_der_ad and reverse_forward_der_ad callbacks each
require defining another atomic function to evaluate their derivatives.

sumsq_forward_fun; see forward_zero.rs
z = g(x) = x[0] * x[0] + x[1] * x[1] + ...

sumsq_forward_der: see forward_one.rs
dz = g'(x) * dx = 2 * ( x[0] * dx[0] + x[1] * dx[1] + ... )

sumsq_reverse_der; see reverse_one.rs
dx^T = dz * g'(x) = 2 * dz * ( x[0], x[1], ... )

for_sumsq_forward_fun; see for_atom.rs
z = g(x, y) = 2 * ( x[0] * y[0] + x[1] * y[1] + ... )

rev_sumsq_forward_fun; see rev_atom.rs
z = g(x, y) = 2 * y * (x[0], x[1], ... )^T

TODO: The following change of argument names in callbacks for this example:
    domain_zero -> domain
    ramge_one   -> range_der
    domain_der  -> domain_der
*/
use std::cell::RefCell;
//
use rustad::{
    AzFloat,
    register_atom,
    AtomCallback,
    IndexT,
};
//
// V
type V = AzFloat<f64>;
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
// sumsq_forward_fun_value
// sumsq_forward_fun_ad
mod forward_fun;
use forward_fun::{
    sumsq_forward_fun_value,
    sumsq_forward_fun_ad,
};
//
// sumsq_forward_der_value
// sumsq_forward_der_ad
mod forward_der;
use forward_der::{
    sumsq_forward_der_value,
    sumsq_forward_der_ad,
};
//
// sumsq_reverse_der_value
// sumsq_reverse_der_ad
mod reverse_der;
use reverse_der::{
    sumsq_reverse_der_value,
    sumsq_reverse_der_ad,
};
//
// sumsq_rev_depend
fn sumsq_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    let mut error_msg = String::new();
    if 0 < rng_index {
        error_msg += "sumsq_rev_depend: 0 < rng_index";
    } else {
        for j in 0 .. n_dom {
            depend.push( j );
        }
    }
    error_msg
}
//
// register_sumsq_atom
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_callback
    let sumsq_callback = AtomCallback {
        name                 : &"sumsq",
        rev_depend           :  Some( sumsq_rev_depend ),
        //
        forward_fun_value    :  Some(sumsq_forward_fun_value),
        forward_fun_ad       :  Some( sumsq_forward_fun_ad ),
        //
        forward_der_value    :  Some( sumsq_forward_der_value ),
        forward_der_ad       :  Some( sumsq_forward_der_ad ),
        //
        reverse_der_value    :  Some( sumsq_reverse_der_value ),
        reverse_der_ad       :  Some( sumsq_reverse_der_ad ),
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_callback );
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
    tests::callback_forward_fun_value(sumsq_atom_id, call_info, trace);
    tests::callback_forward_fun_ad(sumsq_atom_id,    call_info, trace);
    //
    tests::callback_forward_der_value(sumsq_atom_id,  call_info, trace);
    tests::callback_forward_der_ad(sumsq_atom_id,  call_info, trace);
    //
    tests::callback_reverse_der_value(sumsq_atom_id,  call_info, trace);
    tests::callback_reverse_der_ad(sumsq_atom_id,  call_info, trace);
}
