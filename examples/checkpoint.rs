// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
// ---------------------------------------------------------------------------
// Example of doing checkpointing using atomic functions.
//
// TODO: convert this example in to a general purpose checkpoint utility.
// ---------------------------------------------------------------------------
//
use rustad::{
    Direction,
    AzFloat,
    AD,
    ADfn,
    ad_from_value,
    start_recording,
    stop_recording,
    register_checkpoint,
    call_checkpoint,
    ad_from_vector,
};
//
// V
type V = AzFloat<f64>;
//
// sumsq_fn
fn sumsq_fn(nx : usize) -> ADfn<V> {
    let x   : Vec<V> = vec![ V::from(1.0);  nx ];
    let (_, ax)      = start_recording(None, x);
    let mut asumsq : AD<V> = ad_from_value( V::from(0) );
    for j in 0 .. nx {
        let term = &ax[j] * &ax[j];
        asumsq  += &term;
    }
    let ay      = vec![ asumsq ];
    let f       = stop_recording(ay);
    f
}
//
// no_ad_derivative
fn no_ad_derivative() {
    //
    // trace
    let trace = false;
    //
    // nx, f
    let nx = 2;
    let f  = sumsq_fn(nx);
    //
    // checkpoint_id
    let directions  : Vec<Direction> = Vec::new();
    let arg_vec     = vec![
        ["name",  "example"],
        ["trace", "false"  ],
    ];
    let checkpoint_id  = register_checkpoint(f, &directions, &arg_vec);
    //
    // g
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let ay           = call_checkpoint(ax, checkpoint_id, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_var_value
    let x       : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, v)           = g.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_der_value
    let dx      : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let dy               = g.forward_der_value(None, &v , dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * ( x[0]*dx[0] + x[1]*dx[1] ) );
    //
    // g.reverse_der_value
    let dy      : Vec<V> = vec![ V::from(5.0) ];
    let dx               = g.reverse_der_value(None, &v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0]*dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1]*dy[0] );
    //
    // g.forward_var_ad
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let (ay, _av)    = g.forward_var_ad(None, ax, trace);
    let h            = stop_recording(ay);
    let x   : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, _v)      = h.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// forward_ad_derivative
fn forward_ad_derivative() {
    //
    // trace
    let trace = false;
    //
    // nx, f
    let nx = 2;
    let f  = sumsq_fn(nx);
    //
    // checkpoint_id
    let directions  = vec![ Direction::Forward ];
    let arg_vec     = vec![
        ["name",  "example"],
        ["trace", "false"  ],
    ];
    let checkpoint_id  = register_checkpoint(f, &directions, &arg_vec);
    //
    // g
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let ay           = call_checkpoint(ax, checkpoint_id, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_der_ad
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let dx  : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let adx          = ad_from_vector(dx.clone());
    let (_, ax)      = start_recording(None, x);
    let (_ay, av)    = g.forward_var_ad(None, ax, trace);
    let ady          = g.forward_der_ad(None, &av , adx, trace);
    let h            = stop_recording(ady);
    let x   : Vec<V> = vec![ V::from(7.0) , V::from(8.0) ];
    let (dy, _v)     = h.forward_var_value(None, x.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * ( x[0]*dx[0] + x[1]*dx[1] ) );
}
//
// reverse_ad_derivative
fn reverse_ad_derivative() {
    //
    // trace
    let trace = false;
    //
    // nx, f
    let nx = 2;
    let f  = sumsq_fn(nx);
    //
    // checkpoint_id
    let directions  = vec![ Direction::Reverse ];
    let arg_vec     = vec![
        ["name",  "example"],
        ["trace", "false"  ],
    ];
    let checkpoint_id  = register_checkpoint(f, &directions, &arg_vec);
    //
    // g
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let ay           = call_checkpoint(ax, checkpoint_id, trace);
    let g            = stop_recording(ay);
    //
    // g.reverse_der_ad
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let dy  : Vec<V> = vec![ V::from(5.0) ];
    let ady          = ad_from_vector(dy.clone());
    let (_, ax)      = start_recording(None, x);
    let (_ay, av)    = g.forward_var_ad(None, ax, trace);
    let adx          = g.reverse_der_ad(None, &av , ady, trace);
    let h            = stop_recording(adx);
    let x   : Vec<V> = vec![ V::from(7.0) , V::from(8.0) ];
    let (dx, _v)     = h.forward_var_value(None, x.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0] * dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1] * dy[0] );
}
//
// main
fn main() {
    no_ad_derivative();
    forward_ad_derivative();
    reverse_ad_derivative();
}
