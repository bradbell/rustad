// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD add operator.
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::AD;
use crate::Float;
use crate::Index;
use crate::ad_tape::THIS_THREAD_TAPE;
use crate::ad_tape::Tape;
use crate::operator::OpInfo;
use crate::operator::id::{ADD_CV_OP, ADD_VC_OP, ADD_VV_OP};
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::{ForwardZeroBinary, ForwardOneBinary};
//
// float_forward_0_add_cv
// float_forward_0_add_vc
// float_forward_0_add_vv
binary_op_forward_0!(Float, add, +);
// ---------------------------------------------------------------------------
macro_rules! forward_1_add {
    ($Float_type:ident) => { paste::paste! {

        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  constant + variable"
        ) ]
        fn [< $Float_type:lower _forward_1_add_cv >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[ arg[1] ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  variable + constant"
        ) ]
        fn [< $Float_type:lower _forward_1_add_vc >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[ arg[0] ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " zero order forward  variable + variable"
        ) ]
        fn [< $Float_type:lower  _forward_1_add_vv >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<$Float_type>,
            arg:       &[Index],
            res:       Index) {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[ arg[0] ] + var_one[ arg[1] ];
        }
    } };
}
forward_1_add!(Float);
// ---------------------------------------------------------------------------
// reverse_1_add_cv_fn
/// [ForwardOneBinary]  were op is +, left is constant, right is variable.
fn reverse_1_add_cv_fn(partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[1] ] += partial[ res ];
}
//
// reverse_1_add_vc_fn
/// [ForwardOneBinary]  were op is +, left is variable, right is constant.
fn reverse_1_add_vc_fn(partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[0] ] += partial[ res ];
}
//
// reverse_1_add_vv_fn
/// [ForwardOneBinary]  where op is +, left is variable, right is variable.
fn reverse_1_add_vv_fn(partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, _con: &Vec<Float>, arg: &[Index], res: Index) {
    assert_eq!( arg.len(), 2);
    partial[ arg[0] ] += partial[ res ];
    partial[ arg[1] ] += partial[ res ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the add operators.
///
/// # op_info_vec
/// is a map from [operator::id] to operator information.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[ADD_CV_OP] = OpInfo{
        name         : "add_cv".to_string() ,
        forward_0    : float_forward_0_add_cv as super::ForwardZero,
        forward_1    : float_forward_1_add_cv as super::ForwardOne,
        reverse_1    : reverse_1_add_cv_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
     };
    op_info_vec[ADD_VC_OP] = OpInfo{
        name         : "add_vc".to_string(),
        forward_0    : float_forward_0_add_vc as super::ForwardZero,
        forward_1    : float_forward_1_add_vc as super::ForwardOne,
        reverse_1    : reverse_1_add_vc_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
    };
    op_info_vec[ADD_VV_OP] = OpInfo{
        name         : "add_vv".to_string(),
        forward_0    : float_forward_0_add_vv as super::ForwardZero,
        forward_1    : float_forward_1_add_vv as super::ForwardOne,
        reverse_1    : reverse_1_add_vv_fn,
        ad_forward_0 : super::ad_panic_zero,
        ad_forward_1 : super::ad_panic_one,
        ad_reverse_1 : super::ad_panic_one,
    };
}
impl_binary_operator!( Add, + );
