// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::function;

#[test]
fn test_add_vv() {
    let x  : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<Float> = vec![ 7.0, 8.0 ];
    let ax        = function::ad_domain(&x);
    let ay_0      = ax[0] + ax[1];
    let ay_1      = ax[1] + ax[2];
    let ay        = vec! [ ay_0, ay_1 ];
    let f         = function::ad_fun(&ay);
    let trace     = false;
    let (y, v0)   = f.forward_zero(&x, trace);
    let dy        = f.forward_one(&dx, &v0, trace);
    let rx        = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
    //
    assert_eq!( dy[0], dx[0] + dx[1] );
    assert_eq!( dy[1], dx[1] + dx[2] );
}

#[test]
fn test_add_vc() {
    let x  : Vec<Float> = vec![ 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0 ];
    let ry : Vec<Float> = vec![ 7.0 ];
    let ax        = function::ad_domain(&x);
    let ay_0      = ax[0] + 5.0;
    let ay        = vec! [ ay_0 ];
    let f         = function::ad_fun(&ay);
    let trace     = false;
    let (y, v0)   = f.forward_zero(&x, trace);
    let dy        = f.forward_one(&dx, &v0, trace);
    let rx        = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] + 5.0 );
    assert_eq!( dy[0], dx[0] );
    //
    assert_eq!( rx[0], ry[0] );
    assert_eq!( rx[1], 0.0 );
}

#[test]
fn test_add_cv() {
    let x  : Vec<Float> = vec![ 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0 ];
    let ry : Vec<Float> = vec![ 7.0 ];
    let ax        = function::ad_domain(&x);
    let ay_0      = 5.0 + ax[1];
    let ay        = vec! [ ay_0 ];
    let f         = function::ad_fun(&ay);
    let trace     = false;
    let (y, v0)   = f.forward_zero(&x, trace);
    let dy        = f.forward_one(&dx, &v0, trace);
    let rx        = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], 5.0 + x[1] );
    assert_eq!( dy[0], dx[1] );
    //
    assert_eq!( rx[0], 0.0 );
    assert_eq!( rx[1], ry[0] );
}
