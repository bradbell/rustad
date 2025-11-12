// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AzFloat
};
//
// test_multiply
fn test_multiply() {
    let zero  = AzFloat( 0f32 );
    let nan   = AzFloat( f32::NAN );
    let prod  = zero * nan;
    assert_eq!( prod, zero );
    assert_eq!( nan == nan, true );
    //
    let three = AzFloat( 3f64 );
    let four  = AzFloat( 4f64 );
    let prod  = &three * &four;
    assert_eq!( prod.to_inner(), 12f64 );
    //
    let mut six   = AzFloat( 2f64 );
    six          *= &AzFloat( 3f64 );
    assert_eq!( six, AzFloat( 6f64 ) );
}
//
// test_other
fn test_other() {
    let zero  = AzFloat( 0f32 );
    let nan   = AzFloat( f32::NAN );
    let prod  = zero + nan;
    assert_eq!( prod, nan );
    //
    let twelve = AzFloat( 12f64 );
    let four   = AzFloat( 4f64 );
    let ratio  = twelve / four;
    assert_eq!( ratio.to_inner(), 3f64 );
    //
    let twelve = AzFloat( 12f64 );
    let four   = AzFloat( 4f64 );
    let eight  = &twelve - &four;
    assert_eq!( eight, AzFloat( 8f64 ) );
    //
    let mut six   = AzFloat( 12f64 );
    six          /= &AzFloat( 2f64 );
    assert_eq!( six, AzFloat( 6f64 ) );
}

#[test]
fn az_float() {
    test_multiply();
    test_other();
}
