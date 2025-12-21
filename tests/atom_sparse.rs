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
             [     p[1]    ]
The sparsity pattern for this function w.r.t x is
    var_pattern = [ [0, 0], [0, 1], [1, 1] ]
The sparsity pattern for this function w.r.t p is
    dyp_pattern = [ [1 0], [2, 1] ]
*/
use rustad::{
    AD,
    AzFloat,
    register_atom,
    AtomCallback,
    IndexT,
    start_recording_dyp_var,
    stop_recording,
    call_atom,
};
//
// V
type V = AzFloat<f64>;
//
// h_forward_fun_value
pub fn h_forward_fun_value(
    domain       : &Vec<&V>    ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{
    // range
    let mut range : Vec<V> = Vec::new();
    range.push( domain[0] * domain[1] );
    range.push( domain[1] * domain[2] );
    range.push( domain[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_fun_value");
        print!("domain = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
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
    rng_index     : usize           ,
    n_dom         : usize           ,
    _call_info    : IndexT          ,
    _trace        : bool            ,
) -> String
{   assert_eq!( depend.len(), 0 );
    assert_eq!( n_dom, 4 );
    let mut error_msg = String::new();
    match rng_index {
        0 => { depend.push(0); depend.push(1); },
        1 => { depend.push(1); depend.push(2); },
        2 => { depend.push(3); },
        _ => { error_msg += "h_rev_depend: 2 < rng_index"; },
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
    // h_atom_id
    let h_atom_id = register_atom( h_callback );
    h_atom_id
}
//
// atom_dyp
#[test]
fn atom_sparse() {
    //
    // h_atom_id, call_info, trace, compute_dyp, np, nx
    let h_atom_id   = register_h();
    let call_info   = 0;
    let trace       = false;
    let compute_dyp = true;
    let np          = 2;
    let nx          = 2;
    //
    // f
    let p         = vec![ V::from(1.0); np];
    let x         = vec![ V::from(1.0); nx];
    let (ap, ax)  = start_recording_dyp_var(p, x);
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
    // dyp_pattern, var_pattern
    let (mut dyp_pattern, mut var_pattern) = f.sub_sparsity(trace, compute_dyp);
    dyp_pattern.sort();
    var_pattern.sort();
    //
    // dpy_check
    let dyp_check = vec![ [1, 0], [2, 1] ];
    assert_eq!( dyp_pattern, dyp_check );
    //
    // var_check
    let var_check   = vec![ [0, 0], [0, 1], [1, 1], ];
    assert_eq!( var_pattern, var_check );
    //
    // var_pattern
    let compute_dyp = false;
    let mut var_pattern = f.for_sparsity(trace, compute_dyp);
    var_pattern.sort();
    assert_eq!( var_pattern, var_check );
    //
    // dyp_pattern
    let compute_dyp = true;
    let mut dyp_pattern = f.for_sparsity(trace, compute_dyp);
    dyp_pattern.sort();
    assert_eq!( dyp_pattern, dyp_check );
}
