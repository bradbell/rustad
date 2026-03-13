// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the ln_1p operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Add,
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
use crate::op::id::LN_1P_OP;
// -------------------------------------------------------------------------
// ln_1p_forward_dyp
common::forward_dyp!(ln_1p);
//
// sim_forward_var
common::forward_var!(ln_1p);
//
// ln_1p_rust_src
common::rust_src!(ln_1p);
//
// ln_1p_forward_der
/// First order forward mode for ln_1p(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn ln_1p_forward_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : FConst ,
    for<'a> &'a E : Div<&'a E, Output=E>,
    for<'a> &'a E : Add<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x        = arg[0] as usize;
    let z        = res;
    let plus_one = &var_both[x] + &V::one();
    var_der[z]   = &var_der[x] / &plus_one;
}
// ln_1p_reverse_der
/// First order reverse mode for ln_1p(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn ln_1p_reverse_der<V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : FConst ,
    for<'a> E     : AddAssign<&'a E> ,
    for<'a> &'a E : Div<&'a E, Output=E>,
    for<'a> &'a E : Add<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let x           = arg[0] as usize;
    let z           = res;
    let plus_one    = &var_both[x] + &V::one();
    let term        = &var_der[z] / &plus_one;
    var_der[x]     += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the LN_1P_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for LN_1P_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] ) where
    for<'a> &'a AD<V> : Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Div<&'a V, Output = V> ,
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
    op_info_vec[LN_1P_OP as usize] = OpInfo{
        name              : "ln_1p",
        forward_dyp_value : ln_1p_forward_dyp::<V, V>,
        forward_dyp_ad    : ln_1p_forward_dyp::<V, AD<V> >,
        forward_var_value : ln_1p_forward_var::<V, V>,
        forward_var_ad    : ln_1p_forward_var::<V, AD<V> >,
        forward_der_value : ln_1p_forward_der::<V, V>,
        forward_der_ad    : ln_1p_forward_der::<V, AD<V> >,
        reverse_der_value : ln_1p_reverse_der::<V, V>,
        reverse_der_ad    : ln_1p_reverse_der::<V, AD<V> >,
        rust_src          : ln_1p_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
