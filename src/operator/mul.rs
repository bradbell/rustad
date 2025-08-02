// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Store and compute for AD mul operation
//! : [parent module](super)
//!
//! * F : Floating point type used for value calculations .
//! * U : Unsigned integer type used for indices in the Tape .
//! * E : Evaluation type which is either F, or GAD<F,U> .
//!
//! # Operator Id
//! MUL_CV_OP, MUL_VC_OP, or MUL_VV_OP
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
use crate::operator::id::{MUL_CV_OP, MUL_VC_OP, MUL_VV_OP};
use crate::ptrait::GenericAs;
use crate::record::GTape;
use crate::record::sealed::ThisThreadTape;
// END_SORT_THIS_LINE_MINUS_1
//
#[cfg(doc)]
use crate::operator;
#[cfg(doc)]
use crate::operator::ForwardZeroBinary;
#[cfg(doc)]
use crate::ad::doc_binary_ad_operator;
//
// forward_0_mul_cv<F, U, E>
// forward_0_mul_vc<F, U, E>
// forward_0_mul_vv<F, U, E>
binary_op_forward_0!(Mul, *);
// ---------------------------------------------------------------------------
//
// forward_1_mul_cv
/// Implements first order forward: constant * variable
fn forward_1_mul_cv <F, U, E>(
    var_one:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    con:       &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    F : Copy + std::ops::Mul<E, Output=E> ,
    E : Copy + std::ops::Mul<E, Output=E> ,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] =
        con[ GenericAs::gas(arg[0]) ] * var_one[ GenericAs::gas(arg[1]) ];
}
//
// forward_1_mul_vc
/// Implements first order forward: variable * constant
fn forward_1_mul_vc <F, U, E>(
    var_one:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    con:       &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    F : Copy ,
    E : Copy + std::ops::Mul<F, Output=E> ,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] =
        var_one[ GenericAs::gas(arg[0]) ] * con[ GenericAs::gas(arg[1]) ];
}
//
// forward_1_mul_vv
/// Implements first order forward: variable * variable
fn forward_1_mul_vv <F, U, E>(
    var_one:   &mut Vec<E>,
    var_zero:  &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Mul<E, Output=E> + std::ops::Add<Output=E> ,
{
    debug_assert!( arg.len() == 2);
    var_one[ res ] =
        var_zero[ GenericAs::gas(arg[0]) ] * var_one[ GenericAs::gas(arg[1]) ]
        + var_one[ GenericAs::gas(arg[0]) ] * var_zero[ GenericAs::gas(arg[1]) ];
}
// ---------------------------------------------------------------------------
//
// reverse_1_mul_cv
/// Implements first order reverse: constant * variable
fn reverse_1_mul_cv <F, U, E>(
    partial:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    con:       &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    F : Copy ,
    E : Copy + std::ops::Mul<F, Output=E> + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[1]) ] = partial[ GenericAs::gas(arg[1]) ] +
        partial[res] * con[ GenericAs::gas(arg[0]) ];
}
//
// reverse_1_mul_vc
/// Implements first order reverse: variable * constant
fn reverse_1_mul_vc <F, U, E>(
    partial:   &mut Vec<E>,
    _var_zero: &Vec<E>,
    con:       &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    F : Copy ,
    E : Copy + std::ops::Mul<F, Output = E> + std::ops::Add<Output = E>,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[0]) ] = partial[ GenericAs::gas(arg[0]) ] +
        partial[res] * con[ GenericAs::gas(arg[1]) ];
}
//
// reverse_1_mul_vv
/// Implements first order reverse: variable * variable
fn reverse_1_mul_vv <F, U, E>(
    partial:   &mut Vec<E>,
    var_zero:  &Vec<E>,
    _con:      &Vec<F>,
    arg:       &[U],
    res:       usize)
where
    U : Copy + GenericAs<usize> ,
    E : Copy + std::ops::Mul<Output=E> + std::ops::Add<Output = E> ,
{
    debug_assert!( arg.len() == 2);
    partial[ GenericAs::gas(arg[0]) ] = partial[ GenericAs::gas(arg[0]) ]
        + partial[res] * var_zero[ GenericAs::gas(arg[1]) ];
    //
    partial[ GenericAs::gas(arg[1]) ] = partial[ GenericAs::gas(arg[1]) ]
        + partial[res] * var_zero[ GenericAs::gas(arg[0]) ];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the mul operators.
///
/// # op_info_vec
/// is a map from [operator::id] to operator information.
pub(crate) fn set_op_info<F,U>( op_info_vec : &mut Vec< OpInfo<F,U> > )
where
    usize    : GenericAs<U> ,
    U        : Copy + GenericAs<usize> ,
    F        : Copy +
        std::ops::Add<F, Output=F> +
        std::ops::Mul<F, Output=F> +
        std::ops::Add< GAD<F,U>, Output= GAD<F,U> > +
        std::ops::Mul< GAD<F,U>, Output= GAD<F,U> > ,
    GAD<F,U> : Copy +
        std::ops::Add<F, Output=GAD<F,U> > +
        std::ops::Mul<F, Output=GAD<F,U> > +
        std::ops::Add< GAD<F,U>, Output=GAD<F,U> > +
        std::ops::Mul< GAD<F,U>, Output=GAD<F,U> > ,

{
    op_info_vec[MUL_CV_OP as usize] = OpInfo{
        name           : "mul_cv".to_string() ,
        forward_0      : forward_0_mul_cv,
        forward_1      : forward_1_mul_cv,
        reverse_1      : reverse_1_mul_cv,
        ad_forward_0   : forward_0_mul_cv,
        ad_forward_1   : forward_1_mul_cv,
        ad_reverse_1   : reverse_1_mul_cv,
        arg_var_index  : super::arg_var_index_binary_cv,
     };
    op_info_vec[MUL_VC_OP as usize] = OpInfo{
        name           : "mul_vc".to_string(),
        forward_0      : forward_0_mul_vc,
        forward_1      : forward_1_mul_vc,
        reverse_1      : reverse_1_mul_vc,
        ad_forward_0   : forward_0_mul_vc,
        ad_forward_1   : forward_1_mul_vc,
        ad_reverse_1   : reverse_1_mul_vc,
        arg_var_index  : super::arg_var_index_binary_vc,
    };
    op_info_vec[MUL_VV_OP as usize] = OpInfo{
        name           : "mul_vv".to_string(),
        forward_0      : forward_0_mul_vv,
        forward_1      : forward_1_mul_vv,
        reverse_1      : reverse_1_mul_vv,
        ad_forward_0   : forward_0_mul_vv,
        ad_forward_1   : forward_1_mul_vv,
        ad_reverse_1   : reverse_1_mul_vv,
        arg_var_index  : super::arg_var_index_binary_vv,
    };
}
//
// AD * AD, Float * AD, AD * Float
crate::ad::binary_ad_operator!( Mul, * );
//
// AD *= AD, AD *= Float
crate::ad::binary_ad_assign_op!( Mul, *= );
