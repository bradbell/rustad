// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the abs operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
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
use crate::op::id::ABS_OP;
// -------------------------------------------------------------------------
// abs_forward_dyp
common::forward_dyp!(abs);
//
// sim_forward_var
common::forward_var!(abs);
//
// abs_rust_src
common::rust_src!(abs);
//
// abs_forward_der
/// First order forward mode for abs(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn abs_forward_der<V, E>(
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
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    var_der[z]   = &FUnary::signum( &var_both[x] ) *  &var_der[x];
}
// abs_reverse_der
/// First order reverse mode for abs(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn abs_reverse_der<V, E>(
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
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x           = arg[0] as usize;
    let z           = res;
    let term        = &FUnary::signum( &var_both[x] ) * &var_der[z];
    var_der[x]     += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the ABS_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for ABS_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    //
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_info_vec[ABS_OP as usize] = OpInfo{
        name              : "abs",
        forward_dyp_value : abs_forward_dyp::<V, V>,
        forward_dyp_ad    : abs_forward_dyp::<V, AD<V> >,
        forward_var_value : abs_forward_var::<V, V>,
        forward_var_ad    : abs_forward_var::<V, AD<V> >,
        forward_der_value : abs_forward_der::<V, V>,
        forward_der_ad    : abs_forward_der::<V, AD<V> >,
        reverse_der_value : abs_reverse_der::<V, V>,
        reverse_der_ad    : abs_reverse_der::<V, AD<V> >,
        rust_src          : abs_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
