// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
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
use crate::{
    IndexT,
    AD,
    ADType,
};
//
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_dyp,
    panic_var,
    panic_one,
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
        ForwardOne,
        ReverseOne,
};
// -------------------------------------------------------------------------
// rust_src
// -------------------------------------------------------------------------
// mul_pv_rust_src
// mul_vp_rust_src
// mul_vv_rust_src
binary::binary_rust_src!(Mul, *);
// -------------------------------------------------------------------------
// forward_0
// -------------------------------------------------------------------------
// mul_pv_forward_0
// mul_vp_forward_0
// mul_vv_forward_0
binary::eval_binary_forward_0!(Mul, *);
// ---------------------------------------------------------------------------
// forward_1
// ---------------------------------------------------------------------------
//
// mul_pv_forward_1
/// first order forward for parameter * variable; see [ForwardOne]
fn mul_pv_forward_1 <V, E>(
    _var_both  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    cop        :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a V : std::ops::Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    var_one[ res ] = &cop[lhs] * &var_one[rhs];
}
//
// mul_vp_forward_1
/// first order forward for variable * parameter; see [ForwardOne]
fn mul_vp_forward_1 <V, E>(
    _var_both  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    cop        :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Mul<&'a V, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    var_one[ res ] = &var_one[lhs] * &cop[rhs];
}
//
// mul_vv_forward_1
/// first order forward for variable * variable; see [ForwardOne]
fn mul_vv_forward_1 <V, E>(
    var_both   :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
    for<'a> &'a E : std::ops::Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    let term1    = &var_both[lhs]  * &var_one[rhs];
    let term2    = &var_one[lhs]   * &var_both[rhs];
    var_one[res] = &term1 + &term2;
}
// ---------------------------------------------------------------------------
// reverse_1
// ---------------------------------------------------------------------------
//
// mul_pv_reverse_1
/// first order reverse for parameter * variable; see [ReverseOne]
fn mul_pv_reverse_1 <V, E>(
    _var_both  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    cop        :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> E : std::ops::AddAssign<&'a E> ,
    for<'a> &'a E : std::ops::Mul<&'a V, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    let term      = &var_one[res] * &cop[lhs];
    var_one[rhs] += &term;
}
//
// mul_vp_reverse_1
/// first order reverse for variable * parameter; see [ReverseOne]
fn mul_vp_reverse_1 <V, E>(
    _var_both  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    cop        :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> E : std::ops::AddAssign<&'a E> ,
    for<'a> &'a E : std::ops::Mul<&'a V, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    let term      = &var_one[res] * &cop[rhs];
    var_one[lhs] += &term;
}
//
// mul_vv_reverse_1
/// first order reverse for variable * variable; see [ReverseOne]
fn mul_vv_reverse_1 <V, E>(
    var_both   :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> E : std::ops::AddAssign<&'a E> ,
    for<'a> &'a E : std::ops::Mul<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    let term      = &var_one[res] * &var_both[rhs];
    var_one[lhs] += &term;
    //
    let term      = &var_one[res] * &var_both[lhs];
    var_one[rhs] += &term;
}
// ---------------------------------------------------------------------------
// set_op_info
//
no_rust_src!(Mul);
//
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for MUL_PV_OP, MUL_VP_OP, and MUL_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> V : std::ops::AddAssign<&'a V> ,
    //
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    //
    for<'a> &'a V : std::ops::Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Mul<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[MUL_PP_OP as usize] = OpInfo{
        name              : "mul_pp",
        forward_dyp_value : mul_forward_dyp::<V, V>,
        forward_dyp_ad    : mul_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        rust_src          : rust_src_none,
        arg_var_index     : binary::binary_pp_arg_var_index,
    };
    op_info_vec[MUL_PV_OP as usize] = OpInfo{
        name              : "mul_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_pv_forward_0::<V, V>,
        forward_var_ad    : mul_pv_forward_0::<V, AD<V> >,
        forward_1_value   : mul_pv_forward_1::<V, V>,
        forward_1_ad      : mul_pv_forward_1::<V, AD<V> >,
        reverse_1_value   : mul_pv_reverse_1::<V, V>,
        reverse_1_ad      : mul_pv_reverse_1::<V, AD<V> >,
        rust_src          : mul_pv_rust_src,
        arg_var_index     : binary::binary_pv_arg_var_index,
    };
    op_info_vec[MUL_VP_OP as usize] = OpInfo{
        name              : "mul_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_vp_forward_0::<V, V>,
        forward_var_ad    : mul_vp_forward_0::<V, AD<V> >,
        forward_1_value   : mul_vp_forward_1::<V, V>,
        forward_1_ad      : mul_vp_forward_1::<V, AD<V> >,
        reverse_1_value   : mul_vp_reverse_1::<V, V>,
        reverse_1_ad      : mul_vp_reverse_1::<V, AD<V> >,
        rust_src          : mul_vp_rust_src,
        arg_var_index     : binary::binary_vp_arg_var_index,
    };
    op_info_vec[MUL_VV_OP as usize] = OpInfo{
        name              : "mul_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : mul_vv_forward_0::<V, V>,
        forward_var_ad    : mul_vv_forward_0::<V, AD<V> >,
        forward_1_value   : mul_vv_forward_1::<V, V>,
        forward_1_ad      : mul_vv_forward_1::<V, AD<V> >,
        reverse_1_value   : mul_vv_reverse_1::<V, V>,
        reverse_1_ad      : mul_vv_reverse_1::<V, AD<V> >,
        rust_src          : mul_vv_rust_src,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
