// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Mul operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::numvec::doc_generic_v)
//! * E : see [doc_generic_e](crate::numvec::adfn::doc_generic_e)
//!
//! * [op::id](crate::numvec::op::id)
//!     * MUL_CV_OP : constant * variable
//!     * MUL_VC_OP : variable * constant
//!     * MUL_VV_OP : variable * variable
//!
//! * arg
//!     * arg\[0\]:  Variable or constant index of left operand.
//!     * arg\[1\]:  Variable or constant index of right operand.
// --------------------------------------------------------------------------
// use
//
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::tape::Tindex;
use crate::numvec::ad::AD;
use crate::numvec::op::binary::eval_binary_forward_0;
use crate::numvec::op::info::OpInfo;
use crate::numvec::op::id::{
    MUL_CV_OP,
    MUL_VC_OP,
    MUL_VV_OP,
};
// -------------------------------------------------------------------------
// mul_cv_forward_0
// mul_vc_forward_0
// mul_vv_forward_0
eval_binary_forward_0!(Mul, *);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the Mul operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::numvec::op::id) to operator information.
/// The the map results for MUL_CV_OP, MUL_VC_OP, and MUL_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Mul<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[MUL_CV_OP as usize] = OpInfo{
        name              : "mul_cv",
        forward_0_value   : mul_cv_forward_0::<V, V>,
        forward_0_ad      : mul_cv_forward_0::<V, AD<V> >,
    };
    op_info_vec[MUL_VC_OP as usize] = OpInfo{
        name              : "mul_vc",
        forward_0_value   : mul_vc_forward_0::<V, V>,
        forward_0_ad      : mul_vc_forward_0::<V, AD<V> >,
    };
    op_info_vec[MUL_VV_OP as usize] = OpInfo{
        name              : "mul_vv",
        forward_0_value   : mul_vv_forward_0::<V, V>,
        forward_0_ad      : mul_vv_forward_0::<V, AD<V> >,
    };
}
