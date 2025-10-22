// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Example converting a derivative calculation to rust source code
//
use rustad::{
    AD,
    ad_from_value,
    start_recording,
    stop_recording,
    get_lib,
    RustSrcFn,
    get_rust_src_fn,
    ad_from_vector,
};
//
fn main () {
    //
    //
    type V     = f32;
    let nx     = 3;
    let trace  = false;
    //
    // ax
    let x  : Vec<V> = vec![ 2.0 as V; nx ];
    let ax       = start_recording(x);
    //
    // asum
    let mut asum : AD<V>  = ad_from_value(  0.0 as V );
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
    let x  : Vec<V> = vec![ 2.0 as V; nx ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av, ax, trace);
    //
       //
    // g
    // g(x) = df/dx = [ 2 * x[0], ..., 2 * x[nx-1] ]
    let dy  : Vec<V>  = vec![ 1.0 as V ];
    let ady           = ad_from_vector(dy);
    let adx           = f.reverse_one_ad(&av, ady, trace);
    let g             = stop_recording(adx);
    //
    // src_file
    let gn_name   = "sumsq_reverse_one";
    let src = g.rust_src(gn_name);
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
    let sumsq_reverse_one_fn : RustSrcFn<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // x, y, msg
    let x         : Vec<V>  = vec![ 2.0 as V; nx ];
    let mut dx    : Vec<V>  = Vec::new();
    let mut msg             = String::new();
    let mut x_ref : Vec<&V> = Vec::new();
    for xj in x.iter() {
        x_ref.push( &xj )
    }
    sumsq_reverse_one_fn(&x_ref, &mut dx, &mut msg);
    assert_eq!( &msg, "");
    //
    assert_eq!( dx.len(), nx);
    for j in 0 .. nx {
        assert_eq!( dx[j], 2.0 * x[j] );
    }
}
