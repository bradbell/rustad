// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    FloatCore,
    AzFloat,
    start_recording,
    stop_recording,
    check_nearly_eq,
};
//
// test_abs
fn test_abs() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(3.0), V::from(-2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ ax[0].abs(), ax[1].abs() ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) , V::from(4.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    for j in 0 .. 2 {
        let temp  = FloatCore::signum( &x[j] ) * dx[j];
        assert_eq!( dy[j], temp );
    }
    //
    let dy           = vec![ V::from(5.0), V::from(6.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    for j in 0 .. 2 {
        let temp  = FloatCore::signum( &x[j] ) * dy[j];
        assert_eq!( dx[j], temp );
    }
}
//
// test_cos
fn test_cos() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::cos( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    let temp         = FloatCore::sin( &x[0] ) * dx[0];
    assert_eq!( dy[0], FloatCore::minus(&temp) );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    let temp         = FloatCore::sin( &x[0] ) * dy[0];
    assert_eq!( dx[0], FloatCore::minus(&temp) );
}
//
// test_cosh
fn test_cosh() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::cosh( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0], FloatCore::sinh( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], FloatCore::sinh( &x[0] ) * dy[0] );
}
//
// test_exp
fn test_exp() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::exp( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0], FloatCore::exp( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], FloatCore::exp( &x[0] ) * dy[0] );
}
//
// test_minus
fn test_minus() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::minus( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0].to_inner(), - dx[0].to_inner() );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0].to_inner(), - dy[0].to_inner() );
}
//
// test_signum
fn test_signum() {
    type V      = AzFloat<f32>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::signum( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0], FloatCore::zero() );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], FloatCore::zero() );
}
//
// test_sin
fn test_sin() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::sin( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0], FloatCore::cos( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], FloatCore::cos( &x[0] ) * dy[0] );
}
//
// test_sinh
fn test_sinh() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::sinh( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    assert_eq!( dy[0], FloatCore::cosh( &x[0] ) * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], FloatCore::cosh( &x[0] ) * dy[0] );
}
//
// test_tan
fn test_tan() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::tan( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    let cos          = FloatCore::cos( &x[0] );
    let sec_sq       = V::from(1.0) / ( cos * cos );
    assert_eq!( dy[0], sec_sq * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    assert_eq!( dx[0], sec_sq *  dy[0] );
}
//
// test_tanh
fn test_tanh() {
    type V      = AzFloat<f64>;
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FloatCore::tanh( &ax[0] ) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &arg_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &arg_vec);
    //
    let cosh         = FloatCore::cosh( &x[0] );
    let sech_sq      = V::from(1.0) / ( cosh * cosh );
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    check_nearly_eq::<V>( &dy[0], &(sech_sq * dx[0]), &arg_vec );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &arg_vec);
    //
    check_nearly_eq::<V>( &dx[0], &(sech_sq *  dy[0]), &arg_vec);
}
#[test]
fn unary() {
    test_abs();
    test_cos();
    test_cosh();
    test_exp();
    test_minus();
    test_signum();
    test_sin();
    test_sinh();
    test_tan();
    test_tanh();
}
