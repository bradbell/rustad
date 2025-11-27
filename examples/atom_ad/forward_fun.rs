// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
/*
sumsq_forward_fun
y = g(x) = x[0] * x[0] + x[1] * x[1] + ...
*/
use rustad::{
    AD,
    IndexT,
    call_atom,
};
//
// V, ATOM_ID_VEC
use super::{
    V,
    ATOM_ID_VEC
};
//
// sumsq_forward_fun_value
pub fn sumsq_forward_fun_value(
    domain       : &Vec<&V>    ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{   //
    // var_both, sumsq_zero
    let mut sumsq_zero = V::from(0.0);
    for j in 0 .. domain.len() {
        sumsq_zero += &( domain[j] * domain[j] );
    }
    if trace {
        println!("Begin Trace: sumsq_forward_fun_value");
        print!("domain      = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("sumsq_zero = {}", sumsq_zero);
        println!("End Trace: sumsq_forward_fun_value");
    }
    Ok( vec![ sumsq_zero ] )
}
//
// sumsq_forward_fun_ad
pub fn sumsq_forward_fun_ad(
    domain       : &Vec<& AD<V> >    ,
    call_info    : IndexT            ,
    trace        : bool              ,
) -> Result< Vec< AD<V> >, String >
{   //
    // atom_id
    let atom_id = ATOM_ID_VEC.with_borrow( |atom_id_vec|
        atom_id_vec[3 * (call_info as usize) + 0]
    );
    //
    // n_domain
    let n_domain = domain.len();
    //
    // domain_clone
    let mut domain_clone : Vec< AD<V> > = Vec::with_capacity( n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // sumsq_zero
    let sumsq_zero = call_atom(domain_clone, atom_id, call_info, trace);
    //
    if trace {
        println!("Begin Trace: sumsq_forward_fun_value");
        print!("domain      = [ ");
        for j in 0 .. n_domain {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("sumsq_zero = {:?}", sumsq_zero);
        println!("End Trace: sumsq_forward_fun_value");
    }
    Ok( sumsq_zero )
}
