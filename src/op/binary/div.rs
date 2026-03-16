// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the Div operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
//!     * DIV_PP_OP : parameter / parameter
//!     * DIV_PV_OP : parameter / variable
//!     * DIV_VP_OP : variable / parameter
//!     * DIV_VV_OP : variable / variable
//!
//! * arg
//!     * arg\[0\]:  Variable or parameter index of left operand.
//!     * arg\[1\]:  Variable or parameter index of right operand.
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Div,
    Mul,
    Sub,
    AddAssign,
    SubAssign,
};
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FConst,
    FUnary,
};
//
use crate::op::binary::common;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_dyp,
    panic_var,
    panic_der,
};
use crate::op::id::{
    DIV_PP_OP,
    DIV_PV_OP,
    DIV_VP_OP,
    DIV_VV_OP,
};
#[cfg(doc)]
use crate::op::info::{
        ForwardDer,
        ReverseDer,
};
// -------------------------------------------------------------------------
// div_rust_src
common::binary_rust_src!(div);
// -------------------------------------------------------------------------
// div_forward_dyp
// div_pv_forward_var
// div_vp_forward_var
// div_vv_forward_var
common::binary_arithmetic_function!(Div, div);
// ---------------------------------------------------------------------------
// forward_der
// ---------------------------------------------------------------------------
//
// div_pv_forward_der
/// first order forward for parameter / variable; see [ForwardDer]
fn div_pv_forward_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a V : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
{   // d(p / v) = - p * dv / (v * v) = -  (p / v) * dv / v
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_parameter() );
    debug_assert!( arg_type[1].is_variable() );
    let rhs        = arg[1] as usize;
    let numerator  = &var_both[res] * &var_der[rhs];
    var_der[res]   = (&numerator / &var_both[rhs]).minus()
}
//
// div_vp_forward_der
/// first order forward for variable / parameter; see [ForwardDer]
fn div_vp_forward_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Div<&'a V, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( arg_type[1].is_parameter() );
    let lhs       = arg[0] as usize;
    let rhs       = arg[1] as usize;
    if arg_type[1].is_constant() {
        var_der[ res ] = &var_der[lhs] / &cop[rhs];
    } else {
        debug_assert!( arg_type[1].is_dynamic() );
        var_der[ res ] = &var_der[lhs] / &dyp_both[rhs];
    }
}
//
// div_vv_forward_der
/// first order forward for variable / variable; see [ForwardDer]
fn div_vv_forward_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a V : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    for<'a> &'a E : Sub<&'a E, Output = E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
{   // d(u / v) = ( v * du - u * dv ) / (v * v) = [ du - (u / v) * dv ] / v
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( arg_type[1].is_variable() );
    let lhs       = arg[0] as usize;
    let rhs       = arg[1] as usize;
    let numerator = &var_der[lhs] - &( &var_both[res] * &var_der[rhs] );
    var_der[res]  = &numerator / &var_both[rhs];
}
// ---------------------------------------------------------------------------
// reverse_der
// ---------------------------------------------------------------------------
//
// div_pv_reverse_der
/// first order reverse for parameter / variable; see [ReverseDer]
fn div_pv_reverse_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : SubAssign<&'a E>       ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    // g(v)      = f(w, v) = f[ p / v, v ]
    // dg / dv   = df/dv + df/dw * dw/dv
    // dw / dv   = - w / v
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_parameter() );
    debug_assert!( arg_type[1].is_variable() );
    let rhs = arg[1] as usize;
    let term      = &var_der[res] * &( &var_both[res] / &var_both[rhs] );
    var_der[rhs] -= &term;
}
//
// div_vp_reverse_der
/// first order reverse for variable / parameter; see [ReverseDer]
fn div_vp_reverse_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E     : AddAssign<&'a E>       ,
    for<'a> &'a E : Div<&'a V, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
{
    // g(v)      = f(w, v) = f[ v / p, v ]
    // dg / dv   = df/dv + df/dw * dw/dv
    // dw / dv   = 1 / p
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( arg_type[1].is_parameter() );
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    let term = if arg_type[1].is_constant() {
        &var_der[res] / &cop[rhs]
    } else {
        debug_assert!( arg_type[1].is_dynamic() );
        &var_der[res] / &dyp_both[rhs]
    };
    var_der[lhs] += &term;
}
//
// div_vv_reverse_der
/// first order reverse for variable / variable; see [ReverseDer]
fn div_vv_reverse_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E : AddAssign<&'a E> ,
    for<'a> E : SubAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Div<&'a E, Output = E> ,
{
    // g(u, v)   = f(w, u, v) = f[ u / v, u, v ]
    // dg / du   = df/du + df/dw * dw/du
    // dg / dv   = df/dv + df/dw * dw/dv
    // dw / du   = 1 / v,  dw / dv   = - w / v
    debug_assert!( arg.len() == 2);
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( arg_type[1].is_variable() );
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    let term      = &var_der[res] / &var_both[rhs];
    var_der[lhs] += &term;
    //
    let term      = &var_der[res] * &( &var_both[res] / &var_both[rhs] );
    var_der[rhs] -= &term;
}
// ---------------------------------------------------------------------------
// set_op_info
//
/// Set the operator information for all the Div operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for
///   DIV_PP_OP, DIV_PV_OP, DIV_VP_OP, and DIV_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    for<'a> V     : SubAssign<&'a V> ,
    for<'a> V     : AddAssign<&'a V> ,
    //
    for<'a> &'a V : Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : Sub<&'a AD<V>, Output = AD<V> > ,
    //
    for<'a> &'a V : Div<&'a V, Output = V> ,
    for<'a> &'a V : Mul<&'a V, Output = V> ,
    for<'a> &'a V : Sub<&'a V, Output = V> ,
    V             : Clone + FConst ,
    for<'a> &'a V : FUnary<Output=V>,
    V             : PartialEq + ThisThreadTape ,
{
    op_info_vec[DIV_PP_OP as usize] = OpInfo{
        name              : "div_pp",
        forward_dyp_value : div_forward_dyp::<V, V>,
        forward_dyp_ad    : div_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : div_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[DIV_PV_OP as usize] = OpInfo{
        name              : "div_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : div_pv_forward_var::<V, V>,
        forward_var_ad    : div_pv_forward_var::<V, AD<V> >,
        forward_der_value : div_pv_forward_der::<V, V>,
        forward_der_ad    : div_pv_forward_der::<V, AD<V> >,
        reverse_der_value : div_pv_reverse_der::<V, V>,
        reverse_der_ad    : div_pv_reverse_der::<V, AD<V> >,
        rust_src          : div_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[DIV_VP_OP as usize] = OpInfo{
        name              : "div_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : div_vp_forward_var::<V, V>,
        forward_var_ad    : div_vp_forward_var::<V, AD<V> >,
        forward_der_value : div_vp_forward_der::<V, V>,
        forward_der_ad    : div_vp_forward_der::<V, AD<V> >,
        reverse_der_value : div_vp_reverse_der::<V, V>,
        reverse_der_ad    : div_vp_reverse_der::<V, AD<V> >,
        rust_src          : div_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[DIV_VV_OP as usize] = OpInfo{
        name              : "div_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : div_vv_forward_var::<V, V>,
        forward_var_ad    : div_vv_forward_var::<V, AD<V> >,
        forward_der_value : div_vv_forward_der::<V, V>,
        forward_der_ad    : div_vv_forward_der::<V, AD<V> >,
        reverse_der_value : div_vv_reverse_der::<V, V>,
        reverse_der_ad    : div_vv_reverse_der::<V, AD<V> >,
        rust_src          : div_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
