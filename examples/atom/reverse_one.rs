// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AD,
    IndexT,
};
//
// V, ATOM_ID_VEC
use super::{
    V,
};
//
// sumsq_reverse_one_value
pub fn sumsq_reverse_one_value(
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
        println!("Begin Trace: sumsq_reverse_one_value");
        println!("range_one = [ {} ]", range_one[0]);
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", range_one[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_one_value");
    }
    domain_one
}
//
// sumsq_reverse_one_ad
pub fn sumsq_reverse_one_ad(
    domain_zero  : &Vec<& AD<V> >    ,
    range_one    : Vec<& AD<V> >     ,
    _call_info   : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{   //
    // range_one
    assert_eq!( range_one.len(), 1 );
    //
    // two_v
    let two_v = 2.0 as V;
    //
    // domain_one
    let mut domain_one : Vec< AD<V> > = Vec::with_capacity( domain_zero.len() );
    for j in 0 .. domain_zero.len() {
        domain_one.push( &two_v * &( domain_zero[j] * range_one[0] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_reverse_one_ad");
        println!("range_one = [ {} ]", range_one[0]);
        print!("domain_one = [ ");
        for j in 0 .. domain_one.len() {
                print!("{}, ", range_one[j]);
        }
        println!("]");
        println!("End Trace: sumsq_reverse_one_ad");
    }
    domain_one
}
