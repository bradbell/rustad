// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
// use
use rustad::{
    AD,
    start_recording_dyp_var,
    stop_recording,
    AzFloat,
};
//
// V
type V = AzFloat<f32>;
//
#[test]
fn compress_cop() {
    //
    // trace, n_repeat
    let trace    = true;
    let n_repeat = 2;
    //
    // four, p, x, ap, ax
    let p    = vec! [V::from(2.0) ];
    let x    = vec![ V::from(3.0) ];
    let four = V::from(4.0);
    let (ap, ax) = start_recording_dyp_var(p.clone(), x.clone());
    //
    // ay
    let mut ay : Vec< AD<V> >  = Vec::new();
    for _i in 0 .. n_repeat {
        ay.push( &ap[0] + &four );
    }
    for _i in 0 .. n_repeat {
        ay.push( &ax[0] + &four );
    }
    //
    // f, n_cop
    // 2*n_repeat constants in computation pulse Nan (which is always there).
    let mut f = stop_recording(ay);
    let n_cop = f.cop_len();
    assert_eq!( 2 * n_repeat + 1, n_cop);
    //
    // f, n_cop
    // one version of repeated constant in computation pulse the Nan
    f.optimize(trace);
    let n_cop = f.cop_len();
    assert_eq!( 2, n_cop);
    //
    // y
    let p_both       = f.forward_dyp_value(p.clone(), trace);
    let (y, _y_both) = f.forward_var_value(&p_both, x.clone(), trace);
    //
    // check
    let check = &p[0] + &four;
    for i in 0 .. n_repeat {
        assert_eq!( y[i], check );
    }
    let check = &x[0] + &four;
    for i in 0 .. n_repeat {
        assert_eq!( y[n_repeat + i], check );
    }
}
