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
// rev_sumsq_forward_zero_value
fn rev_sumsq_forward_zero_value(
    rev_domain_zero : &Vec<&V>  ,
    _call_info      : IndexT    ,
    _trace           : bool      ,
) -> Vec<V>
{   //
    // n_domain
    assert!( rev_domain_zero.len() > 1 );
    let n_domain = rev_domain_zero.len() - 1;
    //
    // domain_zero, range_one
    let domain_zero =  &rev_domain_zero[0 .. n_domain];
    let range_one   =  &rev_domain_zero[n_domain .. n_domain + 1];
    //
    // two_v
    let two_v : V = 2.0 as V;
    //
    // rev_domain_zero
    let mut rev_domain_zero : Vec<V> = Vec::with_capacity( n_domain );
    for j in 0 .. n_domain {
        rev_domain_zero.push( &two_v * &( range_one[0] * domain_zero[j] ) );
    }
    //
    rev_domain_zero
}
//
// rev_sumsq_forward_zero_ad
pub fn rev_sumsq_forward_zero_ad(
    _rev_domain_zero  : &Vec<& AD<V> >    ,
    _call_info        : IndexT            ,
    _trace            : bool              ,
) -> Vec< AD<V> >
{   panic!("rev_sumsq_forward_zero_ad: not implemented");
}
//
// rev_sumsq_forward_one_value
fn rev_sumsq_forward_one_value(
    rev_domain_zero : &Vec<&V>  ,
    rev_domain_one  : Vec<&V>   ,
    _call_info      : IndexT    ,
    _trace           : bool      ,
) -> Vec<V>
{   //
    // n_domain
    assert!( rev_domain_zero.len() > 1 );
    let n_domain = rev_domain_zero.len() - 1;
    //
    // domain_zero, range_one
    let domain_zero =  &rev_domain_zero[0 .. n_domain];
    let range_one   =  &rev_domain_zero[n_domain .. n_domain + 1];
    //
    // two_v
    let two_v   : V = 2.0 as V;
    //
    // domain_one_zero, domain_one_one
    let domain_zero_one =  &rev_domain_one[0 .. n_domain];
    let range_one_one   =  &rev_domain_one[n_domain .. n_domain + 1];
    //
    // rev_range_one
    let mut rev_range_one : Vec<V> = Vec::with_capacity( n_domain );
    for j in 0 .. n_domain {
        let mut term_j  = &two_v * &( range_one[0] * domain_zero_one[j] );
        term_j         += &two_v * &( range_one_one[0] * domain_zero[j] );
        rev_range_one.push( term_j );
    }
    //
    rev_range_one
}
//
// rev_sumsq_forward_one_ad
pub fn rev_sumsq_forward_one_ad(
    _rev_domain_zero  : &Vec<& AD<V> >    ,
    _rev_domain_one   : Vec<& AD<V> >     ,
    _call_info   : IndexT                 ,
    _trace        : bool                  ,
) -> Vec< AD<V> >
{   panic!("rev_sumsq_forward_one_ad: not implemented");
}
//
// rev_sumsq_reverse_one_value
fn rev_sumsq_reverse_one_value(
    rev_domain_zero : &Vec<&V>  ,
    rev_range_one   : Vec<&V>   ,
    _call_info      : IndexT    ,
    _trace          : bool      ,
) -> Vec<V>
{   //
    // n_domain
    assert!( rev_domain_zero.len() > 1 );
    let n_domain = rev_domain_zero.len() - 1;
    //
    // domain_zero, range_one
    let domain_zero =  &rev_domain_zero[0 .. n_domain];
    let range_one   =  &rev_domain_zero[n_domain .. n_domain + 1];
    //
    // two_v
    let two_v   : V = 2.0 as V;
    //
    // domain_zero_one
    let domain_zero_one = &rev_range_one[0 .. n_domain];
    let range_one_one   = rev_range_one[n_domain];
    //
    // rev_domain_one
    let mut rev_domain_one : Vec<V> = Vec::with_capacity( n_domain + 1);
    let mut rev_range_two           = 0.0 as V;
    for j in 0 .. n_domain {
        rev_domain_one.push( &two_v * &( range_one[0] * domain_zero_one[j] ) );
        rev_range_two  += &two_v * ( range_one_one * domain_zero[j] );
    }
    rev_domain_one.push( rev_range_two );
    //
    rev_domain_one
}
//
// rev_sumsq_reverse_one_ad
pub fn rev_sumsq_reverse_one_ad(
    _rev_domain_zero  : &Vec<& AD<V> >    ,
    _rev_range_one    : Vec<& AD<V> >     ,
    _call_info        : IndexT            ,
    _trace            : bool              ,
) -> Vec< AD<V> >
{   panic!("rev_sumsq_reverse_one_ad: not implemented");
}
//
// rev_sumsq_forward_depend
fn rev_sumsq_forward_depend(
    is_var_rev_domain  : &Vec<bool> ,
    _call_info         : IndexT     ,
    _trace             : bool       ,
) -> Vec<bool>
{
    let mut is_var_for_range = false;
    for j in 0 .. is_var_rev_domain.len() {
        is_var_for_range = is_var_for_range || is_var_rev_domain[j];
    }
    vec![ is_var_for_range ]
}
//
// register_rev_sumsq_atom
pub fn register_rev_sumsq_atom()-> IndexT {
    //
    // rev_sumsq_atom_eval
    let rev_sumsq_atom_eval = AtomEval {
        forward_zero_value   :  rev_sumsq_forward_zero_value,
        forward_zero_ad      :  rev_sumsq_forward_zero_ad,
        //
        forward_one_value    :  rev_sumsq_forward_one_value,
        forward_one_ad       :  rev_sumsq_forward_one_ad,
        //
        reverse_one_value    :  rev_sumsq_reverse_one_value,
        reverse_one_ad       :  rev_sumsq_reverse_one_ad,
        //
        forward_depend       :  rev_sumsq_forward_depend,
    };
    //
    // rev_sumsq_atom_id
    let rev_sumsq_atom_id = register_atom( rev_sumsq_atom_eval );
    rev_sumsq_atom_id
}
