// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    FloatCore,
    AzFloat,
    start_recording,
    stop_recording,
};
//
// test_cos
fn test_cos() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::cos( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), trace);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    //
    let temp         = FloatCore::sin( &x[0] ) * dx[0];
    assert_eq!( dy[0], FloatCore::minus(&temp) );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    let temp         = FloatCore::sin( &x[0] ) * dy[0];
    assert_eq!( dx[0], FloatCore::minus(&temp) );
}
//
// test_exp
fn test_exp() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::exp( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), trace);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    //
    assert_eq!( dy[0], FloatCore::exp( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0], FloatCore::exp( &x[0] ) * dy[0] );
}
//
// test_minus
fn test_minus() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::minus( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), trace);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    //
    assert_eq!( dy[0].to_inner(), - dx[0].to_inner() );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0].to_inner(), - dy[0].to_inner() );
}
//
// test_signum
fn test_signum() {
    type V      = AzFloat<f32>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::signum( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), trace);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    //
    assert_eq!( dy[0], FloatCore::zero() );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0], FloatCore::zero() );
}
//
// test_sin
fn test_sin() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::sin( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), trace);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    //
    assert_eq!( dy[0], FloatCore::cos( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), trace);
    //
    assert_eq!( dx[0], FloatCore::cos( &x[0] ) * dy[0] );
}
#[test]
fn unary() {
    test_cos();
    test_exp();
    test_minus();
    test_signum();
    test_sin();
}
