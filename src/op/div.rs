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
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::IndexT;
use crate::ad::AD;
use crate::op::info::{
    OpInfo,
    no_forward_one_value,
    no_forward_one_ad,
    no_reverse_one_value,
    no_reverse_one_ad,
};
use crate::op::id::{
    DIV_PV_OP,
    DIV_VP_OP,
    DIV_VV_OP,
};
// -------------------------------------------------------------------------
// div_pv_rust_src
// div_vp_rust_src
// div_vv_rust_src
binary::binary_rust_src!(Div, /);
// -------------------------------------------------------------------------
// div_pv_forward_0
// div_vp_forward_0
// div_vv_forward_0
binary::eval_binary_forward_0!(Div, /);
// ---------------------------------------------------------------------------
// set_op_info
//
no_forward_one_value!(Div);
no_reverse_one_value!(Div);
no_forward_one_ad!(Div);
no_reverse_one_ad!(Div);
//
/// Set the operator information for all the Div operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for DIV_PV_OP, DIV_VP_OP, and DIV_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Div<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[DIV_PV_OP as usize] = OpInfo{
        name              : "div_pv",
        forward_0_value   : div_pv_forward_0::<V, V>,
        forward_0_ad      : div_pv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        arg_var_index     : binary::binary_pv_arg_var_index,
        rust_src          : div_pv_rust_src,
    };
    op_info_vec[DIV_VP_OP as usize] = OpInfo{
        name              : "div_vp",
        forward_0_value   : div_vp_forward_0::<V, V>,
        forward_0_ad      : div_vp_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        arg_var_index     : binary::binary_vp_arg_var_index,
        rust_src          : div_vp_rust_src,
    };
    op_info_vec[DIV_VV_OP as usize] = OpInfo{
        name              : "div_vv",
        forward_0_value   : div_vv_forward_0::<V, V>,
        forward_0_ad      : div_vv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        arg_var_index     : binary::binary_vv_arg_var_index,
        rust_src          : div_vv_rust_src,
    };
}
