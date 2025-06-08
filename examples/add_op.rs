// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::OP_INFO_VEC;
use rustad::ADD_VV_OP;
use rustad::Index;
use rustad::Float;

/// add Example
fn add_example() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax   = rustad::independent(&x);
    let ay_0 = ax[0] + ax[1];
    let ay_1 = ax[1] + ax[2];
    let ay = vec! [ ay_0, ay_1 ];
    let f = rustad::dependent(&ay);
    let y = f.forward(&x);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
}

fn main() {
    add_example();
}
