// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::{Float, AD};
use rustad::function;
use rustad::checkpoint::{store_checkpoint, use_checkpoint};

#[test]
fn simple() {
    let trace = false;
    //
    // f
    // f(x) = [x0 + x1, x1 * x2]
    let  x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax      = function::ad_domain(&x);
    let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2] ];
    let f       = function::ad_fun(&ay);
    //
    // f
    // store as a checkpoint function
    let name    = "f".to_string();
    store_checkpoint(f, &name);
    //
    // g
    // g(u) = f( u0, u0 + u1, u1)
    //      = [ u0 + u0 + u1 , (u0 + u1) * u1 ]
    let  u : Vec<Float>  = vec![ 4.0, 5.0];
    let au      = function::ad_domain(&u);
    let ax      = vec![ au[0], au[0] + au[1], au[1] ];
    let ay      = use_checkpoint(&name, &ax, trace);
    let g       = function::ad_fun(&ay);
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
    let  constant : Float = 11.0;
    let ax      = function::ad_domain(&x);
    let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2], AD::from(constant) ];

    let f       = function::ad_fun(&ay);
    f.forward_zero(&x, trace);
    //
    // f
    // store as a checkpoint function
    let name    = "f".to_string();
    store_checkpoint(f, &name);
    //
    // g
    // g(u) = f( u0, u0 + u1, u1)
    //      = [ u0 + u0 + u1 , (u0 + u1) * u1 , constant]
    let  u : Vec<Float>  = vec![ 4.0, 5.0];
    let au      = function::ad_domain(&u);
    let ax      = vec![ au[0], au[0] + au[1], au[1] ];
    let ay      = use_checkpoint(&name, &ax, trace);
    let g       = function::ad_fun(&ay);
    //
    // w
    // w = g(u)
    let (w, _)  = g.forward_zero(&u, trace);
    assert_eq!( w[0], u[0] + u[0] + u[1] );
    assert_eq!( w[1], (u[0] + u[1]) * u[1] );
    assert_eq!( w[2], constant );
}
