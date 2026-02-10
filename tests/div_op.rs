// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    AzFloat,
    start_recording,
    stop_recording,
    FloatCore,
};
//
// test_div_pv
fn test_div_pv() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(1.0), V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay_0         = &(V::from(4.0)) / &ax[0];
    let ay_1         = &(V::from(5.0)) / &ax[1];
    let ay           = vec! [ ay_0, ay_1 ];
    let f            = stop_recording(ay);
    //
    let (y, v)       = f.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], (V::from(4.0)) / x[0] );
    assert_eq!( y[1], (V::from(5.0)) / x[1] );
    //
    let dx : Vec<V>  = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), trace);
    assert_eq!( dy[0].minus(), dx[0] * V::from(4.0) / (x[0] * x[0]) );
    assert_eq!( dy[1].minus(), dx[1] * V::from(5.0) / (x[1] * x[1]) );
}

#[test]
fn div_op() {
    test_div_pv();
}
