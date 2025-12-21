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
    AzFloat,
    AD,
    ad_from_value,
    ADfn,
    start_recording_var,
    stop_recording,
    register_atom,
    call_atom,
    AtomCallback,
    IndexT,
};
//
// V
type V = AzFloat<f64>;
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
    _use_range       : &[bool]      ,
    domain           : &[&V]        ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
{   //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // range
    let range         = ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       let (range, _) = f.forward_zero_value(
            domain_clone, trace
        );
       range
    } );
    Ok( range )
}
//
// checkpoint_forward_der_value
fn checkpoint_forward_der_value(
    _use_range       : &[bool]     ,
    domain           : &[&V]       ,
    domain_der       : &[&V]       ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
{   //
    assert_eq!( domain.len(), domain_der.len() );
    //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // var_both
    let mut var_both  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f         = &f_vec[call_info as usize];
       (_, var_both) = f.forward_zero_value(domain_clone, trace);
    } );
    //
    // domain_der
    let mut domain_der_clone : Vec<V> = Vec::with_capacity( domain_der.len() );
    for j in 0 .. domain_der.len() {
        domain_der_clone.push( (*domain_der[j]).clone() );
    }
    //
    // range_der
    let mut range_der : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f     = &f_vec[call_info as usize];
       range_der = f.forward_one_value(&var_both, domain_der_clone, trace);
    } );
    Ok( range_der )
}
//
// checkpoint_reverse_der_value
fn checkpoint_reverse_der_value(
    domain           : &[&V]       ,
    range_der        : Vec<&V>     ,
    call_info        : IndexT      ,
    trace            : bool        ,
) -> Result< Vec<V>, String >
{   //
    // domain_clone
    let n_domain = domain.len();
    let mut domain_clone : Vec<V> = Vec::with_capacity(n_domain);
    for j in 0 .. n_domain {
        domain_clone.push( (*domain[j]).clone() );
    }
    //
    // var_both
    let mut var_both  : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f          = &f_vec[call_info as usize];
       (_, var_both)  = f.forward_zero_value(domain_clone, trace);
    } );
    //
    // range_der_clone
    let mut range_der_clone : Vec<V> = Vec::with_capacity( range_der.len() );
    for j in 0 .. range_der.len() {
        range_der_clone.push( (*range_der[j]).clone() );
    }
    //
    // domain_der
    let mut domain_der : Vec<V> = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f      = &f_vec[call_info as usize];
       domain_der = f.reverse_one_value(&var_both, range_der_clone, trace);
    } );
    Ok( domain_der )
}
//
// checkpoint_rev_depend
fn checkpoint_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    _n_dom       : usize           ,
    call_info    : IndexT          ,
    trace        : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    //
    // pattern
    // TODO: store the sparsity pattern in a static structure for this
    // checkpoint function so do not need to recompute. Also sort it so it
    // and store point to beginning of each row so depend computes faster.
    let mut pattern : Vec< [usize; 2] > = Vec::new();
    ADFN_VEC.with_borrow( |f_vec| {
       let f           = &f_vec[call_info as usize];
       let compute_dyp = false;
       (_, pattern)    = f.sub_sparsity(trace, compute_dyp);
    } );
    //
    // depend
    for [i, j] in pattern.iter() {
        if *i == rng_index {
            depend.push( *j );
        }
    }
    let error_msg = String::from("");
    error_msg
}
//
// -------------------------------------------------------------------------
// register_checkpoint_atom
// -------------------------------------------------------------------------
fn register_checkpoint_atom()-> IndexT {
    //
    // checkpoint_callback
    let checkpoint_callback = AtomCallback {
        name                 : &"checkpoint",
        rev_depend           :  Some( checkpoint_rev_depend ),
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
    let atom_id = register_atom( checkpoint_callback );
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
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let ax           = start_recording_var(x);
    let mut asumsq : AD<V> = ad_from_value( V::from(0) );
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
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let ax           = start_recording_var(x);
    let ny           = 1;
    let ay           = call_atom(ny, ax, atom_id, call_info, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_zero_value
    let x       : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, v)           = g.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_one_value
    let dx      : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let dy               = g.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * x[0]*dx[0] + V::from(2.0) * x[1]*dx[1] );
    //
    // g.reverse_one_value
    let dy      : Vec<V> = vec![ V::from(5.0) ];
    let dx               = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0]*dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1]*dy[0] );
}
