// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// Example using CompareAsNumber operators
//
use rustad::{
    CompareAsNumber,
    AzFloat,
    start_recording,
    stop_recording,
};
//
type V = AzFloat<f32>;
//
fn heaviside() {
    // trace
    let trace = false;
    //
    // ax
    let x : Vec<V> = vec![ V::from(2) ];
    let (_ap, ax ) = start_recording(None, x);
    //
    // heaviside
    let zero           = V::from(0);
    let ay             = vec![ ax[0].num_ge(&zero) ];
    let heaviside      = stop_recording(ay);
    //
    let x       = vec![ V::from(-1.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(0));
    //
    let x       = vec![ V::from(-1.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(0));
    //
    let x       = vec![ V::from(1.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(1));
    //
    let x       = vec![ V::from(-1.0f32) ];
    let (y, v)  = heaviside.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(0));
    //
    let dx      = vec![ V::from(1.0f32) ];
    let dy      = heaviside.forward_der_value(None, &v, dx, trace);
    assert_eq!(dy[0], V::from(0));
}
//
fn abs() {
    // trace
    let trace = false;
    //
    // ax
    let x : Vec<V> = vec![ V::from(2) ];
    let (_ap, ax ) = start_recording(None, x);
    //
    // abs
    let zero    = V::from(0);
    let a_lt    = ax[0].num_lt(&zero);
    let a_ge    = ax[0].num_ge(&zero);
    let a_neg   = &zero - &ax[0];
    let ay      = vec![  &(&a_lt * &a_neg) + &(&a_ge * &ax[0]) ];
    let abs     = stop_recording(ay);
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, _v) = abs.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(2));
    //
    let x       = vec![ V::from(3.0f32) ];
    let (y, v)  = abs.forward_var_value(None, x, trace);
    assert_eq!(y[0], V::from(3));
    //
    let dx      = vec![ V::from(1.0f32) ];
    let dy      = abs.forward_der_value(None, &v, dx, trace);
    assert_eq!(dy[0], V::from(1));
}

fn main() {
    heaviside();
    abs();
}
