// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the minus operator
//!
//! Link to [parent module](super)
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
    FloatCore,
};
//
use crate::ad::ADType;
use crate::op::unary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::OpInfo;
use crate::op::id::MINUS_OP;
// -------------------------------------------------------------------------
// minus_forward_dyp
unary::forward_dyp!(minus);
//
// sim_forward_var
unary::forward_var!(minus);
//
// minus_rust_src
unary::rust_src!(minus);
//
// minus_forward_der
/// First order forward mode for minus(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn minus_forward_der<V, E>(
    _dyp_both  :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    E             : FloatCore,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let index    = arg[0] as usize;
    var_der[res] = FloatCore::minus( &var_der[index] );
}
// minus_reverse_der
/// First order reverse mode for minus(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn minus_reverse_der<V, E>(
    _dyp_both  :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : SubAssign<&'a E> ,
    E             : Clone + FloatCore,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let index          = arg[0] as usize;
    let (left, right)  = var_der.split_at_mut(res);
    left[index]       -= &right[0];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the MINUS_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for MINUS_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    V                 : Clone + FloatCore + ThisThreadTape ,
    for<'a> V         : SubAssign<&'a V>,
    for<'a> AD<V>     : SubAssign<&'a AD<V> >,
{
    op_info_vec[MINUS_OP as usize] = OpInfo{
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
        reverse_depend    : unary::reverse_depend,
    };
}
