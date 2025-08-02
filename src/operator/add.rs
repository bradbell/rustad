// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD add operation
//! : [parent module](super)
//!
//! * F : Floating point type used for value calculations .
//! * U : Unsigned integer type used for indices in the Tape .
//! * E : Evaluation type which is either F, or GAD<F,U> .
//!
//! # Operator Id
//! ADD_CV_OP, ADD_VC_OP, or ADD_VV_OP
//!
//! # Operator Arguments
//! 1. arg\[0\]:  Variable or constant index of left operand.
//! 2. arg\[1\]:  Variable or constant index of left operand.
//
use std::cell::RefCell;
use std::thread::LocalKey;
//
// BEGIN_SORT_THIS_LINE_PLUS_1
use crate::ad::GAD;
use crate::operator::OpInfo;
use crate::operator::binary_op_forward_0;
use crate::operator::id::{ADD_CV_OP, ADD_VC_OP, ADD_VV_OP};
use crate::ptrait::GenericAs;
use crate::record::GTape;
use crate::record::sealed::ThisThreadTape;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::{ForwardZeroBinary, ForwardOneBinary};
#[cfg(doc)]
use crate::ad::doc_binary_ad_operator;
//
// forward_0_add_cv<F, U, E>
// forward_0_add_vc<F, U, E>
// forward_0_add_vv<F, U, E>
binary_op_forward_0!(Add, +);
// ---------------------------------------------------------------------------
//
// forward_1_add_cv
/// Implements first order forward: constant + variable
fn forward_1_add_cv <F, U, E>(
    var_one:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy ,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] = var_one[ GenericAs::gas(arg[1]) ];
}
//
// forward_1_add_vc
/// Implements first order forward: variable + constant
fn forward_1_add_vc <F, U, E>(
    var_one:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy ,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] = var_one[ GenericAs::gas(arg[0]) ];
}
//
// forward_1_add_vv
/// Implements first order forward: variable + variable
fn forward_1_add_vv <F, U, E>(
    var_one:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] =
        var_one[ GenericAs::gas(arg[0]) ] + var_one[ GenericAs::gas(arg[1]) ];
}
// ---------------------------------------------------------------------------
//
// reverse_1_add_cv
/// Implements first order reverse: constant + variable
fn reverse_1_add_cv <F, U, E>(
    partial:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[1]) ] =
        partial[ GenericAs::gas(arg[1]) ] + partial[ res ];
}
//
// reverse_1_add_vc
/// Implements first order reverse: variable + constant
fn reverse_1_add_vc <F, U, E>(
    partial:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[0]) ] =
        partial[ GenericAs::gas(arg[0]) ] + partial[ res ];
}
//
// reverse_1_add_vv
/// Implements first order reverse: variable + variable
fn reverse_1_add_vv <F, U, E>(
    partial:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[0]) ] =
        partial[ GenericAs::gas(arg[0]) ] + partial[ res ];
    //
    partial[ GenericAs::gas(arg[1]) ] =
        partial[ GenericAs::gas(arg[1]) ] + partial[ res ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the add operators.
///
/// * op_info_vec :
/// The map from [operator::id] to operator information.
/// The the map results for ADD_CV_OP, ADD_VC_OP, and ADD_VV_OP are set.
pub(crate) fn set_op_info<F,U>( op_info_vec : &mut Vec< OpInfo<F,U> > )
where
    usize    : GenericAs<U> ,
    U        : Copy + 'static + GenericAs<usize> ,
    F        : Copy + 'static +
        std::ops::Add<F, Output = F> +
        std::ops::Add< GAD<F,U>, Output =  GAD<F,U> > +
        ThisThreadTape<U> ,
    GAD<F,U> : Copy +
        std::ops::Add< F, Output = GAD<F,U> > +
        std::ops::Add< GAD<F,U>, Output =  GAD<F,U> > ,
{
    op_info_vec[ADD_CV_OP as usize] = OpInfo{
        name           : "add_cv".to_string() ,
        forward_0      : forward_0_add_cv,
        forward_1      : forward_1_add_cv,
        reverse_1      : reverse_1_add_cv,
        ad_forward_0   : forward_0_add_cv,
        ad_forward_1   : forward_1_add_cv,
        ad_reverse_1   : reverse_1_add_cv,
        arg_var_index  : super::arg_var_index_binary_cv,
     };
    op_info_vec[ADD_VC_OP as usize] = OpInfo{
        name           : "add_vc".to_string(),
        forward_0      : forward_0_add_vc,
        forward_1      : forward_1_add_vc,
        reverse_1      : reverse_1_add_vc,
        ad_forward_0   : forward_0_add_vc,
        ad_forward_1   : forward_1_add_vc,
        ad_reverse_1   : reverse_1_add_vc,
        arg_var_index  : super::arg_var_index_binary_vc,
    };
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name           : "add_vv".to_string(),
        forward_0      : forward_0_add_vv,
        forward_1      : forward_1_add_vv,
        reverse_1      : reverse_1_add_vv,
        ad_forward_0   : forward_0_add_vv,
        ad_forward_1   : forward_1_add_vv,
        ad_reverse_1   : reverse_1_add_vv,
        arg_var_index  : super::arg_var_index_binary_vv,
    };
}
//
// AD + AD, Float + AD, AD + Float
crate::ad::binary_ad_operator!( Add, + );
//
// AD += AD, AD += Float
crate::ad::binary_ad_assign_op!( Add, += );
