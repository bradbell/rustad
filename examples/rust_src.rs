// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Example converting a derivative calculation to rust source code
//
use rustad::{
    AzFloat,
    AD,
    ad_from_value,
    start_recording,
    stop_recording,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
    ad_from_vector,
};
//
fn main () {
    //
    //
    type V     = AzFloat<f32>;
    let nx     = 3;
    let trace  = false;
    //
    // ax
    let x  : Vec<V> = vec![ V::from(2.0); nx ];
    let (_, ax)  = start_recording(None, x);
    //
    // asum
    let mut asum : AD<V>  = ad_from_value(  V::from(0.0) );
    for j in 0 .. nx {
        let square  = &ax[j] * &ax[j];
        asum       += &square;
    }
    //
    // f
    // f(x) = x[0] * x[0] + ... + x[nx-1] * x[nx-1]
    let ay = vec![ asum ];
    let f  = stop_recording(ay);
    //
    // av
    let x  : Vec<V> = vec![ V::from(2.0); nx ];
    let (_, ax) = start_recording(None, x);
    let (_, av) = f.forward_var_ad(None, ax, trace);
    //
    // g
    // g(x) = df/dx = [ 2 * x[0], ..., 2 * x[nx-1] ]
    let dy  : Vec<V>  = vec![ V::from(1.0) ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_der_ad(None, &av, ady, trace);
    let g             = stop_recording(adx);
    //
    // src
    let src      = String::from( rustad::AZ_FLOAT_SRC );
    let gn_name  = "sumsq_reverse_one";
    let src      = src + &g.rust_src(gn_name);
    //
    // src_file
    let src_file = "tmp/example_rust_src.rs";
    let result = std::fs::write(src_file, src);
    if result.is_err() {
        panic!( "Cannot write {src_file}"  );
    }
    //
    // lib
    let lib_file    = "tmp/example_rust_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_file, lib_file, replace_lib);
    //
    // sumsq_fn
    let sumsq_reverse_one_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
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
    let result = sumsq_reverse_one_fn(&p_ref, &x_ref);
    let dx     = result.unwrap();
    assert_eq!( dx.len(), nx);
    for j in 0 .. nx {
        assert_eq!( dx[j], V::from(2.0) * x[j] );
    }
}
