// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the Sub operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
//!     * SUB_PP_OP : parameter - parameter
//!     * SUB_PV_OP : parameter - variable
//!     * SUB_VP_OP : variable - parameter
//!     * SUB_VV_OP : variable - variable
//!
//! * arg
//!     * arg\[0\]:  Variable or parameter index of left operand.
//!     * arg\[1\]:  Variable or parameter index of right operand.
// --------------------------------------------------------------------------
// use
//
use std::ops::Add;
use std::ops::Sub;
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FConst,
};
//
use crate::op::binary::common;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpFns,

    ConstData,
    panic_dyp,
    panic_var,
    panic_der,
};
use crate::op::id::{
    SUB_PP_OP,
    SUB_PV_OP,
    SUB_VP_OP,
    SUB_VV_OP,
};
#[cfg(doc)]
use crate::op::info::{
    ForwardDer,
    ReverseDer,
};
// -------------------------------------------------------------------------
// sub_rust_src
common::binary_rust_src!(sub);
// -------------------------------------------------------------------------
// sub_forward_dyp
// sub_pv_forward_var
// sub_vp_forward_var
// sub_vv_forward_var
common::binary_arithmetic_function!(Sub, sub);
// ---------------------------------------------------------------------------
// forward_der
// ---------------------------------------------------------------------------
//
// sub_pv_forward_der
/// first order forward for parameter - variable; see [ForwardDer]
fn sub_pv_forward_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    E  : Clone + FConst ,
    V  : FConst ,
    for<'a> &'a E : Sub<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    // TODO: use unary minus once it is defined for AD<V>.
    let zero       = E::zero();
    var_der[ res ] = &zero - &var_der[rhs];
}
//
// sub_vp_forward_der
/// first order forward for variable - parameter; see [ForwardDer]
fn sub_vp_forward_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    E             : Clone,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    var_der[ res ] = var_der[lhs].clone();
}
//
// sub_vv_forward_der
/// first order forward for variable - variable; see [ForwardDer]
fn sub_vv_forward_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Sub<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    var_der[res] = &var_der[lhs]  - &var_der[rhs];
}
// ---------------------------------------------------------------------------
// reverse_der
// ---------------------------------------------------------------------------
//
// sub_pv_reverse_der
/// first order reverse for parameter - variable; see [ReverseDer]
fn sub_pv_reverse_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Sub<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    //
    // var_der[rhs] += &var_der[res];
    let diff     = &var_der[rhs] - &var_der[res];
    var_der[rhs] = diff;
}
//
// sub_vp_reverse_der
/// first order reverse for variable - parameter; see [ReverseDer]
fn sub_vp_reverse_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    //
    // var_der[lhs] += &var_der[res];
    let sum      = &var_der[lhs] + &var_der[res];
    var_der[lhs] = sum;
}
//
// sub_vv_reverse_der
/// first order reverse for variable - variable; see [ReverseDer]
fn sub_vv_reverse_der <V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Add<&'a E, Output = E> + Sub<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    // var_der[lhs] += &var_der[res];
    // var_der[rhs] += &var_der[res];
    let diff     = &var_der[rhs] - &var_der[res];
    var_der[rhs] = diff;
    let sum      = &var_der[lhs] + &var_der[res];
    var_der[lhs] = sum;
}
// set_op_fns
//
/// Set the operator functions for all the Sub operators.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for
///   SUB_PP_OP, SUB_PV_OP, SUB_VP_OP, and SUB_VV_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] )
where
    for<'a> &'a V : Add<&'a V, Output = V> + Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : Sub<&'a V, Output = V> + Sub<&'a AD<V>, Output = AD<V> > ,
    V             : Clone + FConst ,
    V             : PartialEq + ThisThreadTape ,
    AD<V>         : From<V>
{
    op_fns_vec[SUB_PP_OP as usize] = OpFns{
        name              : "sub_pp",
        forward_dyp_value : sub_forward_dyp::<V, V>,
        forward_dyp_ad    : sub_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : sub_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[SUB_PV_OP as usize] = OpFns{
        name              : "sub_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_pv_forward_var::<V, V>,
        forward_var_ad    : sub_pv_forward_var::<V, AD<V> >,
        forward_der_value : sub_pv_forward_der::<V, V>,
        forward_der_ad    : sub_pv_forward_der::<V, AD<V>>,
        reverse_der_value : sub_pv_reverse_der::<V, V>,
        reverse_der_ad    : sub_pv_reverse_der::<V, AD<V> >,
        rust_src          : sub_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[SUB_VP_OP as usize] = OpFns{
        name              : "sub_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_vp_forward_var::<V, V>,
        forward_var_ad    : sub_vp_forward_var::<V, AD<V> >,
        forward_der_value : sub_vp_forward_der::<V, V>,
        forward_der_ad    : sub_vp_forward_der::<V, AD<V> >,
        reverse_der_value : sub_vp_reverse_der::<V, V>,
        reverse_der_ad    : sub_vp_reverse_der::<V, AD<V> >,
        rust_src          : sub_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[SUB_VV_OP as usize] = OpFns{
        name              : "sub_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_vv_forward_var::<V, V>,
        forward_var_ad    : sub_vv_forward_var::<V, AD<V> >,
        forward_der_value : sub_vv_forward_der::<V, V>,
        forward_der_ad    : sub_vv_forward_der::<V, AD<V> >,
        reverse_der_value : sub_vv_reverse_der::<V, V>,
        reverse_der_ad    : sub_vv_reverse_der::<V, AD<V> >,
        rust_src          : sub_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
