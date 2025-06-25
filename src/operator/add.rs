// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD add operation:  [parent module](super)
//!
//! # Operator Id
//! ADD_CV_OP, ADD_VC_OP, or ADD_VV_OP
//!
//! # Operator Arguments
//! 1. arg\[0\]:  Variable or constant index of left operand.
//! 2. arg\[1\]:  Variable or constant index of left operand.
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
// float_forward_0_add_cv, ad_forward_0_add_cv
// float_forward_0_add_vc, ad_forward_0_add_vc
// float_forward_0_add_vv, ad_forward_0_add_vv
binary_op_forward_0!(Float, add, +);
binary_op_forward_0!(AD, add, +);
// ---------------------------------------------------------------------------
//
// float_forward_1_add_cv, ad_forward_1_add_cv
// float_forward_1_add_vc, ad_forward_1_add_vc
// float_forward_1_add_vv, ad_forward_1_add_vv
/// Implements first order forward for add operator
macro_rules! forward_1_add {
    ($Float_type:ident) => { paste::paste! {

        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order forward  constant + variable"
        ) ]
        fn [< $Float_type:lower _forward_1_add_cv >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[arg[1] as usize];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order forward  variable + constant"
        ) ]
        fn [< $Float_type:lower _forward_1_add_vc >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            var_one[ res ] = var_one[arg[0] as usize];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order forward  variable + variable"
        ) ]
        fn [< $Float_type:lower  _forward_1_add_vv >](
            var_one:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            var_one[ res ] =
                var_one[arg[0] as usize] + var_one[arg[1] as usize];
        }
    } };
}
forward_1_add!(Float);
forward_1_add!(AD);
// ---------------------------------------------------------------------------
//
// float_reverse_1_add_cv, ad_reverse_1_add_cv
// float_reverse_1_add_vc, ad_reverse_1_add_vc
// float_reverse_1_add_vv, ad_reverse_1_add_vv
/// Implements first order reverse for add operator
macro_rules! reverse_1_add {
    ($Float_type:ident) => { paste::paste! {

        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order reverse  constant + variable"
        ) ]
        fn [< $Float_type:lower _reverse_1_add_cv >](
            partial:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            partial[arg[1] as usize] =
                partial[arg[1] as usize] + partial[ res ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order reverse  variable + constant"
        ) ]
        fn [< $Float_type:lower _reverse_1_add_vc >](
            partial:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            partial[arg[0] as usize] =
                partial[arg[0] as usize] + partial[ res ];
        }
        #[doc = concat!(
            " ", stringify!($Float_type),
            " first order reverse  variable + variable"
        ) ]
        fn [< $Float_type:lower  _reverse_1_add_vv >](
            partial:   &mut Vec<$Float_type>,
            _var_zero: &Vec<$Float_type>,
            _con:      &Vec<Float>,
            arg:       &[Index],
            res:       usize)
        {
            debug_assert!( arg.len() == 2);
            partial[arg[0] as usize] =
                partial[arg[0] as usize] + partial[ res ];
            //
            partial[arg[1] as usize] =
                partial[arg[1] as usize] + partial[ res ];
        }
    } };
}
reverse_1_add!(Float);
reverse_1_add!(AD);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the add operators.
///
/// * op_info_vec :
/// The map from [operator::id] to operator information.
/// The the map results for ADD_CV_OP, ADD_VC_OP, and ADD_VV_OP are set.
pub(crate) fn set_op_info( op_info_vec : &mut Vec<OpInfo> ) {
    op_info_vec[ADD_CV_OP as usize] = OpInfo{
        name           : "add_cv".to_string() ,
        forward_0      : float_forward_0_add_cv,
        forward_1      : float_forward_1_add_cv,
        reverse_1      : float_reverse_1_add_cv,
        ad_forward_0   : ad_forward_0_add_cv,
        ad_forward_1   : ad_forward_1_add_cv,
        ad_reverse_1   : ad_reverse_1_add_cv,
        arg_var_index  : super::arg_var_index_binary_cv,
     };
    op_info_vec[ADD_VC_OP as usize] = OpInfo{
        name           : "add_vc".to_string(),
        forward_0      : float_forward_0_add_vc,
        forward_1      : float_forward_1_add_vc,
        reverse_1      : float_reverse_1_add_vc,
        ad_forward_0   : ad_forward_0_add_vc,
        ad_forward_1   : ad_forward_1_add_vc,
        ad_reverse_1   : ad_reverse_1_add_vc,
        arg_var_index  : super::arg_var_index_binary_vc,
    };
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name           : "add_vv".to_string(),
        forward_0      : float_forward_0_add_vv,
        forward_1      : float_forward_1_add_vv,
        reverse_1      : float_reverse_1_add_vv,
        ad_forward_0   : ad_forward_0_add_vv,
        ad_forward_1   : ad_forward_1_add_vv,
        ad_reverse_1   : ad_reverse_1_add_vv,
        arg_var_index  : super::arg_var_index_binary_vv,
    };
}
//
// AD + AD, Float + AD, AD + Float
crate::ad::binary_ad_operator!( Add, + );
//
// AD += AD, AD += Float
crate::ad::binary_ad_assign_op!( Add, += );
