// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

use rustad::Float;
use rustad::AD;
use rustad::function;
use rustad::utility::avg_seconds_to_execute;

fn vec_float2ad(vec : &Vec<Float> ) -> Vec<AD> {
    let mut result = Vec::new();
    for value in vec {
        result.push( AD::from(*value) );
    }
    result
}

fn test_add_vv() {
    let x  : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<Float> = vec![ 7.0, 8.0 ];
    let ax      = function::ad_domain(&x);
    let ay_0    = ax[0] + ax[1];
    let ay_1    = ax[1] + ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    let trace   = false;
    let (y, v0) = f.forward_zero(&x, trace);
    let dy      = f.forward_one(&dx, &v0, trace);
    let rx      = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
    //
    assert_eq!( dy[0], dx[0] + dx[1] );
    assert_eq!( dy[1], dx[1] + dx[2] );
    //
    assert_eq!( rx[0], ry[0] );
    assert_eq!( rx[1], ry[0] + ry[1] );
    assert_eq!( rx[2], ry[1] );
}

fn test_mul_vv() {
    let x  : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let dx : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let ry : Vec<Float> = vec![ 6.0, 8.0 ];
    let ax      = function::ad_domain(&x);
    let ay_0    = ax[0] * ax[1];
    let ay_1    = ax[1] * ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    let trace   = false;
    let (y, v0) = f.forward_zero(&x, trace);
    let dy      = f.forward_one(&dx, &v0, trace);
    let rx      = f.reverse_one(&ry, &v0, trace);
    //
    assert_eq!( y[0], x[0] * x[1] );
    assert_eq!( y[1], x[1] * x[2] );
    //
    assert_eq!( dy[0], dx[0] * x[1] + x[0] * dx[1] );
    assert_eq!( dy[1], dx[1] * x[2] + x[1] * dx[2] );
    //
    assert_eq!( rx[0], ry[0] * x[1] );
    assert_eq!( rx[1], ry[0] * x[0] + ry[1] * x[2] );
    assert_eq!( rx[2], ry[1] * x[1] );
}

fn test_ad_mul_vv() {
    let x     : Vec<Float> = vec![ 1.0, 2.0, 3.0 ];
    let ad_x  : Vec<AD>    = vec_float2ad(&x);
    //
    let dx    : Vec<Float> = vec![ 4.0, 5.0, 6.0 ];
    let ad_dx : Vec<AD>    = vec_float2ad(&dx);
    //
    let ry    : Vec<Float> = vec![ 6.0, 8.0 ];
    let ad_ry : Vec<AD>    = vec_float2ad(&ry);
    //
    let ax      = function::ad_domain(&x);
    let ay_0    = ax[0] * ax[1];
    let ay_1    = ax[1] * ax[2];
    let ay      = vec! [ ay_0, ay_1 ];
    let f       = function::ad_fun(&ay);
    //
    let trace   = false;
    //
    let (ad_y, ad_v0) = f.forward_zero(&ad_x, trace);
    let ad_dy         = f.ad_forward_one(&ad_dx, &ad_v0, trace);
    let ad_rx         = f.ad_reverse_one(&ad_ry, &ad_v0, trace);
    //
    assert_eq!( ad_y[0].to_value(), x[0] * x[1] );
    assert_eq!( ad_y[1].to_value() , x[1] * x[2] );
    //
    assert_eq!( ad_dy[0].to_value() , dx[0] * x[1] + x[0] * dx[1] );
    assert_eq!( ad_dy[1].to_value() , dx[1] * x[2] + x[1] * dx[2] );
    //
    assert_eq!( ad_rx[0].to_value() , ry[0] * x[1] );
    assert_eq!( ad_rx[1].to_value() , ry[0] * x[0] + ry[1] * x[2] );
    assert_eq!( ad_rx[2].to_value() , ry[1] * x[1] );
}

fn bench( name : &String, test_case : fn() ) {
    let total_seconds = 0.25;
    let seconds  = avg_seconds_to_execute( test_case, total_seconds );
    let nanos    = (seconds * 1e9 + 0.5) as u64;
    let duration = std::time::Duration::from_nanos(nanos);
    println!( "time per {:?} = {:?}", name, duration);
}

fn main() {
    bench( &"test_add_vv".to_string() , test_add_vv );
    bench( &"test_mul_vv".to_string() , test_mul_vv );
    bench( &"test_ad_mul_vv".to_string() , test_ad_mul_vv );
}
