// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::function;
use rustad::checkpoint::{store_checkpoint, use_checkpoint};

#[test]
fn simple() {
    let  x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax      = function::ad_domain(&x);
    let ay      = vec![ ax[0] + ax[1], ax[1] * ax[2] ];
    // f(x)  = [x0 + x1, x1 * x2]
    let f       = function::ad_fun(&ay);
    let name    = "f".to_string();
    store_checkpoint(f, &name);
    let  u : Vec<Float>  = vec![ 4.0, 5.0];
    let au      = function::ad_domain(&u);
    let ax      = vec![ au[0], au[0] + au[1], au[1] ];
    let ay      = use_checkpoint(&name, &ax);
    // g(u)     = [ x0(u) + x1(u),     x1(u)     * x2(2) ]
    //          = [ u0    + (u0 + u1), (u0 + u1) * u1 ]
    let g       = function::ad_fun(&ay);
    let trace   = false;
    let (w, _v) = g.forward_zero(&u, trace);
    assert_eq!( w[0], u[0] + u[0] + u[1] );
    assert_eq!( w[1], (u[0] + u[1]) * u[1] );
}
