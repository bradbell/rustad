// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// Example converting a NumVec function to source code.
//
use rustad::{
    NumVec,
    AzFloat,
    AD,
    ad_from_value,
    start_recording,
    stop_recording,
    get_lib,
    RustSrcLink,
    get_rust_src_fn,
    create_src_dir,
};
//
fn main () {
    //
    //
    type S     = AzFloat<f32>;
    type V     = NumVec<S>;
    let nx     = 3;
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
    // lib_src
    let gn_name  = "num_vec_sumsq";
    let lib_src  = f.rust_src(gn_name);
    //
    // src_dir
    let src_dir = "tmp/example_num_vec_src";
    create_src_dir(src_dir, &lib_src);
    //
    // lib
    let lib_file    = "tmp/example_num_vec_src.so";
    let replace_lib = true;
    let lib         = get_lib(src_dir, lib_file, replace_lib);
    //
    // num_vec_sumsq_fn
    let num_vec_sumsq_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // v
    let sv  : Vec<S>  = (0 .. nx).map( |j| S::from(j) ).collect();
    let v   : V       = NumVec::new( sv );
    //
    // p_ref, x_ref
    let p_ref : Vec<&V> = Vec::new();
    let x     : Vec<V>  = vec![ v.clone() ; nx ];
    let mut x_ref : Vec<&V> = Vec::new();
    for x_j in x.iter() {
        x_ref.push( &x_j )
    }
    //
    // check result
    let result = num_vec_sumsq_fn(&p_ref, &x_ref);
    let sumsq  = result.unwrap();
    assert_eq!( sumsq.len(), 1);
    for j in 0 .. nx {
        let check : AzFloat<f32> = AzFloat::from( j * j * nx );
        assert_eq!( sumsq[0].get(j), check );
    }
}
