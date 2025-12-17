// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
// use
use rustad::{
    IndexT,
    AtomCallback,
    register_atom,
    AtomInfoVecPublic,
};
//
// forward_fun_value
fn forward_fun_value<V>(
    domain     : &Vec<&V>  ,
    _call_info : IndexT    ,
    _trace      : bool     ,
) -> Result< Vec<V>, String >
where
    V : Clone + AtomInfoVecPublic ,
{   // range
    let mut range : Vec<V> = Vec::with_capacity( domain.len() );
    for i in 0 .. domain.len() {
        range.push( domain[i].clone() );
    }
    Ok(range)
}
//
// rev_depend
fn rev_depend<V>(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String
{   assert_eq!( depend.len(), 0 );
    assert!( rng_index < n_dom );
    depend.push(rng_index);
    //
    let error_msg = String::new();
    error_msg
}
//
// register_eye
pub fn register_eye<V>() -> IndexT
where
    V : Clone + AtomInfoVecPublic ,
{
    //
    // callback
    let callback = AtomCallback{
        name               : &"eye",
        rev_depend         : Some(rev_depend::<V>),
        forward_fun_value  : Some(forward_fun_value::<V>) ,
        //
        forward_fun_ad     : None,
        forward_der_value  : None,
        forward_der_ad     : None,
        reverse_der_value  : None,
        reverse_der_ad     : None,
    };
    // atom_id
    let atom_id = register_atom( callback );
    atom_id
}
