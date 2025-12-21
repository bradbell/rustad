// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
/*
Test rust_src with an atomic function and dynamic parameters.

The atomic function will be
       [ z[0] * z[1] ]
h(z) = [ z[1] * z[2] ]
       [    z[3]     ]
Note that the domain (range) of h has size 4 (3).

This test has 2 dyanimic parameters p[0], p[1], and one variable x[0].

We define the following function using h:
          [ p[0] * p[1] ]
f(p, x) = [ p[1] * x[0] ]
          [      5      ]
*/
use rustad::{
    AzFloat,
    AD,
    ad_from_value,
    register_atom,
    AtomCallback,
    IndexT,
    start_recording_dyp_var,
    stop_recording,
    call_atom,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
};
//
// V
type V = AzFloat<f64>;
//
// V_STR
const V_STR : &str = "AzFloat<f64>";
//
// h_rev_depend
fn h_rev_depend(
    depend       : &mut Vec<usize> ,
    rng_index    : usize           ,
    n_dom        : usize           ,
    _call_info   : IndexT          ,
    _trace       : bool            ,
) -> String {
    assert_eq!( depend.len(), 0 );
    assert_eq!( n_dom, 4);
    let mut error_msg = String::new();
    match rng_index {
        0 => { depend.push(0); depend.push(1) },
        1 => { depend.push(1); depend.push(2) },
        2 => { depend.push(3); },
        _ => { error_msg += "h_depend: invalid range index"; },
    }
    error_msg
}
//
// BEGIN h_forward_fun_value
pub fn h_forward_fun_value(
    _use_range   : &[bool]     ,
    domain       : &[&V]       ,
    _call_info   : IndexT      ,
    trace        : bool        ,
) -> Result< Vec<V>, String >
{
    // range
    let mut range : Vec<V> = Vec::new();
    range.push( domain[0] * domain[1] );
    range.push( domain[1] * domain[2] );
    range.push( domain[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_fun_value");
        print!("domain = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("range = {:?}", range);
        println!("End Trace: h_forward_fun_value");
    }
    Ok( range )
}
// END h_forward_fun_value
//
// h_forward_fun_ad
pub fn h_forward_fun_ad(
    domain       : &[& AD<V>]        ,
    _call_info   : IndexT            ,
    trace        : bool              ,
) -> Result< Vec< AD<V> >, String >
{
    // range
    let mut range : Vec< AD<V> > = Vec::new();
    range.push( domain[0] * domain[1] );
    range.push( domain[1] * domain[2] );
    range.push( domain[3].clone() );
    //
    if trace {
        println!("Begin Trace: h_forward_fun_ad");
        print!("domain = [ ");
        for j in 0 .. domain.len() {
                print!("{}, ", domain[j]);
        }
        println!("]");
        println!("range = {:?}", range);
        println!("End Trace: h_forward_fun_ad");
    }
    Ok( range )
}
//
// register_h
fn register_h()-> IndexT {
    //
    // h_callback
    let h_callback = AtomCallback {
        name                 : &"h",
        rev_depend           :  Some( h_rev_depend) ,
        //
        forward_fun_value    :  Some( h_forward_fun_value ),
        forward_fun_ad       :  Some( h_forward_fun_ad ),
        //
        forward_der_value    :  None,
        forward_der_ad       :  None,
        //
        reverse_der_value    :  None,
        reverse_der_ad       :  None,
    };
    //
    // h__atom_id
    let h_atom_id = register_atom( h_callback );
    h_atom_id
}
//
// build_atom_src
fn build_atom_src() -> String {
    //
    // atom_name
    let atom_name = "h";
    //
    // v_str
    let v_str = V_STR;
    //
    // i_str
    let i_str = std::any::type_name::<IndexT>();
    //
    // this_src
    let this_file = file!();
    let this_src  = std::fs::read_to_string(this_file).unwrap();
    //
    // begin_comment, end_comment
    // use concat so that this text does not match during the find below
    let begin_comment = String::new() +
        "// BEGIN " + &atom_name + "_forward_fun_value\n";
    let end_comment = String::new() +
        "// END " + &atom_name + "_forward_fun_value\n";
    //
    // atom_src
    let start      = this_src.find(&begin_comment).unwrap();
    let end        = this_src.find(&end_comment).unwrap();
    let atom_src   = String::from( &this_src[start .. end] );
    let atom_src   = atom_src.replace( &begin_comment, "//\n");
    //
    // atom_src
    let old_name   = String::new() + &atom_name + "_forward_fun_value";
    let new_name   = String::new() + "atom_" + &atom_name;
    let atom_src   = atom_src.replace(&old_name, &new_name);
    //
    // atom_src
        let atom_src = atom_src.replace("pub fn", "fn");
    let atom_src = atom_src.replace("IndexT", i_str);
    //
    // atom_src
    let atom_src = atom_src.replace("<V>", "<v_str>");
    let atom_src = atom_src.replace("[&V]", "[&v_str]");
    let atom_src = atom_src.replace("as V", "as v_str");
    let atom_src = atom_src.replace("v_str", v_str);
    //
    atom_src
}
//
// atom_dyp_src
#[test]
fn atom_dyp_src() {
    let h_atom_id  = register_h();
    let call_info      = 0;
    let trace          = false;
    //
    // f
    let p   : Vec<V> = vec![ V::from(1.0); 2];
    let x   : Vec<V> = vec![ V::from(1.0); 1];
    let (ap, ax)     = start_recording_dyp_var(p, x);
    let z0           = ap[0].clone();
    let z1           = ap[1].clone();
    let z2           = ax[0].clone();
    let z3           = ad_from_value( V::from(5.0) );
    let az           = vec![ z0, z1, z2, z3 ];
    let ny           = 3;
    let ay           = call_atom(ny, az, h_atom_id, call_info, trace);
    let f            = stop_recording(ay);
    //
    let p   : Vec<V> = vec![ V::from(2.0), V::from(3.0) ];
    let x   : Vec<V> = vec![ V::from(4.0) ];
    let q            = f.forward_dyp_value(p.clone(), trace);
    let (y, _v)      = f.forward_var_value(&q, x.clone(), trace);
    //
    // check h_forward_fun_value
    assert_eq!( y.len(), 3 );
    assert_eq!( y[0], p[0] * p[1] );
    assert_eq!( y[1], p[1] * x[0] );
    assert_eq!( y[2], V::from(5.0) );
    //
    // f
    let p   : Vec<V> = vec![ V::from(1.0); 2];
    let x   : Vec<V> = vec![ V::from(1.0); 1];
    let (ap, ax)     = start_recording_dyp_var(p.clone(), x.clone());
    let aq           = f.forward_dyp_ad(ap, trace);
    let (ay, _av)    = f.forward_var_ad(&aq, ax, trace);
    let g            = stop_recording(ay);
    //
    let p   : Vec<V> = vec![ V::from(2.0), V::from(3.0) ];
    let x   : Vec<V> = vec![ V::from(4.0) ];
    let q            = g.forward_dyp_value(p.clone(), trace);
    let (y, _v)      = g.forward_var_value(&q, x.clone(), trace);
    //
    // check h_forward_fun_ad
    assert_eq!( y.len(), 3 );
    assert_eq!( y[0], p[0] * p[1] );
    assert_eq!( y[1], p[1] * x[0] );
    assert_eq!( y[2], V::from(5.0) );
    //
    // az_float_src
    let az_float_src = String::from( rustad::AZ_FLOAT_SRC );
    //
    // atom_src
    let atom_src  = build_atom_src();
    //
    // rust_src
    let fn_name   = "h";
    let rust_src  = f.rust_src(fn_name);
    //
    // src_file
    let src_file  = "tmp/test_atom_dyp_src.rs";
    let src       = az_float_src + &atom_src + &rust_src;
    let result    = std::fs::write(src_file, src);
    if result.is_err() {
        panic!( "Cannot write {src_file}"  );
    }
    //
    // lib
    let lib_file    = "tmp/test_atom_dyp_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_file, lib_file, replace_lib);
    //
    // h_fn
    let h_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &fn_name);
    //
    // p_ref, x_ref
    let mut p_ref : Vec<&V> = Vec::new();
    for p_j in p.iter() {
        p_ref.push( &p_j );
    }
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( &x_j )
    }
    //
    // check result
    let result = h_fn(&p_ref, &x_ref);
    let y      = result.unwrap();
    //
    // check rust_src
    assert_eq!( y.len(), 3 );
    assert_eq!( y[0], p[0] * p[1] );
    assert_eq!( y[1], p[1] * x[0] );
    assert_eq!( y[2], V::from(5.0) );
}
