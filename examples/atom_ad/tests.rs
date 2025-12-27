// -------------------------------------------------------------------------
// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// -------------------------------------------------------------------------
//
use rustad::{
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
    let x       : Vec<V> = vec![ V::from(1.0), V::from(2.0) ];
    let (_, ax)          = start_recording(None, x);
    let ny               = 1;
    let ay               = call_atom(ny, ax, sumsq_atom_id, call_info, trace);
    let f                = stop_recording(ay);
    f
}
//
// callback_forward_fun_value
pub fn callback_forward_fun_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, y
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (y, _)           = f.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_fun_ad
pub fn callback_forward_fun_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // g(x) = f(x)
    let x      : Vec<V>   = vec![ V::from(3.0), V::from(4.0) ];
    let (_, ax)           = start_recording(None, x);
    let (ay, _)           = f.forward_zero_ad(ax.clone(), trace);
    let g                 = stop_recording(ay);
    //
    // x, y
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (y, _)            = g.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], x[0]*x[0] + x[1]*x[1] );
}
//
// callback_forward_der_value
pub fn callback_forward_der_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // x, dx, dy
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (_, v)           = f.forward_zero_value(x.clone(), trace);
    let dx      : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let dy               = f.forward_one_value(&v , dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * x[0]*dx[0] + V::from(2.0) * x[1]*dx[1] );
}
//
// callback_forward_der_ad
pub fn callback_forward_der_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // av, dx1, adx1
    let x      : Vec<V>  = vec![ V::from(3.0), V::from(4.0) ];
    let (_, ax)          = start_recording(None, x);
    let (_, av)          = f.forward_zero_ad(ax, trace);
    let dx1     : Vec<V> = vec![ V::from(5.0), V::from(6.0) ];
    let adx1             = ad_from_vector(dx1.clone());
    //
    // g
    // callback to sumsq_forward_der_ad
    // g(x) = f'(x) * dx1 = 2 * ( x[0] * dx1[0] + x[2] * dx1[2] + ... )
    let ady              = f.forward_one_ad(&av, adx1, trace);
    let g                = stop_recording(ady);
    //
    // x, v, y
    // check forward_fun_value
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (y, v)           = g.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], V::from(2.0) * ( x[0] * dx1[0] + x[1] * dx1[1] ) );
    //
    // check forward_der_value
    let dx2     : Vec<V> = vec![ V::from(7.0), V::from(8.0) ];
    let dy               = g.forward_one_value(&v , dx2.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * ( dx2[0] * dx1[0] + dx2[1] * dx1[1] ) );
    //
    // check reverse_der_value
    let dy     : Vec<V> = vec![ V::from(9.0) ];
    let dx              = g.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * dy[0] * dx1[0] );
    assert_eq!( dx[1], V::from(2.0) * dy[0] * dx1[1] );
}
//
// callback_reverse_der_value
pub fn callback_reverse_der_value(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // v, x, dy, dx
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (_, v)           = f.forward_zero_value(x.clone(), trace);
    let dy      : Vec<V> = vec![ V::from(5.0) ];
    let dx               = f.reverse_one_value(&v , dy.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * x[0]*dy[0] );
    assert_eq!( dx[1], V::from(2.0) * x[1]*dy[0] );
}
//
// callback_reverse_der_ad
pub fn callback_reverse_der_ad(
    sumsq_atom_id : IndexT , call_info : IndexT, trace : bool
) {
    //
    // f
    let f = value_callback_f(sumsq_atom_id, call_info, trace);
    //
    // av, dy1, ady1
    let x      : Vec<V>       = vec![ V::from(3.0), V::from(4.0) ];
    let (_, ax)               = start_recording(None, x);
    let (_, av)               = f.forward_zero_ad(ax, trace);
    let dy1     : Vec<V> = vec![ V::from(5.0) ];
    let ady1             = ad_from_vector(dy1.clone());
    //
    // g
    // callback to sumsq_reverse_der_ad
    // g(x) = dy1 * f'(x) = 2 * ( dy1[0] * x[0], dy1[0] * x[1], ... )
    let adx              = f.reverse_one_ad(&av, ady1, trace);
    let g                = stop_recording(adx);
    //
    // x, v
    // check forward_fun_value
    let x       : Vec<V> = vec![ V::from(3.0), V::from(4.0) ];
    let (y, v)           = g.forward_zero_value(x.clone(), trace);
    assert_eq!( y[0], V::from(2.0) * dy1[0] * x[0]  );
    assert_eq!( y[1], V::from(2.0) * dy1[0] * x[1]  );
    //
    // check forward_der_value
    let dx  : Vec<V> = vec![ V::from(6.0), V::from(7.0) ];
    let dy           = g.forward_one_value(&v, dx.clone(), trace);
    assert_eq!( dy[0], V::from(2.0) * dy1[0] * dx[0] );
    assert_eq!( dy[1], V::from(2.0) * dy1[0] * dx[1] );
    //
    // check reverse_der_value
    let dy2  : Vec<V> = vec![ V::from(8.0), V::from(9.0) ];
    let dx            = g.reverse_one_value(&v, dy2.clone(), trace);
    assert_eq!( dx[0], V::from(2.0) * dy1[0] * dy2[0] );
    assert_eq!( dx[1], V::from(2.0) * dy1[0] * dy2[1] );
}
