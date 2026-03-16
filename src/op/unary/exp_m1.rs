// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the exp_m1 operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = exp(x) - 1
// z_x = z + 1
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Add,
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
use crate::op::id::EXP_M1_OP;
// -------------------------------------------------------------------------
// exp_m1_forward_dyp
common::forward_dyp!(exp_m1);
//
// sim_forward_var
common::forward_var!(exp_m1);
//
// exp_m1_rust_src
common::rust_src!(exp_m1);
//
// exp_m1_forward_der
/// First order forward mode for exp_m1(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn exp_m1_forward_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : FConst ,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a E : Add<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    let z_x      = &var_both[z]  + &V::one();
    var_der[z]   = &z_x *  &var_der[x];
}
// exp_m1_reverse_der
/// First order reverse mode for exp_m1(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn exp_m1_reverse_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : FConst ,
    for<'a> E     : AddAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a E : Add<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x           = arg[0] as usize;
    let z           = res;
    let z_x         = &var_both[z]  + &V::one();
    var_der[x]     += &( &z_x * &var_der[z] );
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the EXP_M1_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for EXP_M1_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    //
    for<'a> &'a AD<V> : Add<&'a V, Output = AD<V> > ,
    for<'a> &'a V     : Add<&'a V, Output = V> ,
    //
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_info_vec[EXP_M1_OP as usize] = OpInfo{
        name              : "exp_m1",
        forward_dyp_value : exp_m1_forward_dyp::<V, V>,
        forward_dyp_ad    : exp_m1_forward_dyp::<V, AD<V> >,
        forward_var_value : exp_m1_forward_var::<V, V>,
        forward_var_ad    : exp_m1_forward_var::<V, AD<V> >,
        forward_der_value : exp_m1_forward_der::<V, V>,
        forward_der_ad    : exp_m1_forward_der::<V, AD<V> >,
        reverse_der_value : exp_m1_reverse_der::<V, V>,
        reverse_der_ad    : exp_m1_reverse_der::<V, AD<V> >,
        rust_src          : exp_m1_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
