// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
for_sumsq_forward_zero
z = g(x, y) = 2 * ( x[0] * y[0] + x[1] * y[1] + ... )

for_sumsq_forward_one
dz = g_x(x, y) * dx + g_y(x, y) * dy
   = 2 * ( dx[0] *  y[0] + dx[1] *  y[1] + ... )
   + 2 * (  x[0] * dy[0] +  x[1] * dy[1] + ... )

for_sumsq_reverse_one
dx^T = dz * g_x(x, y) = 2 * dz * ( y[0], y[1], ... )
dy^T = dz * g_y(x, y) = 2 * dz * ( x[0], x[1], ... )
*/
use rustad::{
    AD,
    register_atom,
    AtomEval,
    IndexT,
};
//
// V
use super::V;
//
// for_sumsq_forward_zero_value
fn for_sumsq_forward_zero_value(
    domain_zero : &Vec<&V>  ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // nx
    let nx = domain_zero.len() / 2;
    assert_eq!( 2 * nx , domain_zero.len() );
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // x, y
    let x = &domain_zero[0 .. nx];
    let y = &domain_zero[nx .. 2 * nx];
    //
    // z
    let mut z = 0.0 as V;
    for j in 0 .. nx {
        z += &two_v * &( x[j] * y[j] );
    }
    //
    vec![ z ]
}
//
// for_sumsq_forward_one_value
fn for_sumsq_forward_one_value(
    domain_zero : &Vec<&V>  ,
    domain_one  : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // domain_zero, domain_one
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // nx
    let nx = domain_zero.len() / 2;
    assert_eq!( 2 * nx , domain_zero.len() );
    //
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // x, y
    let x =  &domain_zero[0 .. nx];
    let y =  &domain_zero[nx .. 2 * nx];
    //
    // dx, dy
    let dx =  &domain_one[0 .. nx];
    let dy =  &domain_one[nx .. 2 * nx];
    //
    // dz
    let mut dz = 0.0 as V;
    for j in 0 .. nx {
        dz += &two_v * &(  x[j] * dy[j] );
        dz += &two_v * &( dx[j] * y[j] );
    }
    //
    vec![ dz ]
}
//
// for_sumsq_reverse_one_value
fn for_sumsq_reverse_one_value(
    domain_zero : &Vec<&V>  ,
    range_one   : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // nx
    let nx = domain_zero.len() / 2;
    assert_eq!( 2 * nx , domain_zero.len() );
    //
    // dz
    assert_eq!( range_one.len(), 1 );
    let dz = range_one[0];
    //
    // factor
    let factor : V = &(2.0 as V) * dz;
    //
    // x, y
    let x =  &domain_zero[0 .. nx];
    let y =  &domain_zero[nx .. 2 * nx];
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
    dx_dy
}
//
// for_sumsq_reverse_one_ad
pub fn for_sumsq_reverse_one_ad(
    _domain_zero  : &Vec<& AD<V> >    ,
    _range_one    : Vec<& AD<V> >     ,
    _call_info    : IndexT            ,
    _trace        : bool              ,
) -> Vec< AD<V> >
{   panic!("for_sumsq_reverse_one_ad: not implemented");
}
//
// for_sumsq_forward_depend
fn for_sumsq_forward_depend(
    is_var_domain  : &Vec<bool> ,
    _call_info     : IndexT     ,
    _trace         : bool       ,
) -> Vec<bool>
{
    // nx
    let nx = is_var_domain.len() / 2;
    assert_eq!( 2 * nx , is_var_domain.len() );
    //
    // is_var_x, is_var_y
    let is_var_x = &is_var_domain[0 .. nx];
    let is_var_y = &is_var_domain[nx .. 2 * nx];
    //
    let mut is_var_z = false;
    for j in 0 .. nx {
        if is_var_x[j] || is_var_y[j] {
            is_var_z = true;
        }
    }
    vec![ is_var_z ]
}
//
// register_for_sumsq_atom
pub fn register_for_sumsq_atom()-> IndexT {
    //
    // for_sumsq_atom_eval
    let for_sumsq_atom_eval = AtomEval {
        name                 : &"for_sumsq",
        forward_depend       :  for_sumsq_forward_depend,
        //
        forward_zero_value   :  for_sumsq_forward_zero_value,
        forward_zero_ad      :  None,
        //
        forward_one_value    :  Some( for_sumsq_forward_one_value ),
        forward_one_ad       :  None,
        //
        reverse_one_value    :  Some( for_sumsq_reverse_one_value ),
        reverse_one_ad       :  for_sumsq_reverse_one_ad,
        //
    };
    //
    // for_sumsq_atom_id
    let for_sumsq_atom_id = register_atom( for_sumsq_atom_eval );
    for_sumsq_atom_id
}
