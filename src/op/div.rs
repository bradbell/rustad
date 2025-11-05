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
    panic_var,
    panic_one,
    no_forward_dyp_value,
    no_forward_dyp_ad,
    no_forward_one_value,
    no_forward_one_ad,
    no_reverse_one_value,
    no_reverse_one_ad,
    no_rust_src,
};
use crate::op::id::{
    DIV_PP_OP,
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
no_forward_dyp_value!(Div);
no_forward_dyp_ad!(Div);
no_forward_one_value!(Div);
no_forward_one_ad!(Div);
no_reverse_one_value!(Div);
no_reverse_one_ad!(Div);
no_rust_src!(Div);
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
    op_info_vec[DIV_PP_OP as usize] = OpInfo{
        name              : "div_pp",
        forward_dyp_value : div_forward_dyp::<V, V>,
        forward_dyp_ad    : div_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_1_value   : panic_one::<V, V>,
        forward_1_ad      : panic_one::<V, AD<V> >,
        reverse_1_value   : panic_one::<V, V>,
        reverse_1_ad      : panic_one::<V, AD<V> >,
        rust_src          : rust_src_none,
        arg_var_index     : binary::binary_pp_arg_var_index,
    };
    op_info_vec[DIV_PV_OP as usize] = OpInfo{
        name              : "div_pv",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_var_value : div_pv_forward_0::<V, V>,
        forward_var_ad    : div_pv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : div_pv_rust_src,
        arg_var_index     : binary::binary_pv_arg_var_index,
    };
    op_info_vec[DIV_VP_OP as usize] = OpInfo{
        name              : "div_vp",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_var_value : div_vp_forward_0::<V, V>,
        forward_var_ad    : div_vp_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : div_vp_rust_src,
        arg_var_index     : binary::binary_vp_arg_var_index,
    };
    op_info_vec[DIV_VV_OP as usize] = OpInfo{
        name              : "div_vv",
        forward_dyp_value : forward_dyp_value_none::<V>,
        forward_dyp_ad    : forward_dyp_ad_none::<V>,
        forward_var_value : div_vv_forward_0::<V, V>,
        forward_var_ad    : div_vv_forward_0::<V, AD<V> >,
        forward_1_value   : forward_one_value_none::<V>,
        forward_1_ad      : forward_one_ad_none::<V>,
        reverse_1_value   : reverse_one_value_none::<V>,
        reverse_1_ad      : reverse_one_ad_none::<V>,
        rust_src          : div_vv_rust_src,
        arg_var_index     : binary::binary_vv_arg_var_index,
    };
}
