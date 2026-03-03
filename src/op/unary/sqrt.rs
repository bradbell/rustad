// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the sqrt operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
    Div,
    AddAssign,
};
//
use crate::{
    IndexT,
    AD,
    FConst,
    FUnary,
};
//
use crate::ad::ADType;
use crate::op::unary::common;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::OpInfo;
use crate::op::id::SQRT_OP;
// -------------------------------------------------------------------------
// sqrt_forward_dyp
common::forward_dyp!(sqrt);
//
// sim_forward_var
common::forward_var!(sqrt);
//
// sqrt_rust_src
common::rust_src!(sqrt);
//
// sqrt_forward_der
/// First order forward mode for sqrt(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn sqrt_forward_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a V : Div<&'a E, Output=E>,
    V             : From<f32>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( f64::from(0.5f32) == 0.5f64 );
    let half     = V::from(0.5f32);
    let index    = arg[0] as usize;
    let dsqrt    = &half / &var_both[res];
    var_der[res] = &dsqrt *  &var_der[index];
}
// sqrt_reverse_der
/// First order reverse mode for sqrt(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn sqrt_reverse_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : AddAssign<&'a E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a V : Div<&'a E, Output=E>,
    V             : From<f32>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let half        = V::from(0.5f32);
    let index       = arg[0] as usize;
    let dsqrt       = &half / &var_both[res];
    let term        = &dsqrt * &var_der[res];
    var_der[index] += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the SQRT_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for SQRT_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    for<'a> &'a V     : Div<&'a V, Output = V> ,
    V                 : Clone + FConst + ThisThreadTape + From<f32>,
    for<'a> &'a V     : FUnary<Output=V>,
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
{
    op_info_vec[SQRT_OP as usize] = OpInfo{
        name              : "sqrt",
        forward_dyp_value : sqrt_forward_dyp::<V, V>,
        forward_dyp_ad    : sqrt_forward_dyp::<V, AD<V> >,
        forward_var_value : sqrt_forward_var::<V, V>,
        forward_var_ad    : sqrt_forward_var::<V, AD<V> >,
        forward_der_value : sqrt_forward_der::<V, V>,
        forward_der_ad    : sqrt_forward_der::<V, AD<V> >,
        reverse_der_value : sqrt_reverse_der::<V, V>,
        reverse_der_ad    : sqrt_reverse_der::<V, AD<V> >,
        rust_src          : sqrt_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
