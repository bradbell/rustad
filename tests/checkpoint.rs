// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::GAD;
use rustad::store_checkpoint;
use rustad::use_checkpoint;
//
type Float = f64; // f32 or u32
type Index = u64; // f64 or u64
type AD    = GAD<Float, Index>;

#[test]
fn simple() {
    let trace = false;
    //
    // f
    // f(x) = [x0 + x1, x1 * x2]
    let  x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax : Vec<AD>    = rustad::ad_domain(&x);
    let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2] ];
    let f       = rustad::ad_fun(&ay);
    //
    // f
    // store as a checkpoint function
    let timeout_sec   = 5;
    let checkpoint_id = store_checkpoint(f, timeout_sec);
    //
    // g
    // g(u) = f( u0, u0 + u1, u1)
    //      = [ u0 + u0 + u1 , (u0 + u1) * u1 ]
    let  u : Vec<Float>  = vec![ 4.0, 5.0];
    let au : Vec<AD>     = rustad::ad_domain(&u);
    let ax      = vec![ au[0], au[0] + au[1], au[1] ];
    let ay      = use_checkpoint(checkpoint_id, &ax, trace, timeout_sec);
    let g       = rustad::ad_fun(&ay);
    //
    // w
    // w = g(u)
    let (w, _)  = g.forward_zero(&u, trace);
    assert_eq!( w[0], u[0] + u[0] + u[1] );
    assert_eq!( w[1], (u[0] + u[1]) * u[1] );
}


#[test]
fn constant_in_range_space() {
    //
    // trace
    let trace   = false;
    //
    // f
    // f(x) = [x0 + x1, x1 * x2, constant]
    let  x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax : Vec<AD>    = rustad::ad_domain(&x);
    let  constant : Float = 11.0;
    let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2], AD::from(constant) ];
    let f       = rustad::ad_fun(&ay);
    f.forward_zero(&x, trace);
    //
    // f
    // store as a checkpoint function
    let timeout_sec   = 3;
    let checkpoint_id = store_checkpoint(f, timeout_sec);
    //
    // g
    // g(u) = f( u0, u0 + u1, u1)
    //      = [ u0 + u0 + u1 , (u0 + u1) * u1 , constant]
    let  u : Vec<Float>  = vec![ 4.0, 5.0];
    let au : Vec<AD>     = rustad::ad_domain(&u);
    let ax      = vec![ au[0], au[0] + au[1], au[1] ];
    let ay      = use_checkpoint(checkpoint_id, &ax, trace, timeout_sec);
    let g       = rustad::ad_fun(&ay);
    //
    // w
    // w = g(u)
    let (w, _)  = g.forward_zero(&u, trace);
    assert_eq!( w[0], u[0] + u[0] + u[1] );
    assert_eq!( w[1], (u[0] + u[1]) * u[1] );
    assert_eq!( w[2], constant );
}
