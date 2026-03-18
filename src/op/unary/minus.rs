// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the minus operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = minus(x)
// z_x = -1
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
    SubAssign,
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
use crate::op::info::OpFns;
use crate::op::info::ConstData;
use crate::op::id::MINUS_OP;
// -------------------------------------------------------------------------
// minus_forward_dyp
common::forward_dyp!(minus);
//
// sim_forward_var
common::forward_var!(minus);
//
// minus_rust_src
common::rust_src!(minus);
//
// minus_forward_der
/// First order forward mode for minus(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn minus_forward_der<V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    const_data : ConstData<V> )
where
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    let ConstData {arg, arg_type, res, ..} = const_data;
    //
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    var_der[z]   = FUnary::minus( &var_der[x] );
}
// minus_reverse_der
/// First order reverse mode for minus(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn minus_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    const_data : ConstData<V> )
where
    for<'a> E     : SubAssign<&'a E> ,
    E             : Clone + FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    let ConstData {arg, arg_type, res, ..} = const_data;
    //
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x              = arg[0] as usize;
    let z              = res;
    let (left, right)  = var_der.split_at_mut(z);
    left[x]           -= &right[0];
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the MINUS_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for MINUS_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    //
    for<'a> V         : SubAssign<&'a V>,
    for<'a> AD<V>     : SubAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_fns_vec[MINUS_OP as usize] = OpFns{
        name              : "minus",
        forward_dyp_value : minus_forward_dyp::<V, V>,
        forward_dyp_ad    : minus_forward_dyp::<V, AD<V> >,
        forward_var_value : minus_forward_var::<V, V>,
        forward_var_ad    : minus_forward_var::<V, AD<V> >,
        forward_der_value : minus_forward_der::<V, V>,
        forward_der_ad    : minus_forward_der::<V, AD<V> >,
        reverse_der_value : minus_reverse_der::<V, V>,
        reverse_der_ad    : minus_reverse_der::<V, AD<V> >,
        rust_src          : minus_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
