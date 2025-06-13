// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::function;

fn time_add_vv() {
    let x : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ax     = function::ad_domain(&x);
    let ay_0   = ax[0] + ax[1];
    let ay_1   = ax[1] + ax[2];
    let ay     = vec! [ ay_0, ay_1 ];
    let f      = function::ad_fun(&ay);
    let trace  = false;
    let y      = f.forward_zero(&x, trace);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
}

fn main() {
    let total_seconds = 0.5;
    let seconds  = rustad::utility::avg_seconds( time_add_vv, total_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per time_add_vv = {:?}", duration);
}
