// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Information about an operator given it's operator id.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use crate::numvec::ad::AD;
use crate::numvec::op::id::NUMBER_OP;
use crate::numvec::IndexT;
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::atom::sealed::AtomEvalVec;
//
#[cfg(doc)]
use crate::numvec::{
    doc_generic_v,
    doc_generic_e,
};
// ---------------------------------------------------------------------------
// doc_common_arguments
/// Common arguments for operator evaluation functions.
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * var_zero :
/// vector of zero order results for all the variable by variable index.
///
/// * con :
/// vector of all the constant values used by operators.
///
/// * flag :
/// vector of all the boolean values used by operators.
///
/// * arg :
/// The arguments for this operator as a sub-vector of all the arguments.
///
/// * res :
/// The variable index corresponding to the first result for this operator.
#[cfg(doc)]
pub fn doc_common_arguments() {}
// ---------------------------------------------------------------------------
// ForwardZero
/// Evaluation of zero order forward mode.
///
/// * var_zero :
/// is the vector of zero order variable values.
/// This is an input for variable indices less than *res* and an output
/// for the results of this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardZero<V, E> = fn(
    _var_zero : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
);
// panic_zero
/// default [ForwardZero] function will panic
pub fn panic_zero<V, E> (
    _var_zero : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ForwardOne
/// Evaluation of first order forward mode.
///
/// * var_one :
/// is the vector of directional derivative for each variable.
/// This is an input for variable indices less than *res* and an output
/// for the result of this operator; i.e. index *res* .
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardOne<V, E> = fn(
    _var_zero : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
);
// ReverseOne
/// Evaluation of first order reverse mode.
///
/// * var_one :
/// Is the vector of partial derivatives with respect to the
/// variables with index less than or equal to *res* .
/// This operation expresses the result of this operator as
/// a function of its arguments and modifies the argument partial derivatives
/// accordingly.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ReverseOne<V, E> = fn(
    _var_zero : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
);
// panic_one
/// default [ForwardOne] and [ReverseOne] function will panic
pub fn panic_one<V, E> (
    _var_zero : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ArgVarIndex
/// Return indices for variables that are arguments for an operation
///
/// * arg_var_index :
/// This vector is both an input and the output for this function
/// (to avoid having to reallocate memory for each call).
/// Only the capacity of the vector matters on input.
/// Upon return, it contains the indices for the variables that are
/// arguments for this operator.
///
/// * flag :
/// vector of all the boolean values used by operators.
///
/// * arg :
/// The arguments for this operator as a sub-vector of all the arguments.
///
pub type ArgVarIndex = fn(
    _arg_var_index : &mut Vec<IndexT> ,
    _flag          : &Vec<bool>       ,
    _arg           : &[IndexT]        ,
);
// panic_arg_var_index
/// default [ArgVarIndex] function will panic.
fn panic_arg_var_index(
    _arg_var_index : &mut Vec<IndexT> ,
    _flag          : &Vec<bool>       ,
    _arg           : &[IndexT]        ,
) { panic!() }
// ---------------------------------------------------------------------------
/// Information for one operator
#[derive(Clone)]
pub struct OpInfo<V> {
    //
    /// name the user sees for this operator
    pub name : &'static str,
        //
    /// zero order forward mode V evaluation for this operator
    pub forward_0_value : ForwardZero<V, V>,
    //
    /// zero order forward mode `AD<V>` evaluation for this operator
    pub forward_0_ad    : ForwardZero<V, AD<V> >,
    //
    /// first order forward mode V evaluation for this operator
    pub forward_1_value : ForwardOne<V, V>,
    //
    /// first order forward mode `AD<V>` evaluation for this operator
    pub forward_1_ad    : ForwardOne<V, AD<V> >,
    //
    /// first order reverse mode V evaluation for this operator
    pub reverse_1_value : ReverseOne<V, V>,
    //
    /// first order reverse mode `AD<V>` evaluation for this operator
    pub reverse_1_ad    : ReverseOne<V, AD<V> >,
    //
    /// get indices for variables that are arguments to this function
    pub arg_var_index   : ArgVarIndex,
}
// ---------------------------------------------------------------------------
// op_info_vec
/// returns the vector of length crate::numvec::op::id::NUMBER_OP
/// that maps each operator id to it's [OpInfo] .
///
pub fn op_info_vec<V>() -> Vec< OpInfo<V> >
where
    // add_assign
    for<'a> V : std::ops::AddAssign<&'a V> ,
    // add
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    // sub
    for<'a> &'a V : std::ops::Sub<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Sub<&'a V, Output = V> ,
    // mul
    for<'a> &'a V : std::ops::Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Mul<&'a V, Output = V> ,
    // div
    for<'a> &'a V : std::ops::Div<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Div<&'a V, Output = V> ,
    //
    V  : Clone + ThisThreadTape + AtomEvalVec + From<f32>,
{
    let empty = OpInfo {
        name             : &"panic",
        forward_0_value  : panic_zero::<V, V>,
        forward_0_ad     : panic_zero::<V, AD<V>>,
        forward_1_value  : panic_one::<V, V>,
        forward_1_ad     : panic_one::<V, AD<V>>,
        reverse_1_value  : panic_one::<V, V>,
        reverse_1_ad     : panic_one::<V, AD<V>>,
        arg_var_index    : panic_arg_var_index,
    };
    let mut result : Vec< OpInfo<V> > = vec![empty ; NUMBER_OP as usize];
    crate::numvec::op::add::set_op_info::<V>(&mut result);
    crate::numvec::op::sub::set_op_info::<V>(&mut result);
    crate::numvec::op::mul::set_op_info::<V>(&mut result);
    crate::numvec::op::div::set_op_info::<V>(&mut result);
    crate::numvec::op::call::set_op_info::<V>(&mut result);
    result
}
// ---------------------------------------------------------------------------
//
// GlobalOpInfoVec
/// returns a reference to the map from operator id to [OpInfo]
///
/// ```text
///     GlobalOpInfoVec::get()
/// ```
///
/// * V : see [doc_generic_v]
///
pub trait GlobalOpInfoVec
where
    Self : Sized + 'static,
{
    fn get() -> &'static std::sync::LazyLock< Vec< OpInfo<Self> > >;
}
// impl_global_op_info_vec!
/// Implement GlobalOpInfoVec for the value type *V* ; see [doc_generic_v]
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
///     use crate::numvec::ad::AD;
/// ```
macro_rules! impl_global_op_info_vec{ ($V:ty) => {
    #[doc = concat!(
        "Operator information used when evaluating `",
        stringify!($V), "`, and `AD<", stringify!($V), ">` operations"
    ) ]
    impl crate::numvec::op::info::GlobalOpInfoVec for $V {
        fn get() -> &'static LazyLock<
            Vec< crate::numvec::op::info::OpInfo<$V> >
        > {
            pub static OP_INFO_VEC :
                LazyLock< Vec< crate::numvec::op::info::OpInfo<$V> > > =
                    LazyLock::new(
                        || crate::numvec::op::info::op_info_vec::<$V>()
                    );
            &OP_INFO_VEC
        }
    }
} }
pub(crate) use impl_global_op_info_vec;
