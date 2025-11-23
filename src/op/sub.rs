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
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
};
//
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_dyp,
    panic_var,
    panic_der,
    no_forward_der_value,
    no_forward_der_ad,
    no_reverse_der_value,
    no_reverse_der_ad,
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
// sub_forward_dyp
// sub_pv_forward_0
// sub_vp_forward_0
// sub_vv_forward_0
binary::eval_binary_forward_0!(Sub, -);
// ---------------------------------------------------------------------------
// set_op_info
//
no_forward_der_value!(Sub);
no_forward_der_ad!(Sub);
no_reverse_der_value!(Sub);
no_reverse_der_ad!(Sub);
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
    V             : Clone + From<f32> + PartialEq + ThisThreadTape ,
{
    op_info_vec[SUB_PP_OP as usize] = OpInfo{
        name              : "sub_pp",
        forward_dyp_value : sub_forward_dyp::<V, V>,
        forward_dyp_ad    : sub_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[SUB_PV_OP as usize] = OpInfo{
        name              : "sub_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_pv_forward_0::<V, V>,
        forward_var_ad    : sub_pv_forward_0::<V, AD<V> >,
        forward_der_value : forward_der_value_none::<V>,
        forward_der_ad    : forward_der_ad_none::<V>,
        reverse_der_value : reverse_der_value_none::<V>,
        reverse_der_ad    : reverse_der_ad_none::<V>,
        rust_src          : sub_pv_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[SUB_VP_OP as usize] = OpInfo{
        name              : "sub_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_vp_forward_0::<V, V>,
        forward_var_ad    : sub_vp_forward_0::<V, AD<V> >,
        forward_der_value : forward_der_value_none::<V>,
        forward_der_ad    : forward_der_ad_none::<V>,
        reverse_der_value : reverse_der_value_none::<V>,
        reverse_der_ad    : reverse_der_ad_none::<V>,
        rust_src          : sub_vp_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
    op_info_vec[SUB_VV_OP as usize] = OpInfo{
        name              : "sub_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : sub_vv_forward_0::<V, V>,
        forward_var_ad    : sub_vv_forward_0::<V, AD<V> >,
        forward_der_value : forward_der_value_none::<V>,
        forward_der_ad    : forward_der_ad_none::<V>,
        reverse_der_value : reverse_der_value_none::<V>,
        reverse_der_ad    : reverse_der_ad_none::<V>,
        rust_src          : sub_vv_rust_src,
        reverse_depend    : binary::reverse_depend,
    };
}
