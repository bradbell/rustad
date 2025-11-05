// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
Test Atomic function with dynamic parameters using a different size

The atomic function will be
       [ z[0] * z[1] ]
h(z) = [ z[1] * z[2] ]
       [    z[3]     ]
Note that the domain (range) of h has size 4 (3).

This test has 2 dyanimic parameters p[0], p[1], and one variable x[0].

We define the following function using h:
          [ p[0] * p[1] ]
f(p, x) = [ p[1] * x[0] ]
          [     5.0     ]
*/
use std::cmp::max;
use rustad::{
    AD,
    ad_from_value,
    ADType,
    register_atom,
    AtomEval,
    IndexT,
    start_recording_both,
    stop_recording,
    call_atom,
};
//
// V
type V = f64;
//
// h_forward_type
fn h_forward_type(
    dom_ad_type  : &[ADType]    ,
    _call_info   : IndexT       ,
    _trace       : bool         ,
) -> Vec<ADType>
{   let n_res = 3;
    let mut res_ad_type : Vec<ADType> = Vec::with_capacity(n_res);
    //
    let ad_type = max( dom_ad_type[0].clone(), dom_ad_type[1].clone() );
    res_ad_type.push( ad_type );
    //
    let ad_type = max( dom_ad_type[1].clone() , dom_ad_type[2].clone() );
    res_ad_type.push( ad_type );
    //
    let ad_type = dom_ad_type[3].clone();
    res_ad_type.push( ad_type );
    //
    res_ad_type
}
//
// h_forward_zero_value
pub fn h_forward_zero_value(
    dom_zero     : &Vec<&V>    ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{
    // range
    let mut range : Vec<V> = Vec::new();
    range.push( dom_zero[0] * dom_zero[1] );
    range.push( dom_zero[1] * dom_zero[2] );
    range.push( dom_zero[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_zero_value");
        print!("dom_zero = [ ");
        for j in 0 .. dom_zero.len() {
                print!("{}, ", dom_zero[j]);
        }
        println!("]");
        println!("range = {:?}", range);
        println!("End Trace: h_forward_zero_value");
    }
    range
}
//
// h_forward_zero_ad
pub fn h_forward_zero_ad(
    dom_zero     : &Vec<& AD<V> >    ,
    _call_info   : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{
    // range
    let mut range : Vec< AD<V> > = Vec::new();
    range.push( dom_zero[0] * dom_zero[1] );
    range.push( dom_zero[1] * dom_zero[2] );
    range.push( dom_zero[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_zero_ad");
        print!("dom_zero = [ ");
        for j in 0 .. dom_zero.len() {
                print!("{}, ", dom_zero[j]);
        }
        println!("]");
        println!("range = {:?}", range);
        println!("End Trace: h_forward_zero_ad");
    }
    range
}
//
// register_h
fn register_h()-> IndexT {
    //
    // h_atom_eval
    let h_atom_eval = AtomEval {
        name                 : &"h",
        forward_type         :  h_forward_type,
        //
        forward_zero_value   :  Some( h_forward_zero_value ),
        forward_zero_ad      :  Some( h_forward_zero_ad ),
        //
        forward_one_value    :  None,
        forward_one_ad       :  None,
        //
        reverse_one_value    :  None,
        reverse_one_ad       :  None,
    };
    //
    // h__atom_id
    let h_atom_id = register_atom( h_atom_eval );
    h_atom_id
}
//
// atom_dyp
#[test]
fn atom_dyp() {
    let h_atom_id  = register_h();
    let call_info      = 0;
    let trace          = false;
    //
    // f
    let p   : Vec<V> = vec![ 1.0; 2];
    let x   : Vec<V> = vec![ 1.0; 1];
    let (ap, ax)     = start_recording_both(p, x);
    let z0           = ap[0].clone();
    let z1           = ap[1].clone();
    let z2           = ax[0].clone();
    let z3           = ad_from_value(5.0);
    let az           = vec![ z0, z1, z2, z3 ];
    let ay           = call_atom(az, h_atom_id, call_info, trace);
    let f            = stop_recording(ay);
    //
    let p   : Vec<V> = vec![ 2.0, 3.0 ];
    let x   : Vec<V> = vec![ 4.0 ];
    let q            = f.forward_dyp_value(p.clone(), trace);
    let (y, _v)      = f.forward_var_value(&q, x.clone(), trace);
    //
    // check h_forward_zero_value
    assert_eq!( y.len(), 3 );
    assert_eq!( y[0], p[0] * p[1] );
    assert_eq!( y[1], p[1] * x[0] );
    assert_eq!( y[2], 5.0  );
    //
    // f
    let p   : Vec<V> = vec![ 1.0; 2];
    let x   : Vec<V> = vec![ 1.0; 1];
    let (ap, ax)     = start_recording_both(p.clone(), x.clone());
    let aq           = f.forward_dyp_ad(ap, trace);
    let (ay, _av)    = f.forward_var_ad(&aq, ax, trace);
    let g            = stop_recording(ay);
    //
    let p   : Vec<V> = vec![ 2.0, 3.0 ];
    let x   : Vec<V> = vec![ 4.0 ];
    let q            = g.forward_dyp_value(p.clone(), trace);
    let (y, _v)      = g.forward_var_value(&q, x.clone(), trace);
    //
    // check h_forward_zero_ad
    assert_eq!( y.len(), 3 );
    assert_eq!( y[0], p[0] * p[1] );
    assert_eq!( y[1], p[1] * x[0] );
    assert_eq!( y[2], 5.0  );
}
