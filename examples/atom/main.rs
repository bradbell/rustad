// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use std::cell::RefCell;
//
use rustad::{
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
mod tests;
use tests::{
    callback_forward_zero_value,
    callback_forward_zero_ad,
    callback_forward_one_value,
    callback_forward_one_ad,
    callback_reverse_one_value,
};
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
fn sumsq_reverse_one_value(
    domain_zero  : &Vec<&V>    ,
    range_one    : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // domain_zero
    assert_eq!( range_one.len(), 1 );
    //
    // domain_one
    let mut domain_one : Vec<V> = Vec::new();
    for j in 0 .. domain_zero.len() {
        domain_one.push( 2.0 * domain_zero[j] * range_one[0] );
    }
    if trace {
        println!("Begin Trace: sumsq_reverse_one_value");
        println!("range_one = [ {} ]", range_one[0]);
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", range_one[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_one_value");
    }
    domain_one
}
//
// sumsq_forward_depend
// -------------------------------------------------------------------------
fn sumsq_forward_depend(
    is_var_domain  : &Vec<bool> ,
    _call_info     : IndexT     ,
    _trace         : bool       ,
) -> Vec<bool>
{
    let mut is_var_range = false;
    for j in 0 .. is_var_domain.len() {
        is_var_range = is_var_range || is_var_domain[j];
    }
    vec![ is_var_range ]
}
// -------------------------------------------------------------------------
// register_sumsq_atom
// -------------------------------------------------------------------------
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_atom_eval
    let sumsq_atom_eval = AtomEval {
        forward_zero_value   :  sumsq_forward_zero_value,
        forward_zero_ad      :  sumsq_forward_zero_ad,
        //
        forward_one_value    :  sumsq_forward_one_value,
        forward_one_ad       :  sumsq_forward_one_ad,
        //
        reverse_one_value    :  sumsq_reverse_one_value,
        forward_depend       :  sumsq_forward_depend,
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
    let sumsq_atom_id = register_sumsq_atom();
    let call_info     = ATOM_ID_VEC.with_borrow_mut(|atom_id_vec| {
        let call_info = atom_id_vec.len() as IndexT;
        atom_id_vec.push( sumsq_atom_id );
        call_info
    } );
    let trace         = false;
    //
    callback_forward_zero_value(sumsq_atom_id, call_info, trace);
    callback_forward_zero_ad(sumsq_atom_id,    call_info, trace);
    //
    callback_forward_one_value(sumsq_atom_id,  call_info, trace);
    callback_forward_one_ad(sumsq_atom_id,  call_info, trace);
    //
    callback_reverse_one_value(sumsq_atom_id,  call_info, trace);
}
