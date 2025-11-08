// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    start_recording,
    stop_recording,
};
//
// test_add_vv
fn test_add_vv() {
    type V      = f64;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ 1.0, 2.0, 3.0 ];
    //
    let ax           = start_recording( x.clone() );
    let ay_0         = &ax[0] + &ax[1];
    let ay_1         = &ax[1] + &ax[2];
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
    //
    let dx : Vec<V>  = vec![ 4.0, 5.0, 6.0 ];
    let dy           = f.forward_one_value(&v, dx.clone(), trace);
    assert_eq!( dy[0], dx[0] + dx[1] );
    assert_eq!( dy[1], dx[1] + dx[2] );
    //
    let dy : Vec<V>  = vec![ 7.0, 8.0 ];
    let dx           = f.reverse_one_value(&v, dy.clone(), trace);
    //
    assert_eq!( dx[0], dy[0] );
    assert_eq!( dx[1], dy[0] + dy[1] );
    assert_eq!( dx[2], dy[1] );
}
//
// test_add_vc
fn test_add_vc() {
    type V      = f64;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ 1.0, 2.0 ];
    //
    let ax           = start_recording( x.clone() );
    let ay_0         = &ax[0] + &(4.0 as V);
    let ay_1         = &ax[1] + &(5.0 as V);
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0] + (4.0 as V) );
    assert_eq!( y[1], x[1] + (5.0 as V) );
    //
    let dx : Vec<V>  = vec![ 4.0, 5.0 ];
    let dy           = f.forward_one_value(&v, dx.clone(), trace);
    assert_eq!( dy[0], dx[0] );
    assert_eq!( dy[1], dx[1] );
    //
    let dy : Vec<V>  = vec![ 7.0, 8.0 ];
    let dx           = f.reverse_one_value(&v, dy.clone(), trace);
    //
    assert_eq!( dx[0], dy[0] );
    assert_eq!( dx[1], dy[1] );
}
//
// test_add_cv
fn test_add_cv() {
    type V      = f64;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ 1.0, 2.0 ];
    //
    let ax           = start_recording( x.clone() );
    let ay_0         = &(4.0 as V) + &ax[1];
    let ay_1         = &(5.0 as V) + &ax[0];
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], (4.0 as V) + x[1] );
    assert_eq!( y[1], (5.0 as V) + x[0] );
    //
    let dx : Vec<V>  = vec![ 4.0, 5.0 ];
    let dy           = f.forward_one_value(&v, dx.clone(), trace);
    assert_eq!( dy[0], dx[1] );
    assert_eq!( dy[1], dx[0] );
    //
    let dy : Vec<V>  = vec![ 7.0, 8.0 ];
    let dx           = f.reverse_one_value(&v, dy.clone(), trace);
    //
    assert_eq!( dx[0], dy[1] );
    assert_eq!( dx[1], dy[0] );
}
#[test]
fn add_op() {
    test_add_vv();
    test_add_vc();
    test_add_cv();
}
