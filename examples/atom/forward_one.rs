// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AD,
    IndexT,
    ad_from_value,
};
//
// V, ATOM_ID_VEC
use super::{
    V,
};
//
// sumsq_forward_one_value
pub fn sumsq_forward_one_value(
    domain_zero  : &Vec<&V>    ,
    domain_one   : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // domain_zero
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // two_v
    let two_v = 2.0 as V;
    //
    // range_one
    let mut range_one = 0.0 as V;
    for j in 0 .. domain_one.len() {
        range_one += &( &two_v * &( domain_zero[j] * domain_one[j] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_one_value");
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
        println!("End Trace: sumsq_forward_one_value");
    }
    vec![ range_one ]
}
//
// sumsq_forward_one_ad
pub fn sumsq_forward_one_ad(
    domain_zero  : &Vec<& AD<V> >    ,
    domain_one   : Vec<& AD<V> >     ,
    _call_info   : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{   //
    // domain_zero
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // two_ad
    let two_ad = ad_from_value(2.0 as V);
    //
    // range_one
    let mut range_one = ad_from_value(0.0 as V);
    for j in 0 .. domain_one.len() {
        range_one += &( &two_ad * &( domain_zero[j] * domain_one[j] ) );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_one_ad");
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
        println!("End Trace: sumsq_forward_one_ad");
    }
    vec![ range_one ]
}
