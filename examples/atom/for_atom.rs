// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
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
    for_domain_zero : &Vec<&V>  ,
    _call_info      : IndexT    ,
    _trace           : bool      ,
) -> Vec<V>
{   //
    // n_domain
    let n_domain = for_domain_zero.len() / 2;
    assert_eq!( 2 * n_domain , for_domain_zero.len() );
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // domain_zero, domain_one
    let domain_zero =  &for_domain_zero[0 .. n_domain];
    let domain_one  =  &for_domain_zero[n_domain .. 2 * n_domain];
    //
    // for_range_zero
    let mut for_range_zero = 0.0 as V;
    for j in 0 .. n_domain {
        for_range_zero += &two_v * &( domain_zero[j] * domain_one[j] );
    }
    //
    vec![ for_range_zero ]
}
//
// for_sumsq_forward_zero_ad
pub fn for_sumsq_forward_zero_ad(
    _for_domain_zero  : &Vec<& AD<V> >    ,
    _call_info        : IndexT            ,
    _trace            : bool              ,
) -> Vec< AD<V> >
{   panic!("for_sumsq_forward_zero_ad: not implemented");
}
//
// for_sumsq_forward_one_value
fn for_sumsq_forward_one_value(
    for_domain_zero : &Vec<&V>  ,
    for_domain_one  : Vec<&V>   ,
    _call_info      : IndexT    ,
    _trace           : bool      ,
) -> Vec<V>
{   //
    // for_domain_zero
    assert_eq!( for_domain_zero.len(), for_domain_one.len() );
    //
    // n_domain
    let n_domain = for_domain_zero.len() / 2;
    assert_eq!( 2 * n_domain , for_domain_zero.len() );
    //
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // domain_zero, domain_one
    let domain_zero =  &for_domain_zero[0 .. n_domain];
    let domain_one  =  &for_domain_zero[n_domain .. 2 * n_domain];
    //
    // domain_one_zero, domain_one_one
    let domain_zero_one =  &for_domain_one[0 .. n_domain];
    let domain_one_one  =  &for_domain_one[n_domain .. 2 * n_domain];
    //
    // rev_range_one
    let mut rev_range_one = 0.0 as V;
    for j in 0 .. n_domain {
        rev_range_one += &two_v * &( domain_zero[j] * domain_one_one[j] );
        rev_range_one += &two_v * &( domain_zero_one[j] * domain_one[j] );
    }
    //
    vec![ rev_range_one ]
}
//
// for_sumsq_forward_one_ad
pub fn for_sumsq_forward_one_ad(
    _for_domain_zero  : &Vec<& AD<V> >    ,
    _for_domain_one   : Vec<& AD<V> >     ,
    _call_info        : IndexT            ,
    _trace            : bool              ,
) -> Vec< AD<V> >
{   panic!("for_sumsq_forward_one_ad: not implemented");
}
//
// for_sumsq_reverse_one_value
fn for_sumsq_reverse_one_value(
    for_domain_zero : &Vec<&V>  ,
    for_range_one   : Vec<&V>   ,
    _call_info      : IndexT    ,
    _trace          : bool      ,
) -> Vec<V>
{   //
    // for_range_one
    assert_eq!( for_range_one.len(), 1 );
    //
    // n_domain
    let n_domain = for_domain_zero.len() / 2;
    assert_eq!( 2 * n_domain , for_domain_zero.len() );
    //
    //
    // factor
    let factor : V = (2.0 as V) * for_range_one[0];
    //
    // domain_zero, domain_one
    let domain_zero =  &for_domain_zero[0 .. n_domain];
    let domain_one  =  &for_domain_zero[n_domain .. 2 * n_domain];
    //
    // for_domain_one
    let mut for_domain_one : Vec<V> = Vec::with_capacity( 2 * n_domain );
    for j in 0 .. n_domain {
        for_domain_one.push(&factor * domain_one[j]);
    }
    for j in 0 .. n_domain {
        for_domain_one.push(&factor * domain_zero[j]);
    }
    //
    for_domain_one
}
//
// for_sumsq_reverse_one_ad
pub fn for_sumsq_reverse_one_ad(
    _for_domain_zero  : &Vec<& AD<V> >    ,
    _for_range_one    : Vec<& AD<V> >     ,
    _call_info        : IndexT            ,
    _trace            : bool              ,
) -> Vec< AD<V> >
{   panic!("for_sumsq_reverse_one_ad: not implemented");
}
//
// for_sumsq_forward_depend
fn for_sumsq_forward_depend(
    is_var_for_domain  : &Vec<bool> ,
    _call_info         : IndexT     ,
    _trace             : bool       ,
) -> Vec<bool>
{
    let mut is_var_for_range = false;
    for j in 0 .. is_var_for_domain.len() {
        is_var_for_range = is_var_for_range || is_var_for_domain[j];
    }
    vec![ is_var_for_range ]
}
//
// register_for_sumsq_atom
pub fn register_for_sumsq_atom()-> IndexT {
    //
    // for_sumsq_atom_eval
    let for_sumsq_atom_eval = AtomEval {
        forward_zero_value   :  for_sumsq_forward_zero_value,
        forward_zero_ad      :  for_sumsq_forward_zero_ad,
        //
        forward_one_value    :  for_sumsq_forward_one_value,
        forward_one_ad       :  for_sumsq_forward_one_ad,
        //
        reverse_one_value    :  for_sumsq_reverse_one_value,
        reverse_one_ad       :  for_sumsq_reverse_one_ad,
        //
        forward_depend       :  for_sumsq_forward_depend,
    };
    //
    // for_sumsq_atom_id
    let for_sumsq_atom_id = register_atom( for_sumsq_atom_eval );
    for_sumsq_atom_id
}
