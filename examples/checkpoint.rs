// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
// ---------------------------------------------------------------------------
// Example of doing checkpointing using atomic functions.
//
// TODO: convert this example in to a general purpose checkpoint utility.
// ---------------------------------------------------------------------------
//
use std::collections::HashMap;
//
use rustad::{
    Direction,
    AzFloat,
    AD,
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
// -------------------------------------------------------------------------
// AD routines
// -------------------------------------------------------------------------
//
// -------------------------------------------------------------------------
// main
// -------------------------------------------------------------------------
fn main() {
    //
    // trace
    let trace = false;
    //
    // f
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let mut asumsq : AD<V> = ad_from_value( V::from(0) );
    for j in 0 .. ax.len() {
        let term = &ax[j] * &ax[j];
        asumsq  += &term;
    }
    let ay      = vec![ asumsq ];
    let f       = stop_recording(ay);
    //
    // checkpoint_id
    let directions  = vec![ Direction::Forward ];
    let hash_map    = HashMap::from( [
        ("name",  "example".to_string() ),
        ("trace", "false".to_string() ),
    ] );
    let checkpoint_id  = register_checkpoint(f, &directions, hash_map);
    //
    // g
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let ay           = call_checkpoint(ax, checkpoint_id, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_zero_value
    let x       : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, v)           = g.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_one_value
    let dx      : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let dy               = g.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * ( x[0]*dx[0] + x[1]*dx[1] ) );
    //
    // g.reverse_one_value
    let dy      : Vec<V> = vec![ V::from(5.0) ];
    let dx               = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0]*dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1]*dy[0] );
    //
    // g.forward_zero_ad
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let (_, ax)      = start_recording(None, x);
    let (ay, _av)    = g.forward_var_ad(None, ax, trace);
    let h            = stop_recording(ay);
    let x   : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, _v)      = h.forward_var_value(None, x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_one_value
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let dx  : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let adx          = ad_from_vector(dx.clone());
    let (_, ax)      = start_recording(None, x);
    let (_ay, av)    = g.forward_var_ad(None, ax, trace);
    let ady          = g.forward_one_ad(&av , adx, trace);
    let h            = stop_recording(ady);
    let x   : Vec<V> = vec![ V::from(7.0) , V::from(8.0) ];
    let (dy, _v)     = h.forward_var_value(None, x.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * ( x[0]*dx[0] + x[1]*dx[1] ) );
}
