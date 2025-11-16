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
    domain_zero  : &Vec<&V>    ,
    domain_one   : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{   //
    // domain_zero
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // two_v
    let two_v = V::from(2.0);
    //
    // range_one
    let mut range_one = V::from(0.0);
    for j in 0 .. domain_one.len() {
        range_one += &( &two_v * &( domain_zero[j] * domain_one[j] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_der_value");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", domain_one[j]);
        }
        println!("]");
        println!("range_one = {}", range_one);
        println!("End Trace: sumsq_forward_der_value");
    }
    Ok( vec![ range_one ] )
}
//
// sumsq_forward_der_ad
pub fn sumsq_forward_der_ad(
    domain_zero  : &Vec<& AD<V> >    ,
    domain_one   : Vec<& AD<V> >     ,
    call_info    : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{   //
    // domain_zero
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // atom_id
    let atom_id = ATOM_ID_VEC.with_borrow( |atom_id_vec|
        atom_id_vec[3 * (call_info as usize) + 1]
    );
    //
    // n_domain
    let n_domain = domain_zero.len();
    //
    // for_domain_zero
    let mut for_domain_zero : Vec< AD<V> > = Vec::with_capacity(2 * n_domain);
    for j in 0 .. n_domain {
        for_domain_zero.push( (*domain_zero[j]).clone() );
    }
    for j in 0 .. n_domain {
        for_domain_zero.push( (*domain_one[j]).clone() );
    }
    //
    // range_one
    let range_one = call_atom(for_domain_zero, atom_id, call_info, trace);
    //
    if trace {
        println!("Begin Trace: sumsq_forward_der_ad");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", domain_one[j]);
        }
        println!("]");
        println!("range_one = {}", range_one[0]);
        println!("End Trace: sumsq_forward_der_ad");
    }
    range_one
}
