// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the Mul operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
//!     * MUL_PV_OP : parameter * variable
//!     * MUL_VP_OP : variable * parameter
//!     * MUL_VV_OP : variable * variable
//!
//! * arg
//!     * arg\[0\]:  Variable or parameter index of left operand.
//!     * arg\[1\]:  Variable or parameter index of right operand.
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Add,
    Mul,
    AddAssign,
};
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FloatCore,
};
//
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_dyp,
    panic_var,
    panic_der,
    no_rust_src,
};
use crate::op::id::{
    MUL_PP_OP,
    MUL_PV_OP,
    MUL_VP_OP,
    MUL_VV_OP,
};
#[cfg(doc)]
use crate::op::info::{
        ForwardDer,
        ReverseDer,
};
// -------------------------------------------------------------------------
// rust_src
// -------------------------------------------------------------------------
// mul_pv_rust_src
// mul_vp_rust_src
// mul_vv_rust_src
binary::binary_rust_src!(Mul, *);
// -------------------------------------------------------------------------
// forward_var
// -------------------------------------------------------------------------
// mul_pv_forward_var
// mul_vp_forward_var
// mul_vv_forward_var
binary::eval_binary_forward_var!(Mul, *);
// ---------------------------------------------------------------------------
// forward_der
// ---------------------------------------------------------------------------
//
// mul_pv_forward_der
/// first order forward for parameter * variable; see [ForwardDer]
fn mul_pv_forward_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a V : Mul<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    if arg_type[0].is_constant() {
        var_der[ res ] = &cop[lhs] * &var_der[rhs];
    } else {
        debug_assert!( arg_type[0].is_dynamic() );
        var_der[ res ] = &dyp_both[lhs] * &var_der[rhs];
    }
}
//
// mul_vp_forward_der
/// first order forward for variable * parameter; see [ForwardDer]
fn mul_vp_forward_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Mul<&'a V, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    if arg_type[1].is_constant() {
        var_der[ res ] = &var_der[lhs] * &cop[rhs];
    } else {
        debug_assert!( arg_type[1].is_dynamic() );
        var_der[ res ] = &var_der[lhs] * &dyp_both[rhs];
    }
}
//
// mul_vv_forward_der
/// first order forward for variable * variable; see [ForwardDer]
fn mul_vv_forward_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : Add<&'a E, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    let term1    = &var_both[lhs]  * &var_der[rhs];
    let term2    = &var_der[lhs]   * &var_both[rhs];
    var_der[res] = &term1 + &term2;
}
// ---------------------------------------------------------------------------
// reverse_der
// ---------------------------------------------------------------------------
//
// mul_pv_reverse_der
/// first order reverse for parameter * variable; see [ReverseDer]
fn mul_pv_reverse_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E : AddAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a V, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    if arg_type[0].is_constant() {
        let term      = &var_der[res] * &cop[lhs];
        var_der[rhs] += &term;
    } else {
        debug_assert!( arg_type[0].is_dynamic() );
        let term      = &var_der[res] * &dyp_both[lhs];
        var_der[rhs] += &term;
    }
}
//
// mul_vp_reverse_der
/// first order reverse for variable * parameter; see [ReverseDer]
fn mul_vp_reverse_der <V, E>(
    dyp_both   :   &[E]        ,
    _var_both  :   &[E]        ,
    var_der    :   &mut [E]    ,
    cop        :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E : AddAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a V, Output = E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    if arg_type[1].is_constant() {
        let term      = &var_der[res] * &cop[rhs];
        var_der[lhs] += &term;
    } else {
        debug_assert!( arg_type[1].is_dynamic() );
        let term      = &var_der[res] * &dyp_both[rhs];
        var_der[lhs] += &term;
    }
}
//
// mul_vv_reverse_der
/// first order reverse for variable * variable; see [ReverseDer]
fn mul_vv_reverse_der <V, E>(
    _dyp_both  :   &[E]        ,
    var_both   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _flag_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> E : AddAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    let term      = &var_der[res] * &var_both[rhs];
    var_der[lhs] += &term;
    //
    let term      = &var_der[res] * &var_both[lhs];
    var_der[rhs] += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
//
// rust_src_none
no_rust_src!(Mul);
//
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The map results for MUL_PV_OP, MUL_VP_OP, and MUL_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    for<'a> V : AddAssign<&'a V> ,
    //
    for<'a> &'a V : Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : Add<&'a V, Output = V> ,
    //
    for<'a> &'a V : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : Mul<&'a V, Output = V> ,
    V             : Clone + FloatCore,
    V             : PartialEq + ThisThreadTape ,
{
    op_info_vec[MUL_PP_OP as usize] = OpInfo{
        name              : "mul_pp",
        forward_dyp_value : mul_forward_dyp::<V, V>,
        forward_dyp_ad    : mul_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[MUL_PV_OP as usize] = OpInfo{
        name              : "mul_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_pv_forward_var::<V, V>,
        forward_var_ad    : mul_pv_forward_var::<V, AD<V> >,
        forward_der_value : mul_pv_forward_der::<V, V>,
        forward_der_ad    : mul_pv_forward_der::<V, AD<V> >,
        reverse_der_value : mul_pv_reverse_der::<V, V>,
        reverse_der_ad    : mul_pv_reverse_der::<V, AD<V> >,
        rust_src          : mul_pv_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[MUL_VP_OP as usize] = OpInfo{
        name              : "mul_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_vp_forward_var::<V, V>,
        forward_var_ad    : mul_vp_forward_var::<V, AD<V> >,
        forward_der_value : mul_vp_forward_der::<V, V>,
        forward_der_ad    : mul_vp_forward_der::<V, AD<V> >,
        reverse_der_value : mul_vp_reverse_der::<V, V>,
        reverse_der_ad    : mul_vp_reverse_der::<V, AD<V> >,
        rust_src          : mul_vp_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[MUL_VV_OP as usize] = OpInfo{
        name              : "mul_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_vv_forward_var::<V, V>,
        forward_var_ad    : mul_vv_forward_var::<V, AD<V> >,
        forward_der_value : mul_vv_forward_der::<V, V>,
        forward_der_ad    : mul_vv_forward_der::<V, AD<V> >,
        reverse_der_value : mul_vv_reverse_der::<V, V>,
        reverse_der_ad    : mul_vv_reverse_der::<V, AD<V> >,
        rust_src          : mul_vv_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
}
