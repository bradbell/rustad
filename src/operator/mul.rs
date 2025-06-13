// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD mul operator.
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::AD;
use crate::Float;
use crate::Index;
use crate::ad_tape::THIS_THREAD_TAPE;
use crate::ad_tape::Tape;
use crate::operator::OpInfo;
use crate::operator::id::{MUL_CV_OP, MUL_VC_OP, MUL_VV_OP};
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::ForwardZeroBinary;
//
// ---------------------------------------------------------------------------
// forward_0_mul_cv_fn
/// [ForwardZeroBinary] were op is *, left is constant, right is variable.
fn forward_0_mul_cv_fn(
    var_zero: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_zero[ res ] = con[ arg[0] ] * var_zero[ arg[1] ];
}
//
// forward_0_mul_vc_fn
/// [ForwardZeroBinary] were op is *, left is variable, right is constant.
fn forward_0_mul_vc_fn(
    var_zero: &mut Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_zero[ res ] = var_zero[ arg[0] ] * con[ arg[1] ];
}
//
// forward_0_mul_vv_fn
/// [ForwardZeroBinary] where op is *, left is variable, right is variable.
fn forward_0_mul_vv_fn(
    var_zero: &mut Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_zero[ res ] = var_zero[ arg[0] ] * var_zero[ arg[1] ];
}
//
// ---------------------------------------------------------------------------
// forward_1_mul_cv_fn
/// ForwardOneBinary were op is *, left is constant, right is variable.
fn forward_1_mul_cv_fn(var_one: &mut Vec<Float>,
    _var_zero: &Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_one[ res ] = con[ arg[0] ] * var_one[ arg[1] ];
}
//
// forward_1_mul_vc_fn
/// ForwardOneBinary were op is *, left is variable, right is constant.
fn forward_1_mul_vc_fn(var_one: &mut Vec<Float>,
    _var_zero: &Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_one[ res ] = var_one[ arg[0] ] * con[ arg[1] ];
}
//
// forward_1_mul_vv_fn
/// ForwardZeroBinary where op is *, left is variable, right is variable.
fn forward_1_mul_vv_fn(var_one: &mut Vec<Float>,
    var_zero: &Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    var_one[ res ]  = var_zero[ arg[0] ] * var_one[ arg[1] ];
    var_one[ res ] += var_one[ arg[0] ] * var_zero[ arg[1] ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the mul operators.
///
/// # op_info_vec
/// is a map from [operator::id] to operator information.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[MUL_CV_OP] = OpInfo{
        name      : "mul_cv".to_string() ,
        forward_0 : forward_0_mul_cv_fn,
        forward_1 : forward_1_mul_cv_fn,
     };
    op_info_vec[MUL_VC_OP] = OpInfo{
        name      : "mul_vc".to_string(),
        forward_0 : forward_0_mul_vc_fn,
        forward_1 : forward_1_mul_vc_fn,
    };
    op_info_vec[MUL_VV_OP] = OpInfo{
        name      : "mul_vv".to_string(),
        forward_0 : forward_0_mul_vv_fn,
        forward_1 : forward_1_mul_vv_fn,
    };
}
impl_binary_operator!( Mul, * );
