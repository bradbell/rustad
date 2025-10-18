// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// ---------------------------------------------------------------------------
// TODO: convert this example in to a general purpose checkpoint utility.
// ---------------------------------------------------------------------------
use std::cell::RefCell;
//
use rustad::{
    AD,
    ad_from_value,
    ADfn,
    start_recording,
    stop_recording,
    register_atom,
    call_atom,
    AtomEval,
    IndexT,
    default_atom_callbacks,
    default_atom_eval,
};
//
// V
type V = f64;
//
thread_local! {
    static ADFN_VEC : RefCell< Vec< ADfn<V> > > =
        RefCell::new( Vec::new() );
}
// -------------------------------------------------------------------------
// Value Routines
// -------------------------------------------------------------------------
//
// checkpoint_forward_zero_value
fn checkpoint_forward_zero_value(
    domain_zero      : &Vec<&V>     ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Vec<V>
{   //
    // domain_zero_clone
    let n_domain = domain_zero.len();
    let mut domain_zero_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_zero_clone.push( (*domain_zero[j]).clone() );
    }
    //
    // range_zero
    let mut var_zero  : Vec<V> = Vec::new();
    let range_zero    = ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       let range_zero = f.forward_zero_value(
            &mut var_zero, domain_zero_clone, trace
        );
       range_zero
    } );
    range_zero
}
//
// checkpoint_forward_one_value
fn checkpoint_forward_one_value(
    domain_zero      : &Vec<&V>    ,
    domain_one       : Vec<&V>     ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Vec<V>
{   //
    assert_eq!( domain_zero.len(), domain_one.len() );
    //
    // domain_zero_clone
    let n_domain = domain_zero.len();
    let mut domain_zero_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_zero_clone.push( (*domain_zero[j]).clone() );
    }
    //
    // var_zero
    let mut var_zero  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       f.forward_zero_value(&mut var_zero, domain_zero_clone, trace);
    } );
    //
    // domain_one
    let mut domain_one_clone : Vec<V> = Vec::with_capacity( domain_one.len() );
    for j in 0 .. domain_one.len() {
        domain_one_clone.push( (*domain_one[j]).clone() );
    }
    //
    // range_one
    let mut range_one : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f     = &f_vec[call_info as usize];
       range_one = f.forward_one_value(&var_zero, domain_one_clone, trace);
    } );
    range_one
}
//
// checkpoint_reverse_one_value
fn checkpoint_reverse_one_value(
    domain_zero      : &Vec<&V>    ,
    range_one        : Vec<&V>     ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Vec<V>
{   //
    // domain_zero_clone
    let n_domain = domain_zero.len();
    let mut domain_zero_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_zero_clone.push( (*domain_zero[j]).clone() );
    }
    //
    // var_zero
    let mut var_zero  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       f.forward_zero_value(&mut var_zero, domain_zero_clone, trace);
    } );
    //
    // range_one_clone
    let mut range_one_clone : Vec<V> = Vec::with_capacity( range_one.len() );
    for j in 0 .. range_one.len() {
        range_one_clone.push( (*range_one[j]).clone() );
    }
    //
    // domain_one
    let mut domain_one : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f      = &f_vec[call_info as usize];
       domain_one = f.reverse_one_value(&var_zero, range_one_clone, trace);
    } );
    domain_one
}
//
// checkpoint_forward_depend
fn checkpoint_forward_depend(
    is_var_domain  : &Vec<bool> ,
    call_info      : IndexT     ,
    trace          : bool       ,
) -> Vec<bool>
{   //
    // dependency
    let mut dependency : Vec< [usize; 2] > = Vec::new();
    let mut call_n_res : usize             = 0;
    ADFN_VEC.with_borrow( |f_vec| {
       let f       = &f_vec[call_info as usize];
       dependency = f.sub_sparsity(trace);
       call_n_res = f.range_len();
    } );
    //
    // is_var_range
    let mut is_var_range = vec![false; call_n_res];
    for [i,j] in dependency {
        if is_var_domain[j] {
            is_var_range[i] = true;
        }
    }
    is_var_range
}
//
// -------------------------------------------------------------------------
// register_checkpoint_atom
// -------------------------------------------------------------------------
fn register_checkpoint_atom()-> IndexT {
    //
    // atom_eval
    default_atom_callbacks!(checkpoint, V);
    let mut atom_eval = default_atom_eval!(checkpoint);
    atom_eval.forward_zero_value = checkpoint_forward_zero_value;
    atom_eval.forward_one_value  = checkpoint_forward_one_value;
    atom_eval.reverse_one_value  = checkpoint_reverse_one_value;
    atom_eval.forward_depend     = checkpoint_forward_depend;
    //
    // atom_id
    let atom_id = register_atom( atom_eval );
    atom_id
}
// -------------------------------------------------------------------------
// AD routines
// -------------------------------------------------------------------------
//
// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------
fn main() {
    //
    // trace
    let trace = false;
    //
    // atom_id
    let atom_id = register_checkpoint_atom();
    //
    // f
    let x   : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax           = start_recording(x);
    let mut asumsq : AD<V> = ad_from_value( 0 as V );
    for j in 0 .. ax.len() {
        let term = &ax[j] * &ax[j];
        asumsq  += &term;
    }
    let ay          = vec![ asumsq ];
    let f           = stop_recording(ay);
    //
    // call_info , ADFN_VEC
    let call_info  = ADFN_VEC.with_borrow_mut( |f_vec| {
            let index = f_vec.len() as IndexT;
            f_vec.push( f );
            index
    } );
    //
    // g
    let x   : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax           = start_recording(x);
    let ay           = call_atom(ax, atom_id, call_info, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_zero_value
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = g.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_one_value
    let dx      : Vec<V> = vec![ 5.0, 6.0 ];
    let dy               = g.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], 2.0 * x[0]*dx[0] + 2.0 * x[1]*dx[1] );
    //
    // g.reverse_one_value
    let dy      : Vec<V> = vec![ 5.0 ];
    let dx               = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], 2.0 * x[0]*dy[0] );
    assert_eq!( dx[1], 2.0 * x[1]*dy[0] );
}
