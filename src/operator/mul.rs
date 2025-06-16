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
// float_forward_0_mul_cv
// float_forward_0_mul_vc
// float_forward_0_mul_vv
binary_op_forward_0!(Float, mul, *);
// ---------------------------------------------------------------------------
macro_rules! forward_1_mul {
    ($Float_type:ident) => { paste::paste! {

        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  constant * variable"
        ) ]
        fn [< $Float_type:lower _forward_1_mul_cv >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            con:       &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = con[ arg[0] ] * var_one[ arg[1] ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  variable * constant"
        ) ]
        fn [< $Float_type:lower _forward_1_mul_vc >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            con:       &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[ arg[0] ] * con[ arg[1] ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  variable * variable"
        ) ]
        fn [< $Float_type:lower  _forward_1_mul_vv >](
            var_one:   &mut Vec<$Float_type>,
            var_zero:  &Vec<$Float_type>,
            _con:      &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_zero[ arg[0] ] * var_one[ arg[1] ]
                           + var_one[ arg[0] ] * var_zero[ arg[1] ];
        }
    } };
}
forward_1_mul!(Float);
// ---------------------------------------------------------------------------
// reverse_1_mul_cv_fn
/// ForwardOneBinary were op is *, left is constant, right is variable.
fn reverse_1_mul_cv_fn(partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[1] ] += partial[res] * con[ arg[0] ];
}
//
// reverse_1_mul_vc_fn
/// ForwardOneBinary were op is *, left is variable, right is constant.
fn reverse_1_mul_vc_fn(partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[0] ] += partial[res] * con[ arg[0] ];
}
//
// reverse_1_mul_vv_fn
/// ForwardZeroBinary where op is *, left is variable, right is variable.
fn reverse_1_mul_vv_fn(partial: &mut Vec<Float>,
    var_zero: &Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[0] ] += partial[res] * var_zero[ arg[1] ];
    partial[ arg[1] ] += partial[res] * var_zero[ arg[0] ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the mul operators.
///
/// # op_info_vec
/// is a map from [operator::id] to operator information.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[MUL_CV_OP] = OpInfo{
        name         : "mul_cv".to_string() ,
        forward_0    : float_forward_0_mul_cv,
        forward_1    : float_forward_1_mul_cv,
        reverse_1    : reverse_1_mul_cv_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
     };
    op_info_vec[MUL_VC_OP] = OpInfo{
        name         : "mul_vc".to_string(),
        forward_0    : float_forward_0_mul_vc,
        forward_1    : float_forward_1_mul_vc,
        reverse_1    : reverse_1_mul_vc_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
    };
    op_info_vec[MUL_VV_OP] = OpInfo{
        name         : "mul_vv".to_string(),
        forward_0    : float_forward_0_mul_vv,
        forward_1    : float_forward_1_mul_vv,
        reverse_1    : reverse_1_mul_vv_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
    };
}
impl_binary_operator!( Mul, * );
