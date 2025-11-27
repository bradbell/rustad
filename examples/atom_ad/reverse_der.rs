// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
sumsq_reverse_der
dx^T = dy * g'(x) = 2 * dy * ( x[0], x[1], ... )
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
// sumsq_reverse_der_value
pub fn sumsq_reverse_der_value(
    domain       : &Vec<&V>    ,
    range_der    : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{   //
    // range_der
    assert_eq!( range_der.len(), 1 );
    //
    // two_v
    let two_v = V::from(2.0);
    //
    // domain_der
    let mut domain_der : Vec<V> = Vec::with_capacity( domain.len() );
    for j in 0 .. domain.len() {
        domain_der.push( &two_v * &( domain[j] * range_der[0] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_reverse_der_value");
        println!("range_der = [ {} ]", range_der[0]);
        print!("domain_der = [ ");
        for j in 0 .. domain_der.len() {
                print!("{}, ", domain_der[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_der_value");
    }
    Ok( domain_der )
}
//
// sumsq_reverse_der_ad
pub fn sumsq_reverse_der_ad(
    domain       : &Vec<& AD<V> >    ,
    range_der    : Vec<& AD<V> >     ,
    call_info    : IndexT            ,
    trace        : bool              ,
) -> Result< Vec< AD<V> >, String >
{   //
    // range_der
    assert_eq!( range_der.len(), 1 );
    //
    // atom_id
    let atom_id = ATOM_ID_VEC.with_borrow( |atom_id_vec|
        atom_id_vec[3 * (call_info as usize) + 2]
    );
    //
    // n_domain
    let n_domain = domain.len();
    //
    // rev_domain
    let mut rev_domain      : Vec< AD<V> > = Vec::with_capacity(n_domain + 1);
    for j in 0 .. domain.len() {
        rev_domain.push( (*domain[j]).clone() );
    }
    rev_domain.push( (*range_der[0]).clone() );
    //
    // domain_der
    let domain_der = call_atom(rev_domain, atom_id, call_info, trace);
    //
    if trace {
        println!("Begin Trace: sumsq_reverse_der_ad");
        print!("domain      = [ ");
        for j in 0 .. n_domain {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("range_der = [ {} ]", range_der[0]);
        print!("domain_der = [ ");
        for j in 0 .. n_domain {
                print!("{}, ", domain_der[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_der_ad");
    }
    Ok( domain_der )
}
