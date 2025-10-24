// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Sub operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
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
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::IndexT;
use crate::ad::AD;
use crate::op::info::{
    OpInfo,
    operator_does_not_implement,
};
use crate::op::id::{
    SUB_PV_OP,
    SUB_VP_OP,
    SUB_VV_OP,
};
// -------------------------------------------------------------------------
// sub_pv_rust_src
// sub_vp_rust_src
// sub_vv_rust_src
binary::binary_rust_src!(Sub, -);
// -------------------------------------------------------------------------
// sub_pv_forward_0
// sub_vp_forward_0
// sub_vv_forward_0
binary::eval_binary_forward_0!(Sub, -);
// ---------------------------------------------------------------------------
// forward_one_value_not_implemented
// forward_one_ad_not_implemented
// reverse_one_value_not_implemented
// reverse_one_ad_not_implemented
operator_does_not_implement!(Sub);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Sub operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for SUB_PV_OP, SUB_VP_OP, and SUB_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Sub<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Sub<&'a V, Output = V>          ,
    V             : Clone + ThisThreadTape                    ,
{
    op_info_vec[SUB_PV_OP as usize] = OpInfo{
        name              : "sub_cv",
        forward_0_value   : sub_pv_forward_0::<V, V>,
        forward_0_ad      : sub_pv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_pv_arg_var_index,
        rust_src          : sub_pv_rust_src,
    };
    op_info_vec[SUB_VP_OP as usize] = OpInfo{
        name              : "sub_vc",
        forward_0_value   : sub_vp_forward_0::<V, V>,
        forward_0_ad      : sub_vp_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_vp_arg_var_index,
        rust_src          : sub_vp_rust_src,
    };
    op_info_vec[SUB_VV_OP as usize] = OpInfo{
        name              : "sub_vv",
        forward_0_value   : sub_vv_forward_0::<V, V>,
        forward_0_ad      : sub_vv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_not_implemented::<V>,
        forward_1_ad      : forward_one_ad_not_implemented::<V>,
        reverse_1_value   : reverse_one_value_not_implemented::<V>,
        reverse_1_ad      : reverse_one_ad_not_implemented::<V>,
        arg_var_index     : binary::binary_vv_arg_var_index,
        rust_src          : sub_vv_rust_src,
    };
}
