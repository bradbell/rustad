// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the tanh operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = tanh(x) = sinh(x) / cosh(x)
// z_x = [ cosh(x)^2 - sinh(x)^2 ] / cosh(x)^2 = 1- tanh(x)^2 = 1 - z^2
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
    Sub,
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
use crate::op::id::TANH_OP;
// -------------------------------------------------------------------------
// tanh_forward_dyp
common::forward_dyp!(tanh);
//
// sim_forward_var
common::forward_var!(tanh);
//
// tanh_rust_src
common::rust_src!(tanh);
//
// tanh_forward_der
/// First order forward mode for tanh(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn tanh_forward_der<V, E>(
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
    for<'a> &'a E : Sub<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let one      = E::one();
    let x        = arg[0] as usize;
    let z        = res;
    let z_x         = &one - &( &var_all[z] * &var_all[z] );
    var_der[z]   = &z_x * &var_der[x];
}
// tanh_reverse_der
/// First order reverse mode for tanh(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn tanh_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : AddAssign<&'a E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a E : Sub<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let one         = E::one();
    let x           = arg[0] as usize;
    let z           = res;
    let z_x         = &one - &( &var_all[z] * &var_all[z] );
    var_der[x]     += &( &z_x * &var_der[z] );
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the TANH_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for TANH_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    //
    for<'a> &'a AD<V> : Sub<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Sub<&'a V, Output = V> ,
    //
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_info_vec[TANH_OP as usize] = OpInfo{
        name              : "tanh",
        forward_dyp_value : tanh_forward_dyp::<V, V>,
        forward_dyp_ad    : tanh_forward_dyp::<V, AD<V> >,
        forward_var_value : tanh_forward_var::<V, V>,
        forward_var_ad    : tanh_forward_var::<V, AD<V> >,
        forward_der_value : tanh_forward_der::<V, V>,
        forward_der_ad    : tanh_forward_der::<V, AD<V> >,
        reverse_der_value : tanh_reverse_der::<V, V>,
        reverse_der_ad    : tanh_reverse_der::<V, AD<V> >,
        rust_src          : tanh_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
