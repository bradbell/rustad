// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate Powf Operators
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// z    = atan2(y, x)
// z_x  = - y / (x^2 + y^2)
// z_y  =   x / (x^2 + y^2)
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Add,
    Mul,
    Div,
    AddAssign,
};
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FConst,
    FUnary,
    FBinary,
};
//
use crate::op::binary::common;
use crate::op::info::OpInfo;
use crate::op::id::ATAN2_OP;
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::op::info::{
        ForwardDer,
        ReverseDer,
};
// -------------------------------------------------------------------------
// atan2_rust_src
// atan2_forward_dyp
// atan2_forward_var
common::f_binary_function!(atan2);
// ---------------------------------------------------------------------------
//
// atan2_forward_der
/// first order forward for atan2
///
fn atan2_forward_der <V, E>(
    dyp_both   :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    //
    for<'a> &'a E : Add<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    //
    for<'a> &'a E : Add<&'a V, Output = E> ,
    for<'a> &'a V : Add<&'a E, Output = E> ,
    for<'a> &'a V : Div<&'a E, Output = E> ,
    //
    for<'a> &'a E : FUnary<Output = E>,
    for<'a> &'a V : FUnary<Output = V>,
{
    debug_assert!( arg.len() == 2);
    let y          = arg[0] as usize;
    let x          = arg[1] as usize;
    let z          = res;
    match [arg_type[0], arg_type[1]] {
        //
        [ADType::Variable, ADType::Variable] => {
            let sum_sq     = &var_both[y].powi(2)  + &var_both[x].powi(2);
            let z_y        = &var_both[x] / &sum_sq;
            let z_x        = FUnary::minus( &(  &var_both[y] / &sum_sq ) );
            var_der[z]     = &( &z_y * &var_der[y] ) + &( &z_x * &var_der[x] );
        },
        [ADType::Variable, ADType::DynamicP] => {
            let sum_sq     = &var_both[y].powi(2)  + &dyp_both[x].powi(2);
            let z_y        = &dyp_both[x] / &sum_sq;
            var_der[z]     = &z_y * &var_der[y];
        },
        [ADType::Variable, ADType::ConstantP] => {
            let sum_sq     = &var_both[y].powi(2)  + &cop[x].powi(2);
            let z_y        = &cop[x] / &sum_sq;
            var_der[z]     = &z_y * &var_der[y];
        },
        [ADType::DynamicP, ADType::Variable] => {
            let sum_sq     = &dyp_both[y].powi(2)  + &var_both[x].powi(2);
            let z_x        = FUnary::minus( &(  &dyp_both[y] / &sum_sq ) );
            var_der[z]     = &z_x * &var_der[x];
        },
        [ADType::ConstantP, ADType::Variable] => {
            let sum_sq     = &cop[y].powi(2)  + &var_both[x].powi(2);
            let z_x        = FUnary::minus( &(  &cop[y] / &sum_sq ) );
            var_der[z]     = &z_x * &var_der[x];
        },
        _ => {
            panic!( "atan2_forward_der: unexpected value in arg_type");
        },
    }
}
// ---------------------------------------------------------------------------
//
// atan2_reverse_der
/// first order reverse for atan2
///
fn atan2_reverse_der <V, E>(
    dyp_both   :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a>     E : AddAssign<&'a E> ,
    //
    for<'a> &'a E : Add<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    //
    for<'a> &'a E : Add<&'a V, Output = E> ,
    for<'a> &'a V : Add<&'a E, Output = E> ,
    for<'a> &'a V : Div<&'a E, Output = E> ,
    //
    for<'a> &'a E : FUnary<Output = E>,
    for<'a> &'a V : FUnary<Output = V>,
{
    debug_assert!( arg.len() == 2);
    let y          = arg[0] as usize;
    let x          = arg[1] as usize;
    let z          = res;
    match [arg_type[0], arg_type[1]] {
        //
        [ADType::Variable, ADType::Variable] => {
            let sum_sq     = &var_both[y].powi(2)  + &var_both[x].powi(2);
            let z_y        = &var_both[x] / &sum_sq;
            let z_x        = FUnary::minus( &(  &var_both[y] / &sum_sq ) );
            var_der[x]    += &( &z_x * &var_der[z] );
            var_der[y]    += &( &z_y * &var_der[z] );
        },
        [ADType::Variable, ADType::DynamicP] => {
            let sum_sq     = &var_both[y].powi(2)  + &dyp_both[x].powi(2);
            let z_y        = &dyp_both[x] / &sum_sq;
            var_der[y]    += &( &z_y * &var_der[z] );
        },
        [ADType::Variable, ADType::ConstantP] => {
            let sum_sq     = &var_both[y].powi(2)  + &cop[x].powi(2);
            let z_y        = &cop[x] / &sum_sq;
            var_der[y]    += &( &z_y * &var_der[z] );
        },
        [ADType::DynamicP, ADType::Variable] => {
            let sum_sq     = &dyp_both[y].powi(2)  + &var_both[x].powi(2);
            let z_x        = FUnary::minus( &(  &dyp_both[y] / &sum_sq ) );
            var_der[x]    += &( &z_x * &var_der[z] );
        },
        [ADType::ConstantP, ADType::Variable] => {
            let sum_sq     = &cop[y].powi(2)  + &var_both[x].powi(2);
            let z_x        = FUnary::minus( &(  &cop[y] / &sum_sq ) );
            var_der[x]    += &( &z_x * &var_der[z] );
        },
        _ => {
            panic!( "atan2_reverse_der: unexpected value in arg_type");
        },
    }
}
// ---------------------------------------------------------------------------
// set_op_info
//
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The map results for ATAN2_OP
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    V : Clone + FConst + PartialEq + ThisThreadTape,
    //
    for<'a>         V : AddAssign<&'a V>,
    for<'a>     AD<V> : AddAssign<&'a AD<V>>,
    //
    for<'a> &'a V     : Add<&'a V, Output = V> ,
    for<'a> &'a V     : Add<&'a AD<V>, Output = AD<V>> ,
    for<'a> &'a AD<V> : Add<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    for<'a> &'a V     : Mul<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : Div<&'a V, Output = V> ,
    for<'a> &'a V     : Div<&'a AD<V>, Output = AD<V>> ,
    for<'a> &'a AD<V> : Div<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : FUnary<Output = V>,
    for<'a> &'a AD<V> : FUnary<Output = AD<V>>,
    //
    for<'a> &'a V     : FBinary<&'a V, Output = V>,
{
    op_info_vec[ATAN2_OP as usize] = OpInfo{
        name              : "atan2_pp",
        forward_dyp_value : atan2_forward_dyp::<V, V>,
        forward_dyp_ad    : atan2_forward_dyp::<V, AD<V> >,
        forward_var_value : atan2_forward_var::<V, V>,
        forward_var_ad    : atan2_forward_var::<V, AD<V> >,
        forward_der_value : atan2_forward_der::<V, V>,
        forward_der_ad    : atan2_forward_der::<V, AD<V> >,
        reverse_der_value : atan2_reverse_der::<V, V>,
        reverse_der_ad    : atan2_reverse_der::<V, AD<V> >,
        rust_src          : atan2_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
