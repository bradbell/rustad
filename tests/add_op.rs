// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AzFloat,
    start_recording,
    stop_recording,
};
//
// test_add_vv
fn test_add_vv() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0), V::from(3.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = &ax[0] + &ax[1];
    let ay_1         = &ax[1] + &ax[2];
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0] + x[1] );
    assert_eq!( y[1], x[1] + x[2] );
    //
    let dx : Vec<V>  = vec![ V::from(4.0), V::from(5.0), V::from(6.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    assert_eq!( dy[0], dx[0] + dx[1] );
    assert_eq!( dy[1], dx[1] + dx[2] );
    //
    let dy : Vec<V>  = vec![ V::from(7.0), V::from(8.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0], dy[0] );
    assert_eq!( dx[1], dy[0] + dy[1] );
    assert_eq!( dx[2], dy[1] );
}
//
// test_add_vc
fn test_add_vc() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = &ax[0] + &(V::from(4.0));
    let ay_1         = &ax[1] + &(V::from(5.0));
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0] + (V::from(4.0)) );
    assert_eq!( y[1], x[1] + (V::from(5.0)) );
    //
    let dx : Vec<V>  = vec![ V::from(4.0), V::from(5.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    assert_eq!( dy[0], dx[0] );
    assert_eq!( dy[1], dx[1] );
    //
    let dy : Vec<V>  = vec![ V::from(7.0), V::from(8.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0], dy[0] );
    assert_eq!( dx[1], dy[1] );
}
//
// test_add_cv
fn test_add_cv() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = &(V::from(4.0)) + &ax[1];
    let ay_1         = &(V::from(5.0)) + &ax[0];
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], (V::from(4.0)) + x[1] );
    assert_eq!( y[1], (V::from(5.0)) + x[0] );
    //
    let dx : Vec<V>  = vec![ V::from(4.0), V::from(5.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    assert_eq!( dy[0], dx[1] );
    assert_eq!( dy[1], dx[0] );
    //
    let dy : Vec<V>  = vec![ V::from(7.0), V::from(8.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
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
