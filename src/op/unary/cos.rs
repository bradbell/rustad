// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the cos operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = cos(x)
// z_x = - sin(x)
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
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    let z_x      = FUnary::sin( &var_all[x] ).minus();
    var_der[z]   = &z_x *  &var_der[x];
}
// cos_reverse_der
/// First order reverse mode for cos(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn cos_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : SubAssign<&'a E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x           = arg[0] as usize;
    let z           = res;
    let neg_z_x     = FUnary::sin( &var_all[x] );
    var_der[x]     -= &( &neg_z_x * &var_der[z] );
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the SIN_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for SIN_OP are set.
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
    op_fns_vec[COS_OP as usize] = OpFns{
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
