// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
// use
use rustad::{
    AD,
    start_recording_dyp_var,
    stop_recording,
    AzFloat,
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
#[test]
fn atom_opt() {
    //
    // trace
    let trace = false;
    //
    // eye_atom_id, call_info
    let eye_atom_id = register_eye();
    let call_info   = 0;
    //
    // f
    let p    = vec![V::from(1.0), V::from(2.0) ];
    let x    = vec![V::from(3.0), V::from(4.0) ];
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // aq
    let mut aq  : Vec< AD<V> > = Vec::new();
    aq.push( &ap[0] + &ap[0] );  // q[0] = p[0] + p[0]
    aq.push( &ap[1] * &ap[1] );  // q[1] = p[1] * p[1]
    //
    // ay
    let mut ay  : Vec< AD<V> > = Vec::new();
    ay.push( &ax[0] + &ap[0] );  // y[0] = x[0] + p[0]
    ay.push( &ax[1] * &ap[1] );  // y[1] = x[1] * p[1]
    //
    // az
    let mut az  : Vec< AD<V> > = Vec::new();
    az.push( aq[0].clone() );  // z[0] = q[0]
    az.push( aq[1].clone() );  // z[1] = q[1]
    //
    // aw
    // w = z
    let aw = call_atom(az, eye_atom_id, call_info, trace);
    //
    // au
    let mut au : Vec< AD<V> > = Vec::new();
    au.push( aw[1].clone() ); // u[0] = w[1]
    //
    // f, n_dyp, n_var
    let mut f = stop_recording(au);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (u, _u) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!(u[0], &p[1] * &p[1] );
    //
    // n_dyp_dep, n_var_dep
    let n_dyp_dep = f.dyp_dep_len();
    let n_var_dep = f.var_dep_len();
    assert_eq!( n_dyp_dep, 4); // q[0], q[1], w[0], w[1]
    assert_eq!( n_var_dep, 2); // y[0], y[1]
    //
    // optimize
    f.optimize(trace);
    //
    // n_dyp_dep, n_var_dep
    let n_dyp_dep = f.dyp_dep_len();
    let n_var_dep = f.var_dep_len();
    assert_eq!( n_dyp_dep, 2); // q[1], w[1]
    assert_eq!( n_var_dep, 0);
    //
    // check f
    let p_      = f.forward_dyp_value(p.clone(), trace);
    let (u, _u) = f.forward_var_value(&p_, x.clone(), trace);
    assert_eq!(u[0], &p[1] * &p[1] );
}
