// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] dead_code method.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
use crate::{
   AD,
    start_recording_dyp_var,
    stop_recording,
    AzFloat,
    ad_from_value,
    IndexT,
    AtomCallback,
    register_atom,
    call_atom,
};
//
// V
type V = AzFloat<f64>;
//
// eye_forward_fun_value
fn eye_forward_fun_value(
    domain     : &Vec<&V>  ,
    _call_info : IndexT    ,
    _trace      : bool     ,
) -> Result< Vec<V>, String >
{   // range
    let mut range : Vec<V> = Vec::with_capacity( domain.len() );
    for i in 0 .. domain.len() {
        range.push( domain[i].clone() );
    }
    Ok(range)
}
//
// eye_rev_depend
fn eye_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String
{   assert_eq!( depend.len(), 0 );
    assert!( rng_index < n_dom );
    depend.push(rng_index);
    //
    let error_msg = String::new();
    error_msg
}
//
// register_eye
fn register_eye() -> IndexT {
    //
    // eye_callback
    let eye_callback = AtomCallback{
        name               : &"eye",
        rev_depend         : Some(eye_rev_depend),
        forward_fun_value  : Some(eye_forward_fun_value) ,
        //
        forward_fun_ad     : None,
        forward_der_value  : None,
        forward_der_ad     : None,
        reverse_der_value  : None,
        reverse_der_ad     : None,
    };
    // eye_atom_id
    let eye_atom_id = register_atom( eye_callback );
    eye_atom_id
}
/*
domain values
p[0]                        used     dynamic  0
p[1]                    not used     dynamic  1
x[0]                        used     variable 0
x[1]                    not used     variable 1
dependent values
q[0] = p[0] + p[1]          used     dynamic  2
q[1] = p[0] * p[1]      not used     dynamic  3
y[0] = x[0] + q[0]          used     variable 2
y[1] = x[0] * q[1]      not used     variable 3
z[0] = 5
z[1] = q[0]                 used     dynamic  2
z[2] = y[0]                 used     variable 2
// call operator creates new dependents even when equal.
w[0] = 5
w[1] = z[1]                 used     dynamic  4
w[2] = z[2]                 used     variable 4
*/
#[test]
fn test_reverse_depend() {
    //
    // trace
    let trace = false;
    //
    // eye_atom_id, call_info
    let eye_atom_id = register_eye();
    let call_info   = 0;
    //
    // f
    let np   = 2;
    let nx   = 2;
    let p    = vec![V::from(1.0); np ];
    let x    = vec![V::from(1.0); nx ];
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // aq
    let mut aq  : Vec< AD<V> > = Vec::new();
    aq.push( &ap[0] + &ap[0] );  // dynamic with index np
    aq.push( &ap[1] * &ap[1] );  // dynamic with index np + 1
    //
    // ay
    let mut ay  : Vec< AD<V> > = Vec::new();
    ay.push( &ax[0] + &aq[0] );  // variable with index nx
    ay.push( &ax[1] * &aq[1] );  // variable with index nx + 1
    //
    // az
    let mut az  : Vec< AD<V> > = Vec::new();
    az.push( ad_from_value( V::from( 5.0 ) ) );  // constant with index 0
    az.push( aq[0].clone() );  // az[1] = dynamic with index np
    az.push( ay[0].clone() );  // az[2] = variable with index nx
    //
    // aw
    // aw[0] = last constant
    // aw[1] = dynamic with index np + 2 depoends on dynamic with index np
    // aw[2] = variable with index nx + 2 depends on variable with index nx
    let aw = call_atom(az, eye_atom_id, call_info, trace);
    let f  = stop_recording(aw);
    //
    let p_both      = f.forward_dyp_value(p, trace);
    f.forward_var_value(&p_both, x, trace);
    //
    // depend
    let depend = f.reverse_depend(trace);
    //
    // depend.cop
    // TODO: There are three constants, but should only be one.
    assert_eq!( depend.cop, [false, false, true] );
    //
    // depend.dyp
    assert_eq!( depend.dyp, [true, false, true, false, true] );
    //
    // depend.var
    assert_eq!( depend.var, [true, false, true, false, true] );
}
