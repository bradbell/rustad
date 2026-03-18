// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the ln operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = ln(x)
// z_x = 1 / x
// --------------------------------------------------------------------------
// use
//
use std::ops::{
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
use crate::op::info::OpFns;
use crate::op::info::ConstData;
use crate::op::id::LN_OP;
// -------------------------------------------------------------------------
// ln_forward_dyp
common::forward_dyp!(ln);
//
// sim_forward_var
common::forward_var!(ln);
//
// ln_rust_src
common::rust_src!(ln);
//
// ln_forward_der
/// First order forward mode for ln(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn ln_forward_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    const_data : ConstData<V> )
where
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Div<&'a E, Output=E>,
{
    let ConstData {arg, arg_type, res, ..} = const_data;
    //
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    let inv_z_x  = &var_all[x];
    var_der[z]   = &var_der[x] / inv_z_x;
}
// ln_reverse_der
/// First order reverse mode for ln(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn ln_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    const_data : ConstData<V> )
where
    for<'a> E     : AddAssign<&'a E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Div<&'a E, Output=E>,
{
    let ConstData {arg, arg_type, res, ..} = const_data;
    //
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x           = arg[0] as usize;
    let z           = res;
    let inv_z_x     = &var_all[x];
    var_der[x]     += &( &var_der[z] / inv_z_x );
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the LN_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for LN_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] ) where
    for<'a> &'a AD<V> : Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Div<&'a V, Output = V> ,
    //
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_fns_vec[LN_OP as usize] = OpFns{
        name              : "ln",
        forward_dyp_value : ln_forward_dyp::<V, V>,
        forward_dyp_ad    : ln_forward_dyp::<V, AD<V> >,
        forward_var_value : ln_forward_var::<V, V>,
        forward_var_ad    : ln_forward_var::<V, AD<V> >,
        forward_der_value : ln_forward_der::<V, V>,
        forward_der_ad    : ln_forward_der::<V, AD<V> >,
        reverse_der_value : ln_reverse_der::<V, V>,
        reverse_der_ad    : ln_reverse_der::<V, AD<V> >,
        rust_src          : ln_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
