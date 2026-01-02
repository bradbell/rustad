// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// Example using CompareAsNumber operators
//
use rustad::{
    CompareAsNumber,
    AzFloat,
    AD,
    start_recording,
    stop_recording,
};
//
type V = AzFloat<f32>;
//
fn main () {
    // trace
    let trace = false;
    //
    // ax
    let x : Vec<V> = vec![ V::from(2) ];
    let (_ap, ax ) = start_recording(None, x);
    //
    // heaviside
    let azero : AD<V>  = 0f32.into();
    let ay             = vec![ ax[0].num_ge(&azero) ];
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
