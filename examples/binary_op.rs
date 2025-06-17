// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::{Float, AD, advec};
use rustad::function;

/// example_mul
fn example_mul() {
    let x  : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<Float> = vec![ 7.0, 8.0 ];
    let ax          = function::ad_domain(&x);
    let ay_0        = ax[0] * ax[1];
    let mut ay_1    = ax[1]; // ax[1] * ax[2] using *=
    ay_1           *= ax[2];
    let ay          = vec! [ ay_0, ay_1 ];
    let f           = function::ad_fun(&ay);
    let trace       = false;
    let (y, v0 )    = f.forward_zero(&x, trace);
    let dy          = f.forward_one(&dx, &v0, trace);
    let rx          = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] * x[1] );
    assert_eq!( y[1], x[1] * x[2] );
    //
    assert_eq!( dy[0], x[0] * dx[1] + dx[0] * x[1] );
    assert_eq!( dy[1], x[1] * dx[2] + dx[1] * x[2] );
    //
    assert_eq!( rx[0], ry[0] * x[1] );
    assert_eq!( rx[1], ry[0] * x[0] + ry[1] * x[2]  );
    assert_eq!( rx[2], ry[1] * x[1]  );
}
/// ad_example_mul
fn ad_example_mul() {
    let x   : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let adx : Vec<AD> = advec![ 4.0, 5.0, 6.0 ];
    let ary : Vec<AD> = advec![ 7.0, 8.0 ];
    let ax         = function::ad_domain(&x);
    let ay_0       = ax[0] * ax[1];
    let ay_1       = ax[1] * ax[2];
    let ay         = advec! [ ay_0, ay_1 ];
    let f          = function::ad_fun(&ay);
    let trace      = false;
    let (ay, av0 ) = f.ad_forward_zero(&ax, trace);
    let ady        = f.ad_forward_one(&adx, &av0, trace);
    let arx        = f.ad_reverse_one(&ary, &av0, trace);
    //
    assert_eq!( ay[0], ax[0] * ax[1] );
    assert_eq!( ay[1], ax[1] * ax[2] );
    //
    assert_eq!( ady[0], ax[0] * adx[1] + adx[0] * ax[1] );
    assert_eq!( ady[1], ax[1] * adx[2] + adx[1] * ax[2] );
    //
    assert_eq!( arx[0], ary[0] * ax[1] );
    assert_eq!( arx[1], ary[0] * ax[0] + ary[1] * ax[2]  );
    assert_eq!( arx[2], ary[1] * ax[1]  );
}

#[test]
fn main() {
    example_mul();
    ad_example_mul();
}
