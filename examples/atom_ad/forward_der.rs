// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
sumsq_forward_der
dy = g'(x) * dx = 2 * ( x[0] * dx[0] + x[1] * dx[1] + ... )
*/
//
use rustad::{
    AD,
    IndexT,
    call_atom,
};
//
// V, ATOM_ID_VEC
use super::{
    V,
    ATOM_ID_VEC,
};
//
// sumsq_forward_der_value
pub fn sumsq_forward_der_value(
    _use_range   : &[bool]     ,
    domain       : &[&V]       ,
    domain_der   : &[&V]       ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{   //
    // domain
    assert_eq!( domain.len(), domain_der.len() );
    //
    // two_v
    let two_v = V::from(2.0);
    //
    // range_der
    let mut range_der = V::from(0.0);
    for j in 0 .. domain_der.len() {
        range_der += &( &two_v * &( domain[j] * domain_der[j] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_der_value");
        print!("domain      = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        print!("domain_der = [ ");
        for j in 0 .. domain_der.len() {
                print!("{}, ", domain_der[j]);
        }
        println!("]");
        println!("range_der = {}", range_der);
        println!("End Trace: sumsq_forward_der_value");
    }
    Ok( vec![ range_der ] )
}
//
// sumsq_forward_der_ad
pub fn sumsq_forward_der_ad(
    _use_range   : &[bool]           ,
    domain       : &[& AD<V>]        ,
    domain_der   : &[& AD<V>]        ,
    call_info    : IndexT            ,
    trace        : bool              ,
) -> Result< Vec< AD<V> >, String >
{   //
    // domain
    assert_eq!( domain.len(), domain_der.len() );
    //
    // atom_id
    let atom_id = ATOM_ID_VEC.with_borrow( |atom_id_vec|
        atom_id_vec[3 * (call_info as usize) + 1]
    );
    //
    // n_domain
    let n_domain = domain.len();
    //
    // for_domain
    let mut for_domain      : Vec< AD<V> > = Vec::with_capacity(2 * n_domain);
    for j in 0 .. n_domain {
        for_domain.push( (*domain[j]).clone() );
    }
    for j in 0 .. n_domain {
        for_domain.push( (*domain_der[j]).clone() );
    }
    //
    // range_der
    let n_range   = 1;
    let range_der = call_atom(
        n_range, for_domain, atom_id, call_info, trace
    );
    //
    if trace {
        println!("Begin Trace: sumsq_forward_der_ad");
        print!("domain      = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        print!("domain_der = [ ");
        for j in 0 .. domain_der.len() {
                print!("{}, ", domain_der[j]);
        }
        println!("]");
        println!("range_der = {}", range_der[0]);
        println!("End Trace: sumsq_forward_der_ad");
    }
    Ok( range_der )
}
