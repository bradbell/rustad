// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
use rustad::{
    AD,
    AzFloat,
    FBinary,
    start_recording,
    stop_recording,
    get_rust_src_fn,
    create_src_dir,
    get_lib,
    RustSrcLink
};
//
// zero_one
// TODO: get this to work as a test; i.e., change allow(dead_code) to test.
#[allow(dead_code)]
fn zero_one() {
    //
    // V
    type V  = AzFloat<f64>;
    //
    // opt_forward
    let opt_forward : Vec<[&str; 2]> = Vec::new();
    //
    // start_message, opt_is_one
    let start_message = "value changed; see line number ";
    let mut message   = start_message.to_string();
    message          += &format!( "{}", line!() );
    message          += " in file ";
    message          += file!();
    let opt_is_one = vec![
        [ "panic",   "true" ],
        [ "message", &message ],
    ];
    //
    // p, x, ap, ax
    // Note that p < 3 and x < 3 during this this recording
    let p             = vec![ V::from(1.0) ];
    let x             = vec![ V::from(2.0) ];
    let (ap, ax)      = start_recording(Some( p.clone() ),  x.clone() );
    //
    // aq
    let ap_lt_three   = (&ap[0]).num_lt( &V::from( 3.0 ) );
    let aq            = if ap_lt_three.is_one(&opt_is_one) {
        (&ap[0]) * (&ap[0])
    } else {
        (&ap[0]) + (&ap[0])
    };
    //
    // au
    let ax_lt_three   = (&ax[0]).num_lt( &V::from( 3.0 ) );
    let au            = if ax_lt_three.is_one(&opt_is_one) {
        (&ax[0]) * (&ax[0])
    } else {
        (&ax[0]) + (&ax[0])
    };
    //
    // f
    let ay : Vec< AD<V> > = vec![ aq, au ];
    let f                 = stop_recording(ay);
    //
    // y
    // Note that p < 3 and x < 3 during this forward calculation
    let p            = vec![ V::from(2.0) ];
    let dyp_all      = f.forward_dyp_value(p.clone(), &opt_forward);
    let dyp_all      = Some(&dyp_all);
    let (y, _v_all)  = f.forward_var_value(dyp_all, x.clone(), &opt_forward);
    //
    // check
    assert_eq!( y[0], p[0] * p[0] );
    assert_eq!( y[1], x[0] * x[0] );
    //
    // lib_src
    let gn_name  = "zero_one";
    let lib_src  = f.rust_src(gn_name);
    //
    // src_dir
    let src_dir = "tmp/test_zero_one";
    create_src_dir(src_dir, &lib_src);
    //
    // lib
    let lib_file    = "tmp/test_zero_one.so";
    let replace_lib = true;
    let lib         = get_lib(src_dir, lib_file, replace_lib);
    //
    // zero_one_fn
    let zero_one_fn : RustSrcLink<V> = get_rust_src_fn(&lib, &gn_name);
    //
    // p_ref, x_ref
    let p_ref : Vec<&V> = vec![ &p[0] ];
    let x_ref : Vec<&V> = vec![ &x[0] ];
    //
    // check result
    let result = zero_one_fn(&p_ref, &x_ref);
    let y      = result.unwrap();
    assert_eq!( y.len(), 2);
    assert_eq!( y[0], p[0] * p[0] );
    assert_eq!( y[1], x[0] * x[0] );
}
