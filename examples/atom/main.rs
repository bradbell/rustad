// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AD,
    ADfn,
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
// sumsq_forward_zero_value
// sumsq_forward_zero_ad
mod forward_zero;
use forward_zero::{
    sumsq_forward_zero_value,
    sumsq_forward_zero_ad,
};
// -------------------------------------------------------------------------
// Value Routines
// -------------------------------------------------------------------------
//
// sumsq_forward_one_value
fn sumsq_forward_one_value(
    var_zero     : &Vec<V>     ,
    domain_one   : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // domain_zero
    let domain_zero = var_zero;
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // range_one
    let mut range_one = 0 as V;
    for j in 0 .. domain_one.len() {
        range_one += 2.0 * domain_zero[j] * domain_one[j];
    }
    if trace {
        println!("Begin Trace: sumsq_forward_one_value");
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", domain_one[j]);
        }
        println!("]");
        println!("range_one = {}", range_one);
        println!("End Trace: sumsq_forward_one_value");
    }
    vec![ range_one ]
}
//
// sumsq_reverse_one_value
fn sumsq_reverse_one_value(
    var_zero     : &Vec<V>     ,
    range_one    : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // domain_zero
    let domain_zero = var_zero;
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
// sumsq_forward_depend_value
// -------------------------------------------------------------------------
fn sumsq_forward_depend_value(
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
        forward_one_value    :  sumsq_forward_one_value,
        reverse_one_value    :  sumsq_reverse_one_value,
        forward_depend_value :  sumsq_forward_depend_value,
        forward_depend_ad    :  sumsq_forward_depend_ad,
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_atom_eval );
    sumsq_atom_id
}
// -------------------------------------------------------------------------
// AD Routines
// -------------------------------------------------------------------------
//
// sumsq_forward_depend_ad
fn sumsq_forward_depend_ad(
    _is_var_domain  : &Vec<bool> ,
    _call_info      : IndexT     ,
    _trace          : bool       ,
) -> Vec<bool>
{   //
    panic!( "sumsq_forward_depend_ad not implemented");
}
// -------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------
fn call_atomic_fun(sumsq_atom_id : IndexT , trace : bool ) -> ADfn<V> {
    //
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let call_info        = 0 as IndexT;
    let ay               = call_atom(ax, sumsq_atom_id, call_info, trace);
    let f                = stop_recording(ay);
    f
}
fn test_forward_zero_value(sumsq_atom_id : IndexT, trace : bool) {
    //
    // f
    let f = call_atomic_fun(sumsq_atom_id, trace);
    //
    // x, y
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = f.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// test_forward_one
fn test_forward_one_value(sumsq_atom_id : IndexT, trace : bool) {
    //
    // f
    let f = call_atomic_fun(sumsq_atom_id, trace);
    //
    // x, dx, dy
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dx      : Vec<V> = vec![ 5.0, 6.0 ];
    let dy               = f.forward_one_value(&mut v , dx.clone(), trace);
    assert_eq!( dy[0], 2.0 * x[0]*dx[0] + 2.0 * x[1]*dx[1] );
}
//
// test_reverse_one
fn test_reverse_one_value(sumsq_atom_id : IndexT, trace : bool) {
    //
    // f
    let f = call_atomic_fun(sumsq_atom_id, trace);
    //
    // x, dy, dx
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dy      : Vec<V> = vec![ 5.0 ];
    let dx               = f.reverse_one_value(&mut v , dy.clone(), trace);
    assert_eq!( dx[0], 2.0 * x[0]*dy[0] );
    assert_eq!( dx[1], 2.0 * x[1]*dy[0] );
}
// test_forward_zero_ad
fn test_forward_zero_ad(sumsq_atom_id : IndexT, trace : bool) {
    //
    // f
    let f = call_atomic_fun(sumsq_atom_id, trace);
    //
    // g
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
// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------
fn main() {
    let sumsq_atom_id = register_sumsq_atom();
    let trace         = false;
    //
    test_forward_zero_value(sumsq_atom_id, trace);
    test_forward_one_value(sumsq_atom_id, trace);
    test_reverse_one_value(sumsq_atom_id, trace);
    //
    test_forward_zero_ad(sumsq_atom_id, trace);
}
