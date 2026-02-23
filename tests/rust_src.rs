// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// Test generating rust source code.
//
use rustad::{
    AD,
    AzFloat,
    start_recording,
    stop_recording,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
    create_src_dir,
    FloatCore,
};
//
fn test_sub () {
    //
    type V     = AzFloat<f32>;
    //
    // p, x, ap, ax
    let p  = vec![ V::from(1.0), V::from(1.0) ];
    let x  = vec![ V::from(1.0), V::from(1.0) ];
    let (ap, ax)    = start_recording(Some(p), x.clone());
    //
    // ay
    let mut ay : Vec< AD<V> > = Vec::new();
    //
    // y[0] = p[0] - p[1]
    ay.push( &ap[0] - &ap[1] );
    //
    // y[1] = x[0] - x[1];
    ay.push( &ax[0] - &ax[1] );
    //
    // f
    // f(x) = y
    let f  = stop_recording(ay);
    //
    // lib_src
    let gn_name  = "test_sub";
    let lib_src  = f.rust_src(gn_name);
    //
    // src_dir
    let src_dir = "tmp/test_sub_rust_src";
    create_src_dir(src_dir, &lib_src);
    //
    // lib
    let lib_file    = "tmp/test_sub_rust_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_dir, lib_file, replace_lib);
    //
    // test_sub_fn
    let test_sub_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // p_ref, x_ref
    let p                   = vec! [ V::from(2.0), V::from(3.0) ];
    let mut p_ref : Vec<&V> = Vec::new();
    for p_j in p.iter() {
        p_ref.push( p_j );
    }
    let x                   =  vec! [ V::from(5.0), V::from(4.0) ];
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( x_j )
    }
    //
    // y
    let result = test_sub_fn(&p_ref, &x_ref);
    let y      = result.unwrap();
    //
    // check
    assert_eq!( y[0], p[0] - p[1] );
    assert_eq!( y[1], x[0] - x[1] );
}
//
fn test_unary () {
    //
    type V     = AzFloat<f32>;
    //
    // p, x, ap, ax
    let p  = vec![ V::from(1.0) ];
    let x  = vec![ V::from(1.0) ];
    let (ap, ax)    = start_recording(Some(p), x.clone());
    //
    // ay
    let mut ay : Vec< AD<V> > = Vec::new();
    //
    // y[0] = sin( p[0] )
    ay.push( ap[0].sin() );
    //
    // y[1] = abs( x[0] );
    ay.push( ax[0].abs() );
    //
    // f
    // f(x) = y
    let f  = stop_recording(ay);
    //
    // lib_src
    let gn_name  = "test_unary";
    let lib_src  = f.rust_src(gn_name);
    //
    // src_dir
    let src_dir = "tmp/test_unary_rust_src";
    create_src_dir(src_dir, &lib_src);
    //
    // lib
    let lib_file    = "tmp/test_unary_rust_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_dir, lib_file, replace_lib);
    //
    // test_unary_fn
    let test_unary_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // p_ref, x_ref
    let p                   = vec! [ V::from(2.0) ];
    let mut p_ref : Vec<&V> = Vec::new();
    for p_j in p.iter() {
        p_ref.push( p_j );
    }
    let x                   =  vec! [ V::from(-3.0) ];
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( x_j )
    }
    //
    // y
    let result = test_unary_fn(&p_ref, &x_ref);
    let y      = result.unwrap();
    //
    // check
    assert_eq!( y[0], p[0].sin() );
    assert_eq!( y[1], x[0].abs() );
}
//
#[test]
fn rust_src() {
    test_sub();
    test_unary();
}
