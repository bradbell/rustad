// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AzFloat,
    ad_from_value,
    start_recording,
    stop_recording,
};
//
// test_left_zero_one_both_ad
fn test_left_zero_one_both_ad() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(3.0) ];
    //
    let ax  = start_recording( x.clone() );
    let a1  = &ad_from_value(V::from(0.0)) + &ax[0]; // optimized to ax[0]
    let a2  = &ad_from_value(V::from(1.0)) * &a1;    // optimized to ax[0]
    let a3  = &ad_from_value(V::from(0.0)) * &a2;    // constant 0
    let a4  = &a3 + &a2;                         // optimized to ax[0]
    let ay  = vec![ a4 ];
    let f   = stop_recording(ay);
    //
    let (y, _)       = f.forward_zero_value(x.clone(), trace);
    //
    // f.var_dep_len()
    // Not necessary to create any dependent variables.
    assert_eq!( f.var_dep_len(), 0 );
    //
    assert_eq!(y[0], x[0]);
}
//
// test_left_zero_one_right_ad
fn test_left_zero_one_right_ad() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(3.0) ];
    //
    let ax  = start_recording( x.clone() );
    let a1  = &(V::from(0.0)) + &ax[0]; // optimized to ax[0]
    let a2  = &(V::from(1.0)) * &a1;    // optimized to ax[0]
    let a3  = &(V::from(0.0)) * &a2;    // constant 0
    let a4  = &a3 + &a2;            // optimized to ax[0]
    let a5  = &(V::from(0.0)) / &a4;    // constant 0
    let a6  = &a5 + &a4;            // optimized to ax[0]
    let ay  = vec![ a6 ];
    let f   = stop_recording(ay);
    //
    let (y, _)       = f.forward_zero_value(x.clone(), trace);
    //
    // f.var_dep_len()
    // Not necessary to create any dependent variables.
    assert_eq!( f.var_dep_len(), 0 );
    //
    assert_eq!(y[0], x[0]);
}
//
// test_right_zero_one_both_ad
fn test_right_zero_one_both_ad() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(3.0) ];
    //
    let ax  = start_recording( x.clone() );
    let a1  = &ax[0] + &ad_from_value(V::from(0.0)); // optimized to ax[0]
    let a2  = &a1    * &ad_from_value(V::from(1.0)); // optimized to ax[0]
    let a3  = &a2    * &ad_from_value(V::from(0.0)); // constant 0
    let a4  = &a2 + &a3;                         // optimized to ax[0]
    let a5  = &a4    / &ad_from_value(V::from(1.0)); // optimized to ax[0]
    let ay  = vec![ a5 ];
    let f   = stop_recording(ay);
    //
    let (y, _)       = f.forward_zero_value(x.clone(), trace);
    //
    // f.var_dep_len()
    // Not necessary to create any dependent variables.
    assert_eq!( f.var_dep_len(), 0 );
    //
    assert_eq!(y[0], x[0]);
}
//
// test_right_zero_one_left_ad
fn test_right_zero_one_left_ad() {
    type V      = AzFloat<f64>;
    let trace   = false;
    //
    let x  : Vec<V>  = vec![ V::from(3.0) ];
    //
    let ax  = start_recording( x.clone() );
    let a1  = &ax[0] + &(V::from(0.0)); // optimized to ax[0]
    let a2  = &a1    * &(V::from(1.0)); // optimized to ax[0]
    let a3  = &a2    * &(V::from(0.0)); // constant 0
    let a4  = &a2 + &a3;            // optimized to ax[0]
    let a5  = &a4    / &(V::from(1.0)); // optimized to ax[0]
    let ay  = vec![ a5 ];
    let f   = stop_recording(ay);
    //
    let (y, _)       = f.forward_zero_value(x.clone(), trace);
    //
    // f.var_dep_len()
    // Not necessary to create any dependent variables.
    assert_eq!( f.var_dep_len(), 0 );
    //
    assert_eq!(y[0], x[0]);
}
#[test]
fn optimize() {
    test_left_zero_one_both_ad();
    test_left_zero_one_right_ad();
    //
    test_right_zero_one_both_ad();
    test_right_zero_one_left_ad();
}
