// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// Example using numeric comparison operators
//
use rustad::{
    FBinary,
    AzFloat,
    start_recording,
    stop_recording,
};
//
type V = AzFloat<f32>;
//
fn heaviside() {
    // arg_vec
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    // ax
    let x : Vec<V> = vec![ V::from(2) ];
    let (_ap, ax ) = start_recording(None, x);
    //
    // heaviside
    let zero           = V::from(0);
    let ay             = vec![ FBinary::num_ge( &ax[0], &zero) ];
    let heaviside      = stop_recording(ay);
    //
    let x       = vec![ V::from(-1.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(0));
    // ------------------------------------------------------------------------
    // forward_var_value
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(0));
    //
    let x       = vec![ V::from(2.0f32) ];
    let (y, _v) = heaviside.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(1));
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, v)  = heaviside.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(0));
    //
    let dx      = vec![ V::from(2.0f32) ];
    let dy      = heaviside.forward_der_value(None, &v, dx, &arg_vec);
    assert_eq!(dy[0], V::from(0));
    // ------------------------------------------------------------------------
    // forward_var_ad
    //
    // f(x) = if x > 0 { x } else { 0 }
    let x         = vec![ V::from(-1.0f32) ];
    let (_ap, ax) = start_recording(None, x);
    let (ah, av) = heaviside.forward_var_ad(None, ax, &arg_vec);
    let ay        = vec![ &ah[0] * &av[0] ]; // av[0] == ax[0]
    let f         = stop_recording(ay);
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, _v) = f.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(0));
    //
    let x       = vec![ V::from(2.0f32) ];
    let (y, _v) = f.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(2));
}
//
fn abs() {
    // arg_vec
    let arg_vec : Vec<[&str; 2]> = Vec::new();
    //
    // ax
    let x : Vec<V> = vec![ V::from(1) ];
    let (_ap, ax ) = start_recording(None, x);
    //
    // abs
    let z        = V::from(0);
    let az_lt_x  = FBinary::num_lt( &z, &ax[0] );
    let az_ge_x  = FBinary::num_ge( &z, &ax[0] );
    let ax_neg   = &z - &ax[0];
    let ay      = vec![  &(&az_lt_x * &ax[0]) + &(&az_ge_x * &ax_neg) ];
    let abs     = stop_recording(ay);
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, _v) = abs.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(2));
    //
    let x       = vec![ V::from(3.0f32) ];
    let (y, v)  = abs.forward_var_value(None, x, &arg_vec);
    assert_eq!(y[0], V::from(3));
    //
    let dx      = vec![ V::from(1.0f32) ];
    let dy      = abs.forward_der_value(None, &v, dx, &arg_vec);
    assert_eq!(dy[0], V::from(1));
    // ------------------------------------------------------------------------
    // forward_var_ad
    //
    // f(x) = if x > 0 { x * x } else { - x * x }
    let x           = vec![ V::from(-1.0f32) ];
    let (_ap, ax)   = start_recording(None, x);
    let (a_abs, av) = abs.forward_var_ad(None, ax, &arg_vec);
    let ay        = vec![ &a_abs[0] * &av[0] ]; // av[0] == ax[0]
    let f         = stop_recording(ay);
    //
    let x       = vec![ V::from(-2.0f32) ];
    let (y, _v) = f.forward_var_value(None, x, &arg_vec);
    assert_eq!( y[0], V::from(- 4f32 ) );
    //
    let x       = vec![ V::from(2.0f32) ];
    let (y, _v) = f.forward_var_value(None, x, &arg_vec);
    assert_eq!( y[0], V::from( 4f32 ) );
}

fn main() {
    heaviside();
    abs();
}
