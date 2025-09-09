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
// sumsq_panic
fn sumsq_panic(
    _var_zero     : &mut Vec<V> ,
    _domain_zero  : &Vec<&V>    ,
    _trace        : bool        ,
    _call_info    : IndexT      ) -> Vec<V>
{   panic!( "atomic sumsq not implemented for this case"); }
//
// sumsq_forward_zero
fn sumsq_forward_zero(
    var_zero     : &mut Vec<V> ,
    domain_zero  : &Vec<&V>    ,
    trace        : bool        ,
    call_info    : IndexT      ) -> Vec<V>
{   //
    assert_eq!( var_zero.len(), 0 );
    let mut sumsq = 0 as V;
    for j in 0 .. domain_zero.len() {
        sumsq += domain_zero[j] * domain_zero[j];
        if call_info == 1 {
            var_zero.push( *domain_zero[j] )
        }
    }
    if trace {
        println!("Begin Trace: sumsq_forward_one");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        println!("]");
        println!("sumsq = {}", sumsq);
        println!("End Trace: sumsq_forward_one");
    }
    vec![ sumsq ]
}
//
// sumsq_forward_one
fn sumsq_forward_one(
    domain_zero  : &mut Vec<V> ,
    domain_one   : &Vec<&V>    ,
    _trace       : bool        ,
    call_info    : IndexT      ) -> Vec<V>
{   //
    assert_eq!( call_info,  1 );
    assert_eq!( domain_zero.len(), domain_one.len() );
    let mut sumsq = 0 as V;
    for j in 0 .. domain_one.len() {
        sumsq += 2.0 * domain_zero[j] * domain_one[j];
    }
    vec![ sumsq ]
}
//
// sumsq_forward_depend
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
//
// register_sumsq_atom
fn register_sumsq_atom()-> IndexT {
    //
    // sumsq_atom_eval
    let sumsq_atom_eval = AtomEval {
        forward_zero_value   :  sumsq_forward_zero,
        forward_one_value    :  sumsq_forward_one,
        reverse_one_value    :  sumsq_panic,
        forward_depend       :  sumsq_forward_depend,
    };
    //
    // sumsq_atom_id
    let sumsq_atom_id = register_atom( sumsq_atom_eval );
    sumsq_atom_id
}
//
// test_forward_zero
fn test_forward_zero(sumsq_atom_id : IndexT) {
    //
    // ax
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let call_info        = 0 as IndexT;
    let trace            = false;
    let ay               = call_atom(sumsq_atom_id, call_info, ax, trace);
    let f                = stop_recording(ay);
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = f.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// test_forward_one
fn test_forward_one(sumsq_atom_id : IndexT) {
    //
    // ax
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let call_info        = 1 as IndexT;
    let trace            = false;
    let ay               = call_atom(sumsq_atom_id, call_info, ax, trace);
    let f                = stop_recording(ay);
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dx      : Vec<V> = vec![ 5.0, 6.0 ];
    let dy               = f.forward_one_value(&mut v , dx.clone(), trace);
    assert_eq!( dy[0], 2.0 * x[0]*dx[0] + 2.0 * x[1]*dx[1] );
}

#[test]
fn main() {
    let sumsq_atom_id = register_sumsq_atom();
    test_forward_zero(sumsq_atom_id);
    test_forward_one(sumsq_atom_id);
}
