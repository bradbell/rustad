// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// ---------------------------------------------------------------------------
// Example of doing checkpointing using atomic functions.
//
// TODO: convert this example in to a general purpose checkpoint utility.
// ---------------------------------------------------------------------------
use std::cell::RefCell;
//
use rustad::{
    AD,
    ADType,
    ad_from_value,
    ADfn,
    start_recording,
    stop_recording,
    register_atom,
    call_atom,
    AtomEval,
    IndexT,
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
// checkpoint_forward_fun_value
fn checkpoint_forward_fun_value(
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
    let mut var_both  : Vec<V> = Vec::new();
    let range_zero    = ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       let range_zero = f.forward_zero_value(
            &mut var_both, domain_zero_clone, trace
        );
       range_zero
    } );
    range_zero
}
//
// checkpoint_forward_der_value
fn checkpoint_forward_der_value(
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
    // var_both
    let mut var_both  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       f.forward_zero_value(&mut var_both, domain_zero_clone, trace);
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
       range_one = f.forward_one_value(&var_both, domain_one_clone, trace);
    } );
    range_one
}
//
// checkpoint_reverse_der_value
fn checkpoint_reverse_der_value(
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
    // var_both
    let mut var_both  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       f.forward_zero_value(&mut var_both, domain_zero_clone, trace);
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
       domain_one = f.reverse_one_value(&var_both, range_one_clone, trace);
    } );
    domain_one
}
//
// checkpoint_forward_type
fn checkpoint_forward_type(
    domain_ad_type : &[ADType]    ,
    call_info      : IndexT       ,
    trace          : bool         ,
) -> Vec<ADType>
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
    let mut range_ad_type = vec![ADType::ConstantP; call_n_res];
    for [i,j] in dependency {
        range_ad_type[i] = std::cmp::max(
            range_ad_type[i].clone(), domain_ad_type[j].clone()
        );
    }
    range_ad_type
}
//
// -------------------------------------------------------------------------
// register_checkpoint_atom
// -------------------------------------------------------------------------
fn register_checkpoint_atom()-> IndexT {
    //
    // checkpoint_atom_eval
    let checkpoint_atom_eval = AtomEval {
        name                 : &"checkpoint",
        forward_type         :  checkpoint_forward_type,
        //
        forward_fun_value    :  Some(checkpoint_forward_fun_value),
        forward_fun_ad       :  None,
        //
        forward_der_value    :  Some(checkpoint_forward_der_value),
        forward_der_ad       :  None,
        //
        reverse_der_value    :  Some(checkpoint_reverse_der_value),
        reverse_der_ad       :  None,
    };
    //
    // atom_id
    let atom_id = register_atom( checkpoint_atom_eval );
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
