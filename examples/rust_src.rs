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
    let fn_name   = "sumsq";
    let src = f.rust_src(fn_name);
    println!("{}", src);
}
