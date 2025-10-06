// -------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// -------------------------------------------------------------------------
//
use rustad::{
    AD,
    ADfn,
    ad_from_vector,
    start_recording,
    stop_recording,
    call_atom,
    IndexT,
};
//
//
// V
use super::V;
//
// value_callback_f
// f(x) = x[0] * x[0] + x[1] * x[1] + ...
fn value_callback_f(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) -> ADfn<V> {
    //
    let x       : Vec<V> = vec![ 1.0 , 2.0 ];
    let ax               = start_recording(x);
    let ay               = call_atom(ax, sumsq_atom_id, call_info, trace);
    let f                = stop_recording(ay);
    f
}
//
// callback_forward_zero_value
pub fn callback_forward_zero_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, y
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = f.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_zero_ad
pub fn callback_forward_zero_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // g(x) = f(x)
    let x      : Vec<V>       = vec![ 3.0 , 4.0 ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    let ay  = f.forward_zero_ad(&mut av , ax.clone(), trace);
    let g   = stop_recording(ay);
    //
    // x, y
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = g.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
    //
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_one_value
pub fn callback_forward_one_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, dx, dy
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dx      : Vec<V> = vec![ 5.0, 6.0 ];
    let dy               = f.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], 2.0 * x[0]*dx[0] + 2.0 * x[1]*dx[1] );
}
//
// callback_forward_one_ad
pub fn callback_forward_one_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // av, dx1, adx1
    let x      : Vec<V>       = vec![ 3.0 , 4.0 ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av , ax, trace);
    let dx1     : Vec<V> = vec![ 5.0, 6.0 ];
    let adx1             = ad_from_vector(dx1.clone());
    //
    // g
    // callback to sumsq_forward_one_ad
    // g(x) = f'(x) * dx1 = 2 * ( x[0] * dx1[0] + x[2] * dx1[2] + ... )
    let ady              = f.forward_one_ad(&av, adx1, trace);
    let g                = stop_recording(ady);
    //
    // x, v, y
    // check forward_zero_value
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = g.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], 2.0 * ( x[0] * dx1[0] + x[1] * dx1[1] ) );
    //
    // check forward_one_value
    let dx2     : Vec<V> = vec![ 7.0, 8.0 ];
    let dy               = g.forward_one_value(&v , dx2.clone(), trace);
    assert_eq!( dy[0], 2.0 * ( dx2[0] * dx1[0] + dx2[1] * dx1[1] ) );
    //
    // check reverse_one_value
    let dy     : Vec<V> = vec![ 9.0 ];
    let dx              = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], 2.0 * dy[0] * dx1[0] );
    assert_eq!( dx[1], 2.0 * dy[0] * dx1[1] );
}
//
// callback_reverse_one_value
pub fn callback_reverse_one_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, dy, dx
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    f.forward_zero_value(&mut v , x.clone(), trace);
    let dy      : Vec<V> = vec![ 5.0 ];
    let dx               = f.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], 2.0 * x[0]*dy[0] );
    assert_eq!( dx[1], 2.0 * x[1]*dy[0] );
}
//
// callback_reverse_one_ad
pub fn callback_reverse_one_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // av, dy1, ady1
    let x      : Vec<V>       = vec![ 3.0 , 4.0 ];
    let ax                    = start_recording(x);
    let mut av : Vec< AD<V> > = Vec::new();
    f.forward_zero_ad(&mut av , ax, trace);
    let dy1     : Vec<V> = vec![ 5.0 ];
    let ady1             = ad_from_vector(dy1.clone());
    //
    // g
    // callback to sumsq_reverse_one_ad
    // g(x) = dy1 * f'(x) = 2 * ( dy1[0] * x[0], dy1[0] * x[1], ... )
    let adx              = f.reverse_one_ad(&av, ady1, trace);
    let g                = stop_recording(adx);
    //
    // x, v, y
    // check forward_zero_value
    let x       : Vec<V> = vec![ 3.0 , 4.0 ];
    let mut v   : Vec<V> = Vec::new();
    let y                = g.forward_zero_value(&mut v , x.clone(), trace);
    assert_eq!( y[0], 2.0 * dy1[0] * x[0]  );
    assert_eq!( y[1], 2.0 * dy1[0] * x[1]  );
}
