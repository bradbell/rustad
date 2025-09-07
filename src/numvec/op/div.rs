// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Div operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::numvec::doc_generic_v)
//! * E : see [doc_generic_e](crate::numvec::adfn::doc_generic_e)
//!
//! * [op::id](crate::numvec::op::id)
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
use crate::numvec::op::binary;
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::tape::IndexT;
use crate::numvec::ad::AD;
use crate::numvec::op::info::OpInfo;
use crate::numvec::op::info::panic_one;
use crate::numvec::op::id::{
    DIV_CV_OP,
    DIV_VC_OP,
    DIV_VV_OP,
};
// -------------------------------------------------------------------------
// div_cv_forward_0
// div_vc_forward_0
// div_vv_forward_0
binary::eval_binary_forward_0!(Div, /);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Div operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::numvec::op::id) to operator information.
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
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_cv_arg_var_index,
    };
    op_info_vec[DIV_VC_OP as usize] = OpInfo{
        name              : "div_vc",
        forward_0_value   : div_vc_forward_0::<V, V>,
        forward_0_ad      : div_vc_forward_0::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_vc_arg_var_index,
    };
    op_info_vec[DIV_VV_OP as usize] = OpInfo{
        name              : "div_vv",
        forward_0_value   : div_vv_forward_0::<V, V>,
        forward_0_ad      : div_vv_forward_0::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
