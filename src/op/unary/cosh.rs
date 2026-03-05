// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the cosh operator
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
use crate::op::id::COSH_OP;
// -------------------------------------------------------------------------
// cosh_forward_dyp
common::forward_dyp!(cosh);
//
// sim_forward_var
common::forward_var!(cosh);
//
// cosh_rust_src
common::rust_src!(cosh);
//
// cosh_forward_der
/// First order forward mode for cosh(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn cosh_forward_der<V, E>(
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
    let index    = arg[0] as usize;
    var_der[res] = &FUnary::sinh( &var_both[index] ) *  &var_der[index];
}
// cosh_reverse_der
/// First order reverse mode for cosh(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn cosh_reverse_der<V, E>(
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
    let index       = arg[0] as usize;
    let term        = &FUnary::sinh( &var_both[index] ) * &var_der[res];
    var_der[index] += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the COSH_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for COSH_OP are set.
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
    op_info_vec[COSH_OP as usize] = OpInfo{
        name              : "cosh",
        forward_dyp_value : cosh_forward_dyp::<V, V>,
        forward_dyp_ad    : cosh_forward_dyp::<V, AD<V> >,
        forward_var_value : cosh_forward_var::<V, V>,
        forward_var_ad    : cosh_forward_var::<V, AD<V> >,
        forward_der_value : cosh_forward_der::<V, V>,
        forward_der_ad    : cosh_forward_der::<V, AD<V> >,
        reverse_der_value : cosh_reverse_der::<V, V>,
        reverse_der_ad    : cosh_reverse_der::<V, AD<V> >,
        rust_src          : cosh_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
