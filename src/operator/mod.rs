// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Operations for specific operators
//! : [parent module](super)
//
//
use crate::ptrait::GenericAs;
use crate::ad::GAD;
//
#[cfg(doc)]
use crate::record::{Tape};
//
#[cfg(doc)]
use crate::function::ADFun;
//
use crate::Float;
use crate::Index;
use id::NUMBER_OP;
//
// id
pub mod id;
// ---------------------------------------------------------------------------
//
// operators
pub mod add;
pub mod mul;
pub mod call;
//
// ---------------------------------------------------------------------------
/// Implement zero order forward for binary operators.
///
/// * F      : floating point type for values in this operation sequence.
/// * U      : unsigned integer type for indices in this operation sequence.
/// * E      : is the evaluation type, either F or GAD<F,U>.
/// * Trait  :  is Add , Sub , Mul , or Div  ,
/// * op     : is the operator; e.g. + for add.
///
/// This defines the following functions:
/// ```text
///     forward_0_{op_name}_cv<F, U, E>
///     forward_0_{op_name}_vc<F, U, E>
///     forward_0_{op_name}_vv<F, U, E>
/// ```
/// where {op_name} is a lower case version of Trait and
/// v (c) means the corresponding operand is a variable (constant) .
macro_rules! binary_op_forward_0 {
    ($Trait:ident, $op:tt) => { paste::paste! {

        #[doc = concat!(
            " E zero order forward constant ",
            stringify!( $op ), " variable where E = F or GAD<F,U>"
        ) ]
        fn [< forward_0_ $Trait:lower  _cv >]<F, U, E> (
            var_zero: &mut Vec<E>,
            con:           &Vec<F>,
            _flag_all:     &Vec<bool>,
            arg:           &[U],
            res:           usize)
        where
            F : Copy + std::ops::$Trait<E, Output = E> ,
            U : Copy + GenericAs<usize> ,
            E : Copy ,
        {
            assert_eq!( arg.len(), 2);
            let lhs : usize = GenericAs::gas( arg[0] );
            let rhs : usize = GenericAs::gas( arg[1] );
            var_zero[ res ] = con[lhs] $op var_zero[rhs];
        }
        #[doc = concat!(
            " E zero order forward variable ",
            stringify!( $op ), " constant whee E = F or GAD<F,U>"
        ) ]
        fn [< forward_0_ $Trait:lower  _vc >]<F, U, E> (
            var_zero: &mut Vec<E>,
            con:           &Vec<F>,
            _flag_all:     &Vec<bool>,
            arg:           &[U],
            res:           usize)
        where
            F : Copy ,
            U : Copy + GenericAs<usize> ,
            E : Copy + std::ops::$Trait<F, Output = E> ,
        {
            assert_eq!( arg.len(), 2);
            let lhs : usize = GenericAs::gas( arg[0] );
            let rhs : usize = GenericAs::gas( arg[1] );
            var_zero[ res ] = var_zero[lhs] $op con[rhs];
        }
        #[doc = concat!(
            " E zero order forward variable ",
            stringify!( $op ), " variable where E = F or GAD<F,U>"
        ) ]
        fn [< forward_0_ $Trait:lower  _vv >]<F, U, E> (
            var_zero: &mut Vec<E>,
            _con:          &Vec<F>,
            _flag_all:     &Vec<bool>,
            arg:           &[U],
            res:           usize)
        where
            U : Copy + GenericAs<usize> ,
            E : Copy + std::ops::$Trait<E, Output = E> ,
        {
            assert_eq!( arg.len(), 2);
            let lhs : usize = GenericAs::gas( arg[0] );
            let rhs : usize = GenericAs::gas( arg[1] );
            var_zero[ res ] = var_zero[lhs] $op var_zero[rhs];
        }
    } };
}
pub(crate) use binary_op_forward_0;
// ---------------------------------------------------------------------------
// Function types used to evaluate one operator in operation sequence.
//
/// Common arguments for operator evaluation functions.
///
/// * var_zero :
/// The vector of zero order variable values.
///
/// * con_all :
/// The vector of all the constant values used by operators.
///
/// * flag_all :
/// The vector of all the boolean values used by operators.
///
/// * arg :
/// The sub-vector of arguments for this operator.
///
/// * res :
/// The variable index corresponding to the first result for this operator.
#[cfg(doc)]
pub fn doc_common_arguments() {}
//
// ForwardZero
/// Evaluation of zero order forward mode.
///
/// * F      : floating point type for values in this operation sequence.
/// * U      : unsigned integer type for indices in this operation sequence.
/// * E      : is the evaluation type, either F or GAD<F,U>.
///
/// * var_zero :
/// is the vector of zero order variable values.
/// This is an input for variable indices less than *res* and an output
/// for the results of this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardZero<F, U, E> = fn(
    _var_zero : &mut Vec<E> ,
    _con_all  : &Vec<F>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[U]        ,
    _res      : usize       ,
);
//
// ForwardOne
/// Evaluation of first order forward mode; see [doc_common_arguments]
///
/// * F      : floating point type for values in this operation sequence.
/// * U      : unsigned integer type for indices in this operation sequence.
/// * E      : is the evaluation type, either F or GAD<F,U>.
///
/// * var_one :
/// is the directional derivative for each variable.
/// This is an input for variable indices less than *res* and an output
/// for the results of this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardOne<F, U, E> = fn(
    _var_one  : &mut Vec<E>         ,
    _var_zero : &Vec<E>             ,
    _con_all  : &Vec<F>             ,
    _arg      : &[U]                ,
    _res      : usize               ,
);
//
// reverse_one_fn
/// Float evaluation of first order reverse mode.
///
/// * F      : floating point type for values in this operation sequence.
/// * U      : unsigned integer type for indices in this operation sequence.
/// * E      : is the evaluation type, either F or GAD<F,U>.
///
/// * partial :
/// is the partial, for each variable, of the scalar function.
/// This is an input for variable indices greater than *res* and an output
/// for the arguments for this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ReverseOne<F, U, E> = fn(
    _partial  : &mut Vec<E> ,
    _var_zero : &Vec<E>     ,
    _con_all  : &Vec<F>     ,
    _arg      : &[U]        ,
    _res      : usize       ,
);
//
// ArgVarIndex
/// Determine variable indices that are arguments to this operator.
///
/// * U : unsigned integer type for indices in this operation sequence.
///
/// * arg_var_index :
/// Passing in arg_var_index avoids reallocating memory for each call.
///
/// * flag_all :
/// The vector of all the boolean values used by operators.
///
/// * arg :
/// The subvector of arguments for this operator.
///
pub type ArgVarIndex<U> = fn(
    _arg_var_index: &mut Vec<U>, _flag_all : &Vec<bool>, _arg: &[U]
);
//
// ForwardZeroBinary
/// This is a [ForwardZero] with E = F and the following extra conditions:
///
/// * op :
/// we use the notation *op* for this operator's symbol; e.g. + for addition.
///
/// * arg :
/// is a slice of size two.  We use the notation
/// ```text
///     lhs = arg[0]
///     rhs = arg[1]
/// ```
///
/// * res :
/// is the index in *var_zero* where the result for this operator is placed.
///
/// * var_zero :
/// is the vector of the zero order values for all the variables.
/// If both left and right are variables:
/// ```text
///     var_zero[res] = var_zero[lhs] op var_zero[rhs]
/// ```
/// If left is a variable and the right is a constant:
/// ```text
///     var_zero[res] = var_zero[lhs] op con[rhs]
/// ```
/// If left is a constant and the right is a variable:
/// ```text
///     var_zero[res] = con[lhs] op var_zero[rhs]
/// ```
#[cfg(doc)]
pub type ForwardZeroBinary = fn(_var_zero: &mut Vec<Float>,
    _con_all: &Vec<Float>, _arg: &[Index], _res: usize
);
//
// ForwardOneBinary
/// This is a [ForwardOne] with the following extra conditions:
///
/// * var_one :
/// is the vector of directional derivatives.
/// The directional deriative var_one\[res\] is computed using the
/// its value of var_one\[i\] for indices *i* less tham *res* .
///
/// * Other Arguments : see [ForwardZeroBinary]
///
#[cfg(doc)]
pub type ForwardOneBinary = fn(_var_one: &mut Vec<Float>,
    _var_zero : &Vec<Float>, _con_all: &Vec<Float>, _arg: &[Index], _res: usize
);
//
// ReverseOneBinary
/// This is a [ReverseOne] with the following extra conditions:
///
/// * partial :
/// Reverse mode computes the partial derivatives of a scalar function of the
/// range vector.
/// On input *partial* contains the derivative w.r.t. the variables
/// up to and including *res* .
/// Upon return, the variable with index *res* has been removed by
/// expressing it as a function of the variables with lower indices.
///
/// * Other Arguments : see [ForwardZeroBinary]
///
#[cfg(doc)]
pub type ReverseOneBinary = fn(_var_one: &mut Vec<Float>,
    _var_zero : &Vec<Float>, _con_all: &Vec<Float>, _arg: &[Index], _res: usize
);
// ---------------------------------------------------------------------------
// Default evaluations
//
// panic_zero
/// default [ForwardZero] function, will panic
fn panic_zero<F,U,E>( _var_zero: &mut Vec<E>,
    _con_all: &Vec<F>,_flag_all : &Vec<bool>, _arg: &[U], _res: usize)
{
    panic!();
}
//
// panic_one
/// default [ForwardOne] or [ReverseOne] function, will panic
/// if it does not get replaced.
fn panic_one<F,U,E>( _var_one: &mut Vec<E>, _var_zero : &Vec<E>,
    _con_all: &Vec<F>, _arg: &[U], _res: usize) {
    panic!();
}
//
// panic_arg_var_index
/// default [ArgVarIndex] function,  will panic
fn panic_arg_var_index<U>(
    _arg_var_index: &mut Vec<U>, _flag_all: &Vec<bool>, _arg: &[U]
) {
    panic!();
}
// ---------------------------------------------------------------------------
// The binary ArgVarIndex cases
//
//  arg_var_index_binary_cv
/// The [ArgVarIndex] function for constant op variable cases.
fn arg_var_index_binary_cv<U>(
    arg_var_index: &mut Vec<U>, _flag_all: &Vec<bool>, arg: &[U]
) where
    U     : Copy ,
    usize : GenericAs<U> ,
{
    arg_var_index.resize( 1, GenericAs::gas(0) );
    arg_var_index[0] = arg[1];
}
//
//  arg_var_index_binary_vc
/// The [ArgVarIndex] function for variable op constant cases.
fn arg_var_index_binary_vc<U>(
    arg_var_index: &mut Vec<U>, _flag_all: &Vec<bool>, arg: &[U]
) where
    U     : Copy ,
    usize : GenericAs<U> ,
{
    arg_var_index.resize( 1, GenericAs::gas(0) );
    arg_var_index[0] = arg[0];
}
//
//  arg_var_index_binary_vv
/// The [ArgVarIndex] function for variable op variable cases.
fn arg_var_index_binary_vv<U>(
    arg_var_index: &mut Vec<U>, _flag_all: &Vec<bool>, arg: &[U]
) where
    U     : Copy ,
    usize : GenericAs<U> ,
{
    arg_var_index.resize( 2, GenericAs::gas(0) );
    arg_var_index[0] = arg[0];
    arg_var_index[1] = arg[1];
}
// ---------------------------------------------------------------------------
// OpInfo
//
/// information connected to each operator id
#[derive(Clone)]
pub struct OpInfo<F,U> {
    //
    /// name the user sees for this operator
    pub name           : String,
    //
    /// evaluates this operator during [ADFun::forward_zero]
    pub forward_0      : ForwardZero<F, U, F>,
    //
    /// evaluates this operator during [ADFun::forward_one]
    pub forward_1      : ForwardOne<F, U, F>,
    //
    /// evaluates this operator during [ADFun::reverse_one]
    pub reverse_1      : ReverseOne<F, U, F>,
    //
    /// evaluates this operator during [ADFun::forward_zero]
    pub ad_forward_0   : ForwardZero<F, U, GAD<F,U> >,
    //
    /// evaluates this operator during [ADFun::ad_forward_one]
    pub ad_forward_1   : ForwardOne<F, U, GAD<F,U> >,
    //
    /// evaluates this operator during [ADFun::ad_reverse_one]
    pub ad_reverse_1   : ReverseOne<F, U, GAD<F,U> >,
    //
    /// operator arguments that are variable indices;
    /// see [ArgVarIndex]
    pub arg_var_index  : ArgVarIndex<U>,
}
//
// default_op_info_vec
/// a vector of [OpInfo] where the name is empty and all the functions panic.
fn default_op_info_vec() -> Vec< OpInfo<Float,Index> > {
    let empty         = OpInfo{
        name           : "".to_string(),
        forward_0      : panic_zero,
        forward_1      : panic_one,
        reverse_1      : panic_one,
        ad_forward_0   : panic_zero,
        ad_forward_1   : panic_one,
        ad_reverse_1   : panic_one,
        arg_var_index  : panic_arg_var_index,
    };
    let mut result    = vec![empty ; NUMBER_OP as usize];
    add::set_op_info(&mut result);
    call::set_op_info(&mut result);
    mul::set_op_info(&mut result);
    result
}
//
// OP_INFO_VEC
/// mapping from each operator [id] to it's [OpInfo],
/// initialized using [default_op_info_vec] .
pub static OP_INFO_VEC: std::sync::LazyLock< Vec< OpInfo<Float,Index> > > =
   std::sync::LazyLock::new( || default_op_info_vec() );

// Test that all operators have the proper name.
// This test is referenced as at the end of id.rs.
#[test]
fn test_op_info() {
    let op_info_vec = &*OP_INFO_VEC;
    assert_eq!( "add_cv",   op_info_vec[id::ADD_CV_OP as usize].name );
    assert_eq!( "add_vc",   op_info_vec[id::ADD_VC_OP as usize].name );
    assert_eq!( "add_vv",   op_info_vec[id::ADD_VV_OP as usize].name );
    //
    assert_eq!( "call",     op_info_vec[id::CALL_OP as usize].name );
    assert_eq!( "call_res", op_info_vec[id::CALL_RES_OP as usize].name );
    //
    assert_eq!( "mul_cv",   op_info_vec[id::MUL_CV_OP as usize].name );
    assert_eq!( "mul_vc",   op_info_vec[id::MUL_VC_OP as usize].name );
    assert_eq!( "mul_vv",   op_info_vec[id::MUL_VV_OP as usize].name );
}
