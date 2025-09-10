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
//!     * SUB_CV_OP : constant - variable
//!     * SUB_VC_OP : variable - constant
//!     * SUB_VV_OP : variable - variable
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
use crate::op::info::OpInfo;
use crate::op::info::panic_one;
use crate::op::id::{
    SUB_CV_OP,
    SUB_VC_OP,
    SUB_VV_OP,
};
// -------------------------------------------------------------------------
// sub_cv_forward_0
// sub_vc_forward_0
// sub_vv_forward_0
binary::eval_binary_forward_0!(Sub, -);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Sub operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for SUB_CV_OP, SUB_VC_OP, and SUB_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Sub<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Sub<&'a V, Output = V>          ,
    V             : Clone + ThisThreadTape                    ,
{
    op_info_vec[SUB_CV_OP as usize] = OpInfo{
        name              : "sub_cv",
        forward_0_value   : sub_cv_forward_0::<V, V>,
        forward_0_ad      : sub_cv_forward_0::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_cv_arg_var_index,
    };
    op_info_vec[SUB_VC_OP as usize] = OpInfo{
        name              : "sub_vc",
        forward_0_value   : sub_vc_forward_0::<V, V>,
        forward_0_ad      : sub_vc_forward_0::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_vc_arg_var_index,
    };
    op_info_vec[SUB_VV_OP as usize] = OpInfo{
        name              : "sub_vv",
        forward_0_value   : sub_vv_forward_0::<V, V>,
        forward_0_ad      : sub_vv_forward_0::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
