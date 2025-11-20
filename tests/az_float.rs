// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
use rustad::{
    AzFloat,
    start_recording_var_dyp,
    stop_recording,
    ad_from_value,
};
//
// test_multiply_op
fn test_multiply_op() {
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
// test_other_op
fn test_other_op() {
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
//
// test_forward_dyp
fn test_forward_dyp() {
    //
    // V
    type V = AzFloat<f32>;
    //
    // np, nx, p, x
    let np         = 3;
    let nx         = 1;
    let p : Vec<V> = vec![ V::from(1.0) ; np ];
    let x : Vec<V> = vec![ V::from(1.0) ; nx ];
    //
    // asum
    // The first addition adds the constants zero and so is not recorded
    let (ap, ax)   = start_recording_var_dyp(p.clone(), x.clone());
    let mut asum   = ad_from_value( V::from(0.0) );
    for j in 0 .. np {
        asum += &ap[j];
    }
    //
    // f
    let ay = vec![ &ax[0] * &asum ];
    let f  = stop_recording(ay);
    //
    // dyp_both
    let trace = false;
    let dyp_both = f.forward_dyp_value(p.clone(), trace);
    //
    assert_eq!( dyp_both.len(), 2 * np - 1 );
    for j in 0 .. np {
        assert_eq!( dyp_both[j], p[j] );
    }
    let mut sum = p[0];
    for j in 1 .. np {
    sum += &p[j];
        assert_eq!( dyp_both[np + j - 1], sum );
    }
}

#[test]
fn az_float() {
    test_multiply_op();
    test_other_op();
    test_forward_dyp();
}
