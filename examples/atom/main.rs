// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use std::cell::RefCell;
//
use rustad::{
    AD,
    ADfn,
    ad_from_vector,
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
// ATOM_ID_VEC
thread_local! {
    pub static ATOM_ID_VEC : RefCell< Vec<IndexT> > = RefCell::new(Vec::new());
}
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
// Tests
// -------------------------------------------------------------------------
//
// value_callback_f
// f(x) = x[0] * x[0] + x[1] * x[1] + ...
fn value_callback_f(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) -> ADfn<V> {
    //
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let ay               = call_atom(ax, sumsq_atom_id, call_info, trace);
    let f                = stop_recording(ay);
    f
}
//
// callback_forward_zero_value
fn callback_forward_zero_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, y
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = f.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_zero_ad
fn callback_forward_zero_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // g(x) = f(x)
    let x      : Vec<V>       = vec![ 3.0 , 4.0 ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    let ay  = f.forward_zero_ad(&mut av , ax.clone(), trace);
    let g   = stop_recording(ay);
    //
    // x, y
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = g.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_one_value
fn callback_forward_one_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, dx, dy
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dx      : Vec<V> = vec![ 5.0, 6.0 ];
    let dy               = f.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], 2.0 * x[0]*dx[0] + 2.0 * x[1]*dx[1] );
}
//
// callback_forward_one_ad
fn callback_forward_one_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // g, dx1
    // g(x) = f'(x) * dx1 = 2 * ( x[0] * dx1[0] + x[2] * dx1[2] + ... )
    let x      : Vec<V>       = vec![ 3.0 , 4.0 ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av , ax, trace);
    let dx1     : Vec<V> = vec![ 5.0, 6.0 ];
    let adx1             = ad_from_vector(dx1.clone());
    let ady              = f.forward_one_ad(&av, adx1, trace);
    let g                = stop_recording(ady);
    //
    // x, dx2, dy
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    g.forward_zero_value(&mut v , x.clone(), trace);
    let dx2     : Vec<V> = vec![ 7.0, 8.0 ];
    let dy               = g.forward_one_value(&v , dx2.clone(), trace);
    //
    assert_eq!( dy[0], 2.0 * dx1[0]*dx2[0] + 2.0 * dx1[1]*dx2[1] );
}
//
// callback_reverse_one_value
fn callback_reverse_one_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, dy, dx
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dy      : Vec<V> = vec![ 5.0 ];
    let dx               = f.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], 2.0 * x[0]*dy[0] );
    assert_eq!( dx[1], 2.0 * x[1]*dy[0] );
}
//
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
