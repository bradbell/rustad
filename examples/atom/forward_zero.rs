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
// V
use super::V;
//
// sumsq_forward_zero_value
pub fn sumsq_forward_zero_value(
    var_zero     : &mut Vec<V> ,
    domain_zero  : Vec<&V>     ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Vec<V>
{   //
    // var_zero, sumsq_zero
    assert_eq!( var_zero.len(), 0 );
    let mut sumsq_zero = 0 as V;
    for j in 0 .. domain_zero.len() {
        sumsq_zero += &( domain_zero[j] * domain_zero[j] );
        var_zero.push( *domain_zero[j] )
    }
    if trace {
        println!("Begin Trace: sumsq_forward_zero_value");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        println!("]");
        println!("sumsq_zero = {}", sumsq_zero);
        println!("End Trace: sumsq_forward_zero_value");
    }
    vec![ sumsq_zero ]
}
//
// sumsq_forward_zero_ad
pub fn sumsq_forward_zero_ad(
    var_zero     : &mut Vec< AD<V> > ,
    domain_zero  : Vec<& AD<V> >     ,
    _call_info   : IndexT            ,
    trace        : bool              ,
) -> Vec< AD<V> >
{   //
    // var_zero, sumsq_zero
    assert_eq!( var_zero.len(), 0 );
    let mut sumsq_zero = ad_from_value( 0 as V );
    for j in 0 .. domain_zero.len() {
        sumsq_zero += &( domain_zero[j] * domain_zero[j] );
        var_zero.push( (*domain_zero[j]).clone() )
    }
    if trace {
        println!("Begin Trace: sumsq_forward_zero_value");
        print!("domain_zero = [ ");
        for j in 0 .. domain_zero.len() {
                print!("{}, ", domain_zero[j]);
        }
        println!("]");
        println!("sumsq_zero = {}", sumsq_zero);
        println!("End Trace: sumsq_forward_zero_value");
    }
    vec![ sumsq_zero ]
}
