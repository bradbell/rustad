// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// Under Construction:
// Example converting function to rust source code
//
use rustad::{
    AD,
    ad_from_value,
    start_recording,
    stop_recording,
    get_lib,
};
//
fn main () {
    //
    type V     = f32;
    let nx     = 3;
    //
    // x
    let x  : Vec<V> = vec![ 2.0 as V; nx ];
    //
    // ax
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
    // src_file
    let fn_name   = "sumsq";
    let src = f.rust_src(fn_name);
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
    // RustADSrcFn
    type RustADSrcFn = fn(
        domain      : &Vec<&V>,
        range       : &mut Vec<V>,
        message     : &mut String,
    );
    //
    // x, y, msg
    let x         : Vec<V>  = vec![ 2.0 as V; nx ];
    let mut y     : Vec<V>  = Vec::new();
    let mut msg             = String::new();
    let mut x_ref : Vec<&V> = Vec::new();
    for xj in x.iter() {
        x_ref.push( &xj )
    }
    //
    // sumsq_fn
    let sumsq_fn : libloading::Symbol<RustADSrcFn>;
    unsafe {
        sumsq_fn = lib.get(b"rustad_src_sumsq").expect("Cannot get function");
    }
    sumsq_fn(&x_ref, &mut y, &mut msg);
    //
    assert_eq!( y.len(), 1 );
    assert_eq!( y[0], (nx as V) * 4.0 );
}
