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
    domain_zero  : &Vec<&V>    ,
    range_one    : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // range_one
    assert_eq!( range_one.len(), 1 );
    //
    // two_v
    let two_v = 2.0 as V;
    //
    // domain_one
    let mut domain_one : Vec<V> = Vec::with_capacity( domain_zero.len() );
    for j in 0 .. domain_zero.len() {
        domain_one.push( &two_v * &( domain_zero[j] * range_one[0] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_reverse_der_value");
        println!("range_one = [ {} ]", range_one[0]);
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", domain_one[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_der_value");
    }
    domain_one
}
//
// sumsq_reverse_der_ad
pub fn sumsq_reverse_der_ad(
    domain_zero  : &Vec<& AD<V> >    ,
    range_one    : Vec<& AD<V> >     ,
    call_info    : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{   //
    // range_one
    assert_eq!( range_one.len(), 1 );
    //
    // atom_id
    let atom_id = ATOM_ID_VEC.with_borrow( |atom_id_vec|
        atom_id_vec[3 * (call_info as usize) + 2]
    );
    //
    // n_domain
    let n_domain = domain_zero.len();
    //
    // rev_domain_zero
    let mut rev_domain_zero : Vec< AD<V> > = Vec::with_capacity(n_domain + 1);
    for j in 0 .. domain_zero.len() {
        rev_domain_zero.push( (*domain_zero[j]).clone() );
    }
    rev_domain_zero.push( (*range_one[0]).clone() );
    //
    // domain_one
    let domain_one = call_atom(rev_domain_zero, atom_id, call_info, trace);
    //
    if trace {
        println!("Begin Trace: sumsq_reverse_der_ad");
        print!("domain_zero = [ ");
        for j in 0 .. n_domain {
                print!("{}, ", domain_zero[j]);
        }
        println!("]");
        println!("range_one = [ {} ]", range_one[0]);
        print!("domain_one = [ ");
        for j in 0 .. n_domain {
                print!("{}, ", domain_one[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_der_ad");
    }
    domain_one
}
