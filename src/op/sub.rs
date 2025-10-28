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
    no_forward_dyp_value,
    no_forward_dyp_ad,
    no_forward_zero_value,
    no_forward_zero_ad,
    no_forward_one_value,
    no_forward_one_ad,
    no_reverse_one_value,
    no_reverse_one_ad,
    no_rust_src,
};
use crate::op::id::{
    SUB_PP_OP,
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
// set_op_info
//
no_forward_dyp_value!(Sub);
no_forward_dyp_ad!(Sub);
no_forward_zero_value!(Sub);
no_forward_zero_ad!(Sub);
no_forward_one_value!(Sub);
no_forward_one_ad!(Sub);
no_reverse_one_value!(Sub);
no_reverse_one_ad!(Sub);
no_rust_src!(Sub);
//
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
    op_info_vec[SUB_PP_OP as usize] = OpInfo{
        name              : "sub_pp",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_0_value   : forward_zero_value_none::<V>,
        forward_0_ad      : forward_zero_ad_none::<V>,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : rust_src_none,
        arg_var_index     : binary::binary_pp_arg_var_index,
    };
    op_info_vec[SUB_PV_OP as usize] = OpInfo{
        name              : "sub_pv",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_0_value   : sub_pv_forward_0::<V, V>,
        forward_0_ad      : sub_pv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : sub_pv_rust_src,
        arg_var_index     : binary::binary_pv_arg_var_index,
    };
    op_info_vec[SUB_VP_OP as usize] = OpInfo{
        name              : "sub_vp",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_0_value   : sub_vp_forward_0::<V, V>,
        forward_0_ad      : sub_vp_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : sub_vp_rust_src,
        arg_var_index     : binary::binary_vp_arg_var_index,
    };
    op_info_vec[SUB_VV_OP as usize] = OpInfo{
        name              : "sub_vv",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_0_value   : sub_vv_forward_0::<V, V>,
        forward_0_ad      : sub_vv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : sub_vv_rust_src,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
