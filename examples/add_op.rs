// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;

/// add_ad_ad
fn add_ad_ad() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax   = rustad::ad_fun::domain(&x);
    let ay_0 = ax[0] + ax[1];
    let ay_1 = ax[1] + ax[2];
    let ay   = vec! [ ay_0, ay_1 ];
    let f    = rustad::ad_fun::range(&ay);
    let y    = f.forward(&x);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
}

/// add_ad_float
fn add_ad_float() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax   = rustad::ad_fun::domain(&x);
    let ay_0 = ax[0] + 5.0;
    let ay_1 = 5.0 + ax[2];
    let ay   = vec! [ ay_0, ay_1 ];
    let f    = rustad::ad_fun::range(&ay);
    let y    = f.forward(&x);
    assert_eq!( y[0], x[0] + 5.0 );
    assert_eq!( y[1], 5.0 + x[2] );
}

#[test]
fn main() {
    add_ad_ad();
    add_ad_float();
}
