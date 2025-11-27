// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
rev_sumsq_forward_fun
z = g(x, y) = 2 * y * (x[0], x[1], ... )^T

rev_sumsq_forward_der
dz = g_x(x, y) * dx + g_y(x, y) * dy
   = 2 * y * (dx[0], dx[1], ...)^T + 2 * dy * (x[0],  x[1], ...)^T

rev_sumsq_reverse_der
dx^T = dz^T * g_x(x, y) = 2 * y * ( dz[0],  dz[1], + ... + )
dy   = dz^T * g_y(x, y) = 2 * ( dz[0] * x[0]  + dz[1] * x[1]  + ... )
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
// rev_sumsq_forward_fun_value
// z = g(x,y) = 2 * y * ( x[0], x[1], ... )
fn rev_sumsq_forward_fun_value(
    domain      : &Vec<&V>  ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // nx
    assert!( domain.len() > 1 );
    let nx = domain.len() - 1;
    //
    // x, y
    let x =  &domain[0 .. nx];
    let y =  domain[nx];
    //
    // two_v
    let two_v : V = V::from(2.0);
    //
    // z
    let mut z : Vec<V> = Vec::with_capacity(nx);
    for j in 0 .. nx {
        z.push( &two_v * &( y * x[j] ) );
    }
    //
    Ok( z )
}
//
// rev_sumsq_forward_der_value
fn rev_sumsq_forward_der_value(
    domain      : &Vec<&V>  ,
    domain_der  : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // nx
    assert!( domain.len() > 1 );
    let nx = domain.len() - 1;
    //
    // x, y
    let x =  &domain[0 .. nx];
    let y =  domain[nx];
    //
    // two_v
    let two_v   : V = V::from(2.0);
    //
    // domain_der_zero, domain_der_one
    let dx =  &domain_der[0 .. nx];
    let dy =  domain_der[nx];
    //
    // dz
    let mut dz : Vec<V> = Vec::with_capacity( nx );
    for j in 0 .. nx {
        let mut term_j  = &two_v * &( y * dx[j]  );
        term_j         += &( &two_v * &( dy * x[j] ) );
        dz.push( term_j );
    }
    //
    Ok( dz )
}
//
// rev_sumsq_reverse_der_value
fn rev_sumsq_reverse_der_value(
    domain      : &Vec<&V>  ,
    range_der   : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
{   //
    // nx
    assert_eq!( domain.len(), range_der.len() + 1 );
    let nx = range_der.len();
    //
    // x, y
    let x =  &domain[0 .. nx];
    let y =  domain[nx];
    //
    // two_v
    let two_v   : V = V::from(2.0);
    //
    // dz
    let dz = &range_der;
    //
    // dx_dy
    let mut dx_dy : Vec<V> = Vec::with_capacity(nx + 1);
    let mut dy             = V::from(0.0);
    for j in 0 .. nx {
        dx_dy.push( &two_v * &( y * dz[j] ) );
        dy       += &( &two_v * &( dz[j] * x[j] ) );
    }
    dx_dy.push(dy);
    //
    Ok( dx_dy )
}
//
// sumsq_rev_depend
fn rev_sumsq_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    assert!( n_dom > 1 );
    //
    // nx
    let nx = n_dom - 1;
    //
    let mut error_msg = String::new();
    if nx <= rng_index {
        error_msg += "rev_sumsq_rev_depend: nx <= rng_index";
    } else {
        depend.push( rng_index );
        depend.push( nx );
    }
    error_msg
}
//
// register_rev_sumsq_atom
pub fn register_rev_sumsq_atom()-> IndexT {
    //
    // rev_sumsq_callback
    let rev_sumsq_callback = AtomCallback {
        name                 : &"rev_sumsq",
        rev_depend           :  Some( rev_sumsq_rev_depend ),
        //
        forward_fun_value    :  Some(rev_sumsq_forward_fun_value),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  Some( rev_sumsq_forward_der_value ),
        forward_der_ad       :  None,
        //
        reverse_der_value    :  Some( rev_sumsq_reverse_der_value ),
        reverse_der_ad       :  None,
    };
    //
    // rev_sumsq_atom_id
    let rev_sumsq_atom_id = register_atom( rev_sumsq_callback );
    rev_sumsq_atom_id
}
