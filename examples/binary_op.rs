// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::function;

/// example_mul
fn example_mul() {
    let x  : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let py : Vec<Float> = vec![ 7.0, 8.0 ];
    let ax       = function::ad_domain(&x);
    let ay_0     = ax[0] * ax[1];
    let ay_1     = ax[1] * ax[2];
    let ay       = vec! [ ay_0, ay_1 ];
    let f        = function::ad_fun(&ay);
    let trace    = false;
    let (y, v0 ) = f.forward_zero(&x, trace);
    let dy       = f.forward_one(&dx, &v0, trace);
    let px       = f.reverse_one(&py, &v0, trace);
    //
    assert_eq!( y[0], x[0] * x[1] );
    assert_eq!( y[1], x[1] * x[2] );
    //
    assert_eq!( dy[0], x[0] * dx[1] + dx[0] * x[1] );
    assert_eq!( dy[1], x[1] * dx[2] + dx[1] * x[2] );
    //
    assert_eq!( px[0], py[0] * x[1] );
    assert_eq!( px[1], py[0] * x[0] + py[1] * x[2]  );
    assert_eq!( px[2], py[1] * x[1]  );
}

#[test]
fn main() {
    example_mul();
}
