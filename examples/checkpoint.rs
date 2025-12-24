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
use rustad::{
    AzFloat,
    AD,
    ad_from_value,
    start_recording_var,
    stop_recording,
    call_atom,
};
use rustad::checkpoint::{
    register_checkpoint,
    register_checkpoint_atom,
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
    // atom_id
    let atom_id = register_checkpoint_atom::<V>();
    //
    // f
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let ax           = start_recording_var(x);
    let mut asumsq : AD<V> = ad_from_value( V::from(0) );
    for j in 0 .. ax.len() {
        let term = &ax[j] * &ax[j];
        asumsq  += &term;
    }
    let ay          = vec![ asumsq ];
    let ad_fn       = stop_recording(ay);
    //
    // checkpoint_id
    let checkpoint_id  = register_checkpoint(ad_fn);
    //
    // g
    let x   : Vec<V> = vec![ V::from(1.0) , V::from(2.0) ];
    let ax           = start_recording_var(x);
    let ny           = 1;
    let call_info    = checkpoint_id;
    let ay           = call_atom(ny, ax, atom_id, call_info, trace);
    let g            = stop_recording(ay);
    //
    // g.forward_zero_value
    let x       : Vec<V> = vec![ V::from(3.0) , V::from(4.0) ];
    let (y, v)           = g.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    // g.forward_one_value
    let dx      : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let dy               = g.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * x[0]*dx[0] + V::from(2.0) * x[1]*dx[1] );
    //
    // g.reverse_one_value
    let dy      : Vec<V> = vec![ V::from(5.0) ];
    let dx               = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0]*dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1]*dy[0] );
}
