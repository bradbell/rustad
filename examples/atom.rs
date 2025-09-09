// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::numvec::{
    start_recording,
    stop_recording,
    register_atom,
    call_atom,
    AtomEval,
    IndexT,
};
//
// V
type V = f64;
//
// sumsq_forward_zero
fn sumsq_forward_zero(
    _var_zero    : &mut Vec<V> ,
    domain_zero  : Vec<&V>     ,
    _trace       : bool        ,
    _call_info   : IndexT      ) -> Vec<V>
{   //
    let mut sumsq : V = 0.into();
    for j in 0 .. domain_zero.len() {
        sumsq += domain_zero[j] * domain_zero[j];
    }
    vec![ sumsq ]
}
//
// atomic_panic
fn atomic_panic(
    _var_zero     : &mut Vec<V> ,
    _domain_zero  : Vec<&V>     ,
    _trace        : bool        ,
    _call_info    : IndexT      ) -> Vec<V>
{   panic!(); }
//
// forward_depend
fn sumsq_forward_depend(
    is_var_domain  : &Vec<bool> ,
    _trace         : bool       ,
    _call_info     : IndexT     ) -> Vec<bool>
{
    let mut is_var_range = false;
    for j in 0 .. is_var_domain.len() {
        is_var_range = is_var_range || is_var_domain[j];
    }
    vec![ is_var_range ]
}

#[test]
fn main() {
    //
    // sumsq_atom_eval
    let sumsq_atom_eval = AtomEval {
        forward_zero   :  sumsq_forward_zero,
        forward_one    :  atomic_panic,
        reverse_one    :  atomic_panic,
        forward_depend :  sumsq_forward_depend,
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_atom_eval );
    //
    // ax
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let call_info        = 0 as IndexT;
    let trace            = false;
    let ay               = call_atom(sumsq_atom_id, call_info, ax, trace);
    let f                = stop_recording(ay);
    let x0      : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v0  : Vec<V> = Vec::new();
    let y0               = f.forward_zero_value(&mut v0, x0, trace);
    assert_eq!( y0[0] , 25.0 );
}
