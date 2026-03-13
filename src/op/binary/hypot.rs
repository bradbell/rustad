// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate Powf Operators
//!
//! Link to [parent module](super)
//!
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
    Div,
    AddAssign,
};
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FConst,
    FBinary,
};
//
use crate::op::binary::common;
use crate::op::info::OpInfo;
use crate::op::id::HYPOT_OP;
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::op::info::{
        ForwardDer,
        ReverseDer,
};
// -------------------------------------------------------------------------
// hypot_rust_src
// hypot_forward_dyp
// hypot_forward_var
common::f_binary_function!(hypot);
//
// ---------------------------------------------------------------------------
// z    = hypot(x, y)           = sqrt( x^2 + y^2 )
// z_x  = x / sqrt(x^2 + y^2)   = x / z
// z_y  = y / sqrt(x^2 + y^2)   = y / z
// ---------------------------------------------------------------------------
//
// hypot_forward_der
/// first order forward for hypot
///
fn hypot_forward_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a>     E : FConst + AddAssign<&'a E>,
    //
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    //
{
    debug_assert!( arg.len() == 2);
    let x          = arg[0] as usize;
    let y          = arg[1] as usize;
    let z          = res;
    let mut dz : E = FConst::zero();
    if arg_type[0].is_variable() {
        let z_x = &var_both[x] / &var_both[z];
        dz     += &( &z_x * &var_der[x] );
    }
    if arg_type[1].is_variable() {
        let z_y = &var_both[y] / &var_both[z];
        dz     += &( &z_y * &var_der[y] );
    }
    var_der[z] = dz;
}
// ---------------------------------------------------------------------------
//
// hypot_reverse_der
/// first order reverse for hypot
///
fn hypot_reverse_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a>     E : FConst + AddAssign<&'a E>,
    //
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    //
{
    debug_assert!( arg.len() == 2);
    let x          = arg[0] as usize;
    let y          = arg[1] as usize;
    let z          = res;
    if arg_type[0].is_variable() {
        let z_x     = &var_both[x] / &var_both[z];
        var_der[x] += &( &z_x * &var_der[z] );
    }
    if arg_type[1].is_variable() {
        let z_y     = &var_both[y] / &var_both[z];
        var_der[y] += &( &z_y * &var_der[z] );
    }
}
// ---------------------------------------------------------------------------
// set_op_info
//
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The map results for HYPOT_OP
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    V : Clone + FConst + PartialEq + ThisThreadTape,
    //
    for<'a>         V : AddAssign<&'a V>,
    for<'a>     AD<V> : FConst + AddAssign<&'a AD<V>>,
    //
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : Div<&'a V, Output = V> ,
    for<'a> &'a AD<V> : Div<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : FBinary<&'a V, Output = V>,
{
    op_info_vec[HYPOT_OP as usize] = OpInfo{
        name              : "hypot_pp",
        forward_dyp_value : hypot_forward_dyp::<V, V>,
        forward_dyp_ad    : hypot_forward_dyp::<V, AD<V> >,
        forward_var_value : hypot_forward_var::<V, V>,
        forward_var_ad    : hypot_forward_var::<V, AD<V> >,
        forward_der_value : hypot_forward_der::<V, V>,
        forward_der_ad    : hypot_forward_der::<V, AD<V> >,
        reverse_der_value : hypot_reverse_der::<V, V>,
        reverse_der_ad    : hypot_reverse_der::<V, AD<V> >,
        rust_src          : hypot_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
