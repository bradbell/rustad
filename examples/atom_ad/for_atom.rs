// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
for_sumsq_forward_fun
z = g(x, y) = 2 * ( x[0] * y[0] + x[1] * y[1] + ... )

for_sumsq_forward_der
dz = g_x(x, y) * dx + g_y(x, y) * dy
   = 2 * ( dx[0] *  y[0] + dx[1] *  y[1] + ... )
   + 2 * (  x[0] * dy[0] +  x[1] * dy[1] + ... )

for_sumsq_reverse_der
dx^T = dz * g_x(x, y) = 2 * dz * ( y[0], y[1], ... )
dy^T = dz * g_y(x, y) = 2 * dz * ( x[0], x[1], ... )
*/
use rustad::{
    register_atom,
    AtomCallback,
    IndexT,
};
//
// V
use super::V;
//
// for_sumsq_forward_fun_value
fn for_sumsq_forward_fun_value(
    _use_range  : &[bool]   ,
    domain      : &[&V]     ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // nx
    let nx = domain.len() / 2;
    assert_eq!( 2 * nx , domain.len() );
    //
    // two_v
    let two_v : V = V::from(2.0);
    //
    // x, y
    let x = &domain[0 .. nx];
    let y = &domain[nx .. 2 * nx];
    //
    // z
    let mut z = V::from(0.0);
    for j in 0 .. nx {
        z += &( &two_v * &( x[j] * y[j] ) );
    }
    //
    Ok( vec![ z ] )
}
//
// for_sumsq_forward_der_value
fn for_sumsq_forward_der_value(
    _use_range  : &[bool]   ,
    domain      : &[&V]     ,
    domain_der  : &[&V]     ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // domain, domain_der
    assert_eq!( domain.len(), domain_der.len() );
    //
    // nx
    let nx = domain.len() / 2;
    assert_eq!( 2 * nx , domain.len() );
    //
    //
    // two_v
    let two_v : V = V::from(2.0);
    //
    // x, y
    let x =  &domain[0 .. nx];
    let y =  &domain[nx .. 2 * nx];
    //
    // dx, dy
    let dx =  &domain_der[0 .. nx];
    let dy =  &domain_der[nx .. 2 * nx];
    //
    // dz
    let mut dz = V::from(0.0);
    for j in 0 .. nx {
        dz += &( &two_v * &(  x[j] * dy[j] ) );
        dz += &( &two_v * &( dx[j] * y[j] ) );
    }
    //
    Ok( vec![ dz ] )
}
//
// for_sumsq_reverse_der_value
fn for_sumsq_reverse_der_value(
    domain      : &[&V]     ,
    range_der   : &[&V]     ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // nx
    let nx = domain.len() / 2;
    assert_eq!( 2 * nx , domain.len() );
    //
    // dz
    assert_eq!( range_der.len(), 1 );
    let dz = range_der[0];
    //
    // factor
    let factor : V = &(V::from(2.0)) * dz;
    //
    // x, y
    let x =  &domain[0 .. nx];
    let y =  &domain[nx .. 2 * nx];
    //
    // dx_dy
    let mut dx_dy : Vec<V> = Vec::with_capacity( 2 * nx );
    for j in 0 .. nx {
        dx_dy.push(&factor * y[j]);
    }
    for j in 0 .. nx {
        dx_dy.push(&factor * x[j]);
    }
    //
    Ok( dx_dy )
}
//
// for_sumsq_rev_depend
fn for_sumsq_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    let mut error_msg = String::new();
    if 0 < rng_index {
        error_msg += "for_sumsq_rev_depend: 0 < rng_index";
    } else {
        for j in 0 .. n_dom {
            depend.push( j );
        }
    }
    error_msg
}
//
// register_for_sumsq_atom
pub fn register_for_sumsq_atom()-> IndexT {
    //
    // for_sumsq_callback
    let for_sumsq_callback = AtomCallback {
        name                 : &"for_sumsq",
        rev_depend           :  Some( for_sumsq_rev_depend ),
        //
        forward_fun_value    :  Some(for_sumsq_forward_fun_value),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  Some( for_sumsq_forward_der_value ),
        forward_der_ad       :  None,
        //
        reverse_der_value    :  Some( for_sumsq_reverse_der_value ),
        reverse_der_ad       :  None,
    };
    //
    // for_sumsq_atom_id
    let for_sumsq_atom_id = register_atom( for_sumsq_callback );
    for_sumsq_atom_id
}
