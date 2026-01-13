// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Example converting a derivative calculation to rust source code
//
use rustad::{
    AzFloat,
    start_recording,
    stop_recording,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
    ad_from_vector,
};
//
fn test_sub () {
    //
    //
    type V     = AzFloat<f32>;
    let nx     = 2;
    let trace  = false;
    //
    // ax
    let x  : Vec<V> = vec![ V::from(2.0); nx ];
    let (_, ax)  = start_recording(None, x);
    //
    // asub
    let asub = &ax[0] - &ax[1];
    //
    // f
    // f(x) = x[0] - x[1]
    let ay = vec![ asub ];
    let f  = stop_recording(ay);
    //
    // av
    let x  : Vec<V> = vec![ V::from(2.0); nx ];
    let (_, ax) = start_recording(None, x);
    let (_, av) = f.forward_var_ad(None, ax, trace);
    //
    // g
    // g(x) = df/dx = [ x[0], - x[1] ]
    let dy  : Vec<V>  = vec![ V::from(1.0) ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_der_ad(None, &av, ady, trace);
    let g             = stop_recording(adx);
    //
    // src
    let src      = String::from( rustad::AZ_FLOAT_SRC );
    let gn_name  = "sub_reverse_der";
    let src      = src + &g.rust_src(gn_name);
    //
    // src_file
    let src_file = "tmp/test_sub_rust_src.rs";
    let result = std::fs::write(src_file, src);
    if result.is_err() {
        panic!( "Cannot write {src_file}"  );
    }
    //
    // lib
    let lib_file    = "tmp/test_sub_rust_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_file, lib_file, replace_lib);
    //
    // sub_reverse_der_fn
    let sub_reverse_der_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // p_ref, x_ref
    let p_ref     : Vec<&V> = Vec::new();
    let x         : Vec<V>  = vec![ V::from(3.0); nx ];
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( &x_j )
    }
    //
    // check result
    let result = sub_reverse_der_fn(&p_ref, &x_ref);
    let dx     = result.unwrap();
    assert_eq!( dx.len(), nx);
    assert_eq!( dx[0], V::from(1.0f32) );
    assert_eq!( dx[1], V::from(-1.0f32) );
}
//
#[test]
fn rust_src() {
    test_sub();
}
