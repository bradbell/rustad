// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    FUnary,
    AzFloat,
    start_recording,
    stop_recording,
};
//
// test_powi
fn test_powi() {
    type V      = AzFloat<f64>;
    let opt_vec : Vec<[&str; 2]> = Vec::new();
    //
    let x  : Vec<V>  = vec![ V::from(2.0) ];
    //
    let (_, ax)      = start_recording(None,  x.clone() );
    let ay           = vec! [ FUnary::powi( &ax[0], -2i32) ];
    let f            = stop_recording(ay);
    //
    let (_y, v)      = f.forward_var_value(None, x.clone(), &opt_vec);
    let dx           = vec![ V::from(3.0) ];
    let dy           = f.forward_der_value(None, &v, dx.clone(), &opt_vec);
    //
    let dpowi        =  V::from(2).minus() * (&x[0]).powi(-3i32);
    assert_eq!( dy[0], dpowi * dx[0] );
    //
    let dy           = vec![ V::from(4.0) ];
    let dx           = f.reverse_der_value(None, &v, dy.clone(), &opt_vec);
    //
    assert_eq!( dx[0], dpowi *  dy[0]);
}
#[test]
fn pow() {
    test_powi();
}
