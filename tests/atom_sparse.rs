// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
Test rust_src with an atomic function and dynamic parameters.

The atomic function will be
       [ z[0] * z[1] ]
h(z) = [ z[1] * z[2] ]
       [    z[3]     ]
Note that the domain (range) of h has size 4 (3).

We define the following function using h:
             [ x[0] * x[1] ]
    f(p,x) = [ x[1] * p[0] ]
             [     p[2]    ]
The sparsity pattern for this function w.r.t x is
    [0. 0], [0, 1} [1, 1]
*/
use std::cmp::max;
use rustad::{
    AD,
    AzFloat,
    ADType,
    register_atom,
    AtomCallback,
    IndexT,
    start_recording_dyp,
    stop_recording,
    call_atom,
};
//
// V
type V = AzFloat<f64>;
//
// h_forward_type
fn h_forward_type(
    dom_ad_type  : &[ADType]    ,
    _call_info   : IndexT       ,
    _trace       : bool         ,
) -> Result< Vec<ADType>, String >
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
    Ok( res_ad_type )
}
//
// h_forward_fun_value
pub fn h_forward_fun_value(
    dom_zero     : &Vec<&V>    ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{
    // range
    let mut range : Vec<V> = Vec::new();
    range.push( dom_zero[0] * dom_zero[1] );
    range.push( dom_zero[1] * dom_zero[2] );
    range.push( dom_zero[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_fun_value");
        print!("dom_zero = [ ");
        for j in 0 .. dom_zero.len() {
                print!("{}, ", dom_zero[j]);
        }
        println!("]");
        println!("range = {:?}", range);
        println!("End Trace: h_forward_fun_value");
    }
    Ok( range )
}
//
// h_rev_depend
pub fn h_rev_depend(
    depend        : &mut Vec<usize> ,
    range_index   : usize           ,
    _n_dom        : usize           ,
    _call_info    : IndexT          ,
    _trace        : bool            ,
) -> String
{   assert_eq!( depend.len(), 0 );
    let mut error_msg = String::new();
    match range_index {
        0 => {
            depend.push(0);
            depend.push(1);
        },
        1 => {
            depend.push(1);
            depend.push(2);
        },
        2 => {
            depend.push(3);
        },
        _ => {
            error_msg = String::from( "h_rev_depend: 2 < range_index" );
        },
    }
    //
    error_msg
}
//
// register_h
fn register_h()-> IndexT {
    //
    // h_callback
    let h_callback = AtomCallback {
        name                 : &"h",
        //
        rev_depend           :  Some( h_rev_depend ),
        forward_type         :  Some( h_forward_type ),
        //
        forward_fun_value    :  Some( h_forward_fun_value ),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  None,
        forward_der_ad       :  None,
        //
        reverse_der_value    :  None,
        reverse_der_ad       :  None,
    };
    //
    // h__atom_id
    let h_atom_id = register_atom( h_callback );
    h_atom_id
}
//
// atom_dyp
#[test]
fn atom_sparse() {
    //
    // h_atom_id, call_info, trace, nx
    let h_atom_id  = register_h();
    let call_info  = 0;
    let trace      = true;
    let np         = 2;
    let nx         = 2;
    //
    // f
    let p         = vec![ V::from(1.0); np];
    let x         = vec![ V::from(1.0); nx];
    let (ap, ax)  = start_recording_dyp(p, x);
    let mut az : Vec< AD<V> > = Vec::new();
    az.push( ax[0].clone() );
    az.push( ax[1].clone() );
    az.push( ap[0].clone() );
    az.push( ap[1].clone() );
    let ay  = call_atom(az, h_atom_id, call_info, trace);
    let f   = stop_recording(ay);
    assert_eq!(f.dyp_dom_len(), np);
    assert_eq!(f.var_dom_len(), nx);
    //
    // pattern
    let mut pattern = f.sub_sparsity(trace);
    pattern.sort();
    //
    // check
    let check   = vec![ [0, 0], [0, 1], [1, 1], ];
    assert_eq!( pattern, check );
    //
    // pattern
    let mut pattern = f.for_sparsity(trace);
    pattern.sort();
    assert_eq!( pattern, check );
}
