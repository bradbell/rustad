// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Div operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
//!     * DIV_CV_OP : constant / variable
//!     * DIV_VC_OP : variable / constant
//!     * DIV_VV_OP : variable / variable
//!
//! * arg
//!     * arg\[0\]:  Variable or constant index of left operand.
//!     * arg\[1\]:  Variable or constant index of right operand.
// --------------------------------------------------------------------------
// use
//
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::IndexT;
use crate::ad::AD;
use crate::op::info::{
    OpInfo,
    operator_does_not_implement,
};
use crate::op::id::{
    DIV_CV_OP,
    DIV_VC_OP,
    DIV_VV_OP,
};
// -------------------------------------------------------------------------
// div_cv_rust_src
// div_vc_rust_src
// div_vv_rust_src
binary::binary_rust_src!(Div, /);
// -------------------------------------------------------------------------
// div_cv_forward_0
// div_vc_forward_0
// div_vv_forward_0
binary::eval_binary_forward_0!(Div, /);
// ---------------------------------------------------------------------------
// forward_one_value_not_implemented
// forward_one_ad_not_implemented
// reverse_one_value_not_implemented
// reverse_one_ad_not_implemented
operator_does_not_implement!(Sub);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Div operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for DIV_CV_OP, DIV_VC_OP, and DIV_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Div<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[DIV_CV_OP as usize] = OpInfo{
        name              : "div_cv",
        forward_0_value   : div_cv_forward_0::<V, V>,
        forward_0_ad      : div_cv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_cv_arg_var_index,
        rust_src          : div_cv_rust_src,
    };
    op_info_vec[DIV_VC_OP as usize] = OpInfo{
        name              : "div_vc",
        forward_0_value   : div_vc_forward_0::<V, V>,
        forward_0_ad      : div_vc_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_vc_arg_var_index,
        rust_src          : div_vc_rust_src,
    };
    op_info_vec[DIV_VV_OP as usize] = OpInfo{
        name              : "div_vv",
        forward_0_value   : div_vv_forward_0::<V, V>,
        forward_0_ad      : div_vv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_vv_arg_var_index,
        rust_src          : div_vv_rust_src,
    };
}
