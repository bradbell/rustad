// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
rev_sumsq_forward_zero
z = g(x, y) = 2 * y * (x[0], x[1], ... )^T

rev_sumsq_forward_one
dz = g_x(x, y) * dx + g_y(x, y) * dy
   = 2 * y * (dx[0], dx[1], ...)^T + 2 * dy * (x[0],  x[1], ...)^T

rev_sumsq_reverse_one
dx^T = dz^T * g_x(x, y) = 2 * y * ( dz[0],  dz[1], + ... + )
dy   = dz^T * g_y(x, y) = 2 * ( dz[0] * x[0]  + dz[1] * x[1]  + ... )
*/
use rustad::{
    register_atom,
    AtomEval,
    IndexT,
};
//
// V
use super::V;
//
// rev_sumsq_forward_zero_value
// z = g(x,y) = 2 * y * ( x[0], x[1], ... )
fn rev_sumsq_forward_zero_value(
    domain_zero : &Vec<&V>  ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // nx
    assert!( domain_zero.len() > 1 );
    let nx = domain_zero.len() - 1;
    //
    // x, y
    let x =  &domain_zero[0 .. nx];
    let y =  domain_zero[nx];
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // z
    let mut z : Vec<V> = Vec::with_capacity(nx);
    for j in 0 .. nx {
        z.push( &two_v * &( y * x[j] ) );
    }
    //
    z
}
//
// rev_sumsq_forward_one_value
fn rev_sumsq_forward_one_value(
    domain_zero : &Vec<&V>  ,
    domain_one  : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // nx
    assert!( domain_zero.len() > 1 );
    let nx = domain_zero.len() - 1;
    //
    // x, y
    let x =  &domain_zero[0 .. nx];
    let y =  domain_zero[nx];
    //
    // two_v
    let two_v   : V = 2.0 as V;
    //
    // domain_one_zero, domain_one_one
    let dx =  &domain_one[0 .. nx];
    let dy =  domain_one[nx];
    //
    // dz
    let mut dz : Vec<V> = Vec::with_capacity( nx );
    for j in 0 .. nx {
        let mut term_j  = &two_v * &( y * dx[j]  );
        term_j         += &two_v * &( dy * x[j] );
        dz.push( term_j );
    }
    //
    dz
}
//
// rev_sumsq_reverse_one_value
fn rev_sumsq_reverse_one_value(
    domain_zero : &Vec<&V>  ,
    range_one   : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Vec<V>
{   //
    // nx
    assert_eq!( domain_zero.len(), range_one.len() + 1 );
    let nx = range_one.len();
    //
    // x, y
    let x =  &domain_zero[0 .. nx];
    let y =  domain_zero[nx];
    //
    // two_v
    let two_v   : V = 2.0 as V;
    //
    // dz
    let dz = &range_one;
    //
    // dx_dy
    let mut dx_dy : Vec<V> = Vec::with_capacity(nx + 1);
    let mut dy             = 0.0 as V;
    for j in 0 .. nx {
        dx_dy.push( &two_v * &( y * dz[j] ) );
        dy       += &two_v * ( dz[j] * x[j] );
    }
    dx_dy.push(dy);
    //
    dx_dy
}
//
// rev_sumsq_forward_depend
fn rev_sumsq_forward_depend(
    is_var_domain  : &Vec<bool> ,
    _call_info     : IndexT     ,
    _trace         : bool       ,
) -> Vec<bool>
{   //
    // nx
    assert!( is_var_domain.len() > 1 );
    let nx = is_var_domain.len() - 1;
    //
    // is_var_x, is_var_y
    let is_var_x = &is_var_domain[0 .. nx];
    let is_var_y = is_var_domain[nx];
    //
    // is_var_z
    let mut is_var_z : Vec<bool> = Vec::with_capacity(nx);
    for j in 0 .. nx {
        is_var_z.push( is_var_y || is_var_x[j] );
    }
    is_var_z
}
//
// register_rev_sumsq_atom
pub fn register_rev_sumsq_atom()-> IndexT {
    //
    // rev_sumsq_atom_eval
    let rev_sumsq_atom_eval = AtomEval {
        name                 : &"rev_sumsq",
        forward_depend       :  rev_sumsq_forward_depend,
        //
        forward_zero_value   :  rev_sumsq_forward_zero_value,
        forward_zero_ad      :  None,
        //
        forward_one_value    :  Some( rev_sumsq_forward_one_value ),
        forward_one_ad       :  None,
        //
        reverse_one_value    :  Some( rev_sumsq_reverse_one_value ),
        reverse_one_ad       :  None,
    };
    //
    // rev_sumsq_atom_id
    let rev_sumsq_atom_id = register_atom( rev_sumsq_atom_eval );
    rev_sumsq_atom_id
}
