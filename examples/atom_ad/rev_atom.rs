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
    ADType,
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
    domain_zero : &Vec<&V>  ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
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
    domain_zero : &Vec<&V>  ,
    domain_one  : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
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
    let two_v   : V = V::from(2.0);
    //
    // domain_one_zero, domain_one_one
    let dx =  &domain_one[0 .. nx];
    let dy =  domain_one[nx];
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
    domain_zero : &Vec<&V>  ,
    range_one   : Vec<&V>   ,
    _call_info  : IndexT    ,
    _trace      : bool      ,
) -> Result< Vec<V>, String >
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
    let two_v   : V = V::from(2.0);
    //
    // dz
    let dz = &range_one;
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
// rev_sumsq_forward_type
fn rev_sumsq_forward_type(
    domain_ad_type  : &[ADType]    ,
    _call_info      : IndexT       ,
    _trace          : bool         ,
) -> Result< Vec<ADType>, String >
{
    //
    // nx
    assert!( domain_ad_type.len() > 1 );
    let nx = domain_ad_type.len() - 1;
    //
    // x_ad_type, y_ad_type
    let x_ad_type = &domain_ad_type[0 .. nx];
    let y_ad_type = &domain_ad_type[nx];
    //
    let mut z_ad_type : Vec<ADType> = Vec::with_capacity(nx);
    for j in 0 .. nx {
        let ad_type = std::cmp::max(y_ad_type.clone(), x_ad_type[j].clone());
        z_ad_type.push( ad_type );
    }
    Ok( z_ad_type )
}
//
// sumsq_rev_depend
fn rev_sumsq_rev_depend(
    depend       : &mut Vec<usize> ,
    range_index  : usize           ,
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
    if nx <= range_index {
        error_msg += "rev_sumsq_rev_depend: nx <= range_index";
    } else {
        depend.push( range_index );
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
        forward_type         :  Some( rev_sumsq_forward_type ),
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
