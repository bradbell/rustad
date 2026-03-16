// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate Powf Operators
//!
//! Link to [parent module](super)
//!
// ---------------------------------------------------------------------------
// z    = powf(x, y)           = exp( log(x) * y )
// z_x  = powf(x, y) * y / x   = z * y / x
// z_y  = powf(x, y) * log(x)  = z * log(x)
// --------------------------------------------------------------------------
// use
//
use std::ops::{
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
use crate::op::id::POWF_OP;
use crate::tape::sealed::ThisThreadTape;
//
#[cfg(doc)]
use crate::op::info::{
        ForwardDer,
        ReverseDer,
};
// -------------------------------------------------------------------------
// powf_rust_src
// powf_forward_dyp
// powf_forward_var
common::f_binary_function!(powf);
// ---------------------------------------------------------------------------
//
// powf_forward_der
/// first order forward for powf
///
fn powf_forward_der <V, E>(
    dyp_both   :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a>     E : FConst + AddAssign<&'a E>,
    //
    for<'a> &'a E : Mul<&'a V, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    for<'a> &'a E : FUnary<Output = E>,
    //
    for<'a> &'a V : FUnary<Output = V>,
{
    debug_assert!( arg.len() == 2);
    let x          = arg[0] as usize;
    let y          = arg[1] as usize;
    let z          = res;
    let mut dz : E = FConst::zero();
    if arg_type[0].is_variable() {
        let z_x = match arg_type[1] {
            ADType::Variable => {
                &( &var_both[z] * &var_both[y] ) / &var_both[x]
            },
            ADType::DynamicP => {
                &( &var_both[z] * &dyp_both[y] ) / &var_both[x]
            },
            ADType::ConstantP => {
                &( &var_both[z] * &cop[y] ) / &var_both[x]
            },
            _ => {
                panic!("powf_forward_der: unexpected arg_type[1]");
            },
        };
        dz += &( &z_x * &var_der[x] );
    }
    if arg_type[1].is_variable() {
        let z_y = match  arg_type[0] {
            ADType::Variable => {
                &var_both[z] * &var_both[x].ln()
            },
            ADType::DynamicP => {
                &var_both[z] * &dyp_both[x].ln()
            },
            ADType::ConstantP => {
                &var_both[z] * &cop[x].ln()
            },
            _ => {
                panic!("powf_forward_der: unexpected arg_type[0]");
            },
        };
        dz += &( &z_y * &var_der[y] );
    }
    var_der[z] = dz;
}
// ---------------------------------------------------------------------------
//
// powf_reverse_der
/// first order reverse for powf
///
fn powf_reverse_der <V, E>(
    dyp_both   :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a>     E : FConst + AddAssign<&'a E>,
    //
    for<'a> &'a E : Mul<&'a V, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    for<'a> &'a E : FUnary<Output = E>,
    //
    for<'a> &'a V : FUnary<Output = V>,
{
    debug_assert!( arg.len() == 2);
    let x          = arg[0] as usize;
    let y          = arg[1] as usize;
    let z          = res;
    if arg_type[0].is_variable() {
        let z_x = match arg_type[1] {
            ADType::Variable => {
                &( &var_both[z] * &var_both[y] ) / &var_both[x]
            },
            ADType::DynamicP => {
                &( &var_both[z] * &dyp_both[y] ) / &var_both[x]
            },
            ADType::ConstantP => {
                &( &var_both[z] * &cop[y] ) / &var_both[x]
            },
            _ => {
                panic!("powf_forward_der: unexpected arg_type[1]");
            },
        };
        var_der[x] += &( &z_x * &var_der[z] );
    }
    if arg_type[1].is_variable() {
        let z_y = match  arg_type[0] {
            ADType::Variable => {
                &var_both[z] * &var_both[x].ln()
            },
            ADType::DynamicP => {
                &var_both[z] * &dyp_both[x].ln()
            },
            ADType::ConstantP => {
                &var_both[z] * &cop[x].ln()
            },
            _ => {
                panic!("powf_forward_der: unexpected arg_type[0]");
            },
        };
        var_der[y] += &( &z_y * &var_der[z] );
    }
}
// ---------------------------------------------------------------------------
// set_op_info
//
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The map results for POWF_OP
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    V : Clone + FConst + PartialEq + ThisThreadTape,
    //
    for<'a>         V : AddAssign<&'a V>,
    for<'a>     AD<V> : FConst + AddAssign<&'a AD<V>>,
    //
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    for<'a> &'a AD<V> : Mul<&'a V, Output = AD<V>> ,
    //
    for<'a> &'a V     : Mul<&'a AD<V>, Output = AD<V>> ,
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : Div<&'a V, Output = V> ,
    for<'a> &'a AD<V> : Div<&'a AD<V>, Output = AD<V>> ,
    //
    for<'a> &'a V     : FUnary<Output = V>,
    for<'a> &'a AD<V> : FUnary<Output = AD<V>>,
    //
    for<'a> &'a V     : FBinary<&'a V, Output = V>,
{
    op_info_vec[POWF_OP as usize] = OpInfo{
        name              : "powf_pp",
        forward_dyp_value : powf_forward_dyp::<V, V>,
        forward_dyp_ad    : powf_forward_dyp::<V, AD<V> >,
        forward_var_value : powf_forward_var::<V, V>,
        forward_var_ad    : powf_forward_var::<V, AD<V> >,
        forward_der_value : powf_forward_der::<V, V>,
        forward_der_ad    : powf_forward_der::<V, AD<V> >,
        reverse_der_value : powf_reverse_der::<V, V>,
        reverse_der_ad    : powf_reverse_der::<V, AD<V> >,
        rust_src          : powf_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
