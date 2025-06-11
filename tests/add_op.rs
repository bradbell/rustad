// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::function;

#[test]
fn test_add_vv() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax   = function::domain(&x);
    let ay_0 = ax[0] + ax[1];
    let ay_1 = ax[1] + ax[2];
    let ay   = vec! [ ay_0, ay_1 ];
    let f    = function::range(&ay);
    let y    = f.forward(&x);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
}

#[test]
fn test_add_vc() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax   = function::domain(&x);
    let ay_0 = ax[0] + 5.0;
    let ay_1 = 5.0 + ax[2];
    let ay   = vec! [ ay_0, ay_1 ];
    let f    = function::range(&ay);
    let y    = f.forward(&x);
    assert_eq!( y[0], x[0] + 5.0 );
    assert_eq!( y[1], 5.0 + x[2] );
}
