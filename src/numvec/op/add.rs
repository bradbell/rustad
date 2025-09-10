// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Add operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::numvec::doc_generic_v)
//! * E : see [doc_generic_e](crate::numvec::adfn::doc_generic_e)
//!
//! * [op::id](crate::numvec::op::id)
//!     * ADD_CV_OP : constant + variable
//!     * ADD_VC_OP : variable + constant
//!     * ADD_VV_OP : variable + variable
//!
//! * arg
//!     * arg\[0\]:  Variable or constant index of left operand.
//!     * arg\[1\]:  Variable or constant index of right operand.
// --------------------------------------------------------------------------
// use
//
use crate::numvec::op::binary;
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::IndexT;
use crate::numvec::ad::AD;
use crate::numvec::op::info::OpInfo;
use crate::numvec::op::id::{
    ADD_CV_OP,
    ADD_VC_OP,
    ADD_VV_OP,
};
// -------------------------------------------------------------------------
// add_cv_forward_0
// add_vc_forward_0
// add_vv_forward_0
binary::eval_binary_forward_0!(Add, +);
// ---------------------------------------------------------------------------
// forward_1
// ---------------------------------------------------------------------------
//
// add_cv_forward_1
/// first order forward for constant * variable; see [ForwardOne]
fn add_cv_forward_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    E             : Clone,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    var_one[ res ] = var_one[rhs].clone();
}
//
// add_vc_forward_1
/// first order forward for variable * constant; see [ForwardOne]
fn add_vc_forward_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    E             : Clone,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    var_one[ res ] = var_one[lhs].clone();
}
//
// add_vv_forward_1
/// first order forward for variable * variable; see [ForwardOne]
fn add_vv_forward_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    var_one[res] = &var_one[lhs]  + &var_one[rhs];
}
// ---------------------------------------------------------------------------
// reverse_1
// ---------------------------------------------------------------------------
//
// add_cv_reverse_1
/// first order reverse for constant * variable; see [ForwardOne]
fn add_cv_reverse_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    //
    // var_one[rhs] += &var_one[res];
    let sum      = &var_one[rhs] + &var_one[res];
    var_one[rhs] = sum;
}
//
// add_vc_reverse_1
/// first order reverse for variable * constant; see [ForwardOne]
fn add_vc_reverse_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    //
    // var_one[lhs] += &var_one[res];
    let sum      = &var_one[lhs] + &var_one[res];
    var_one[lhs] = sum;
}
//
// add_vv_reverse_1
/// first order reverse for variable * variable; see [ForwardOne]
fn add_vv_reverse_1 <V, E>(
    _var_zero  :   &Vec<E>     ,
    var_one    :   &mut Vec<E> ,
    _con       :   &Vec<V>     ,
    _flag      :   &Vec<bool>  ,
    arg        :   &[IndexT]   ,
    res        :       usize   )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    // var_one[lhs] += &var_one[res];
    // var_one[rhs] += &var_one[res];
    let sum      = &var_one[rhs] + &var_one[res];
    var_one[rhs] = sum;
    let sum      = &var_one[lhs] + &var_one[res];
    var_one[lhs] = sum;
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Add operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::numvec::op::id) to operator information.
/// The the map results for ADD_CV_OP, ADD_VC_OP, and ADD_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    for<'a> V     : Clone + ThisThreadTape + std::ops::AddAssign<&'a V>,
{
    op_info_vec[ADD_CV_OP as usize] = OpInfo{
        name              : "add_cv",
        forward_0_value   : add_cv_forward_0::<V, V>,
        forward_0_ad      : add_cv_forward_0::<V, AD<V> >,
        forward_1_value   : add_cv_forward_1::<V, V>,
        forward_1_ad      : add_cv_forward_1::<V, AD<V> >,
        reverse_1_value   : add_cv_reverse_1::<V, V>,
        reverse_1_ad      : add_cv_reverse_1::<V, AD<V> >,
        arg_var_index     : binary::binary_cv_arg_var_index,
    };
    op_info_vec[ADD_VC_OP as usize] = OpInfo{
        name              : "add_vc",
        forward_0_value   : add_vc_forward_0::<V, V>,
        forward_0_ad      : add_vc_forward_0::<V, AD<V> >,
        forward_1_value   : add_vc_forward_1::<V, V>,
        forward_1_ad      : add_vc_forward_1::<V, AD<V> >,
        reverse_1_value   : add_vc_reverse_1::<V, V>,
        reverse_1_ad      : add_vc_reverse_1::<V, AD<V> >,
        arg_var_index     : binary::binary_vc_arg_var_index,
    };
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name              : "add_vv",
        forward_0_value   : add_vv_forward_0::<V, V>,
        forward_0_ad      : add_vv_forward_0::<V, AD<V> >,
        forward_1_value   : add_vv_forward_1::<V, V>,
        forward_1_ad      : add_vv_forward_1::<V, AD<V> >,
        reverse_1_value   : add_vv_reverse_1::<V, V>,
        reverse_1_ad      : add_vv_reverse_1::<V, AD<V> >,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
