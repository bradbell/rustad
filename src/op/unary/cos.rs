// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the cos operator
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
use crate::op::unary::common;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::OpInfo;
use crate::op::id::COS_OP;
// -------------------------------------------------------------------------
// cos_forward_dyp
common::forward_dyp!(cos);
//
// sim_forward_var
common::forward_var!(cos);
//
// cos_rust_src
common::rust_src!(cos);
//
// cos_forward_der
/// First order forward mode for cos(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn cos_forward_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
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
    let term     = &FloatCore::sin( &var_both[index] ) *  &var_der[index];
    var_der[res] = FloatCore::minus( &term );
}
// cos_reverse_der
/// First order reverse mode for cos(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn cos_reverse_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : SubAssign<&'a E> ,
    E             : FloatCore,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let index       = arg[0] as usize;
    let term        = &FloatCore::sin( &var_both[index] ) * &var_der[res];
    var_der[index] -= &term;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the SIN_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for SIN_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    V                 : Clone + FloatCore + ThisThreadTape ,
    for<'a> V         : SubAssign<&'a V>,
    for<'a> AD<V>     : SubAssign<&'a AD<V> >,
{
    op_info_vec[COS_OP as usize] = OpInfo{
        name              : "cos",
        forward_dyp_value : cos_forward_dyp::<V, V>,
        forward_dyp_ad    : cos_forward_dyp::<V, AD<V> >,
        forward_var_value : cos_forward_var::<V, V>,
        forward_var_ad    : cos_forward_var::<V, AD<V> >,
        forward_der_value : cos_forward_der::<V, V>,
        forward_der_ad    : cos_forward_der::<V, AD<V> >,
        reverse_der_value : cos_reverse_der::<V, V>,
        reverse_der_ad    : cos_reverse_der::<V, AD<V> >,
        rust_src          : cos_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
