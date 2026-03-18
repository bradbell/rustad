// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Information about an operator given it's operator id.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
use std::cmp::PartialEq;
//
use crate::ad::ADType;
use crate::{
    AD,
    IndexT,
    FBinary,
    FConst,
    FUnary,
};
use crate::op::id::NUMBER_OP;
use crate::tape::sealed::ThisThreadTape;
use crate::atom::sealed::GlobalAtomCallbackVec;
use crate::adfn::optimize;
//
#[cfg(doc)]
use crate::{
    doc_generic_v,
    doc_generic_e,
};
// ---------------------------------------------------------------------------
/// Arguments to operator functions that are always constant; i.e., immutable.
/// TODO: remove allow dead code when this gets used.
#[allow(dead_code)]
pub(crate) struct ConstData<'a, V> {
    // cop
    /// vector of all the constant values in this [ADfn](crate::ADfn)
    pub(crate) cop  : &'a [V] ,
    //
    // flag_all
    /// vector of all the boolean values in this [AGraph](crate::tape::AGraph)
    pub(crate) bool_all : &'a [bool] ,
    //
    // arg
    /// The arguments for this use of this operator.
    pub(crate) arg : &'a [IndexT] ,
    //
    // arg_type
    /// The type for each of the arguments above.
    ///     *   If arg_type\[i\] is ConstantP, then arg\[i\]
    ///         is an index in the constant parameter vector cop.
    ///     *   If arg_type\[i\] is DynamicP, then arg\[i\]
    ///         is an index in vector of all the dynamic parameters.
    ///     *   If arg_type\[i\] is Variable, then arg\[i\]
    ///         is an index in the vector of all the variables.
    ///     *   If arg_type\[i\] is Empty, the arg[\i\] is not a
    ///         constant, dynamic parameter, or variable index.
    pub(crate) arg_type : &'a [ADType] ,
    //
    /// The dynamic parameter or variable index for the result of this operator.
    /// The choice is clear from the context; e.g., in [ForwardDyp] (ForwardVar]
    /// it is a dynamic parameter (variable) index.
    pub(crate) res : usize ,
}
// ---------------------------------------------------------------------------
// ForwardDyp
/// Evaluation of dependent dynamic parameters.
///
/// * dyp_all  :
///   This is an input for dynamic parameters less than *res* and an output
///   for the results of this operator.
///
/// * const_data :  see [ConstData]
///
pub(crate) type ForwardDyp<V, E> = fn(
    _dyp_all  : &mut [E]    ,
    _const_data : ConstData<V> ,
);
// panic_dyp
/// Default [ForwardDyp] function will panic.
/// This can be used for dynamic parameter calculations by operators
/// that only have a variable argument (because they should not be in the
/// dynamic parameter acyclic graph).
pub(crate) fn panic_dyp<V, E> (
    _dyp_all  : &mut [E]    ,
    _const_data : ConstData<V> ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ForwardVar
/// Evaluation of variables.
///
/// * dyp_all :
///   contains the value of all the dynamic parameters.
///
/// * var_all  :
///   This is an input for variable indices less than *res* and an output
///   for the results of this operator.
///
/// * const_data :  see [ConstData]
///
pub(crate) type ForwardVar<V, E> = fn(
    _dyp_all  : &[E]        ,
    _var_all  : &mut [E]    ,
    _const_data : ConstData<V> ,
);
// panic_var
/// Default [ForwardVar] function will panic.
/// This can be used for variable calculations by operators
/// that only have parameter arguments (because they should not be in the
/// variable acyclic graph).
pub(crate) fn panic_var<V, E> (
    _dyp_all  : &[E]        ,
    _var_all  : &mut [E]    ,
    _const_data : ConstData<V> ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ForwardDer
/// Evaluation of first order forward mode.
///
/// * dyp_all :
///   contains the value of all the dynamic parameters.
///
/// * var_all :
///   contains the value of all the variables.
///
/// * var_der :
///   The sub-vector of var_der corresponding to the domain variables
///   specifies the direction for the derivative.
///   For i_var greater than the domain variable indices,
///   var_der\[ i_var \] is the directional derivative of variable i_var.
///   This is an input for i_var < res and an output for the results
///   of this operator.
///
/// * const_data :  see [ConstData]
pub(crate) type ForwardDer<V, E> = fn(
    _dyp_all  : &[E]        ,
    _var_all  : &[E]        ,
    _var_der  : &mut [E]    ,
    _const_data : ConstData<V> ,
);
//
// ReverseDer
/// Evaluation of first order reverse mode.
///
/// * dyp_all :
///   contains the value of all the dynamic parameters.
///
/// * var_all :
///   contains the value of all the variables.
///
/// * var_der :
///   A scalar function is defined by the weight sum of the range components.
///   On input, var_der contains the partial derivatives of the
///   scalar as a function of variable i_var <= res + n_res - 1
///   (where n_res is the number of results for the current operator).
///   On output, var_der contains the partial derivatives of the
///   scalar as a function of i_var < res.
///
/// * const_data :  see [ConstData]
pub(crate) type ReverseDer<V, E> = fn(
    _dyp_all  : &[E]        ,
    _var_all  : &[E]        ,
    _var_der  : &mut [E]    ,
    _const_data : ConstData<V> ,
);
//
// panic_der
/// Default [ForwardDer] and [ReverseDer] function will panic.
/// This can be used for variable calculations by operators
/// that only have parameter arguments (because they should not be in the
/// variable acyclic graph).
pub(crate) fn panic_der<V, E>  (
    _dyp_all  : &[E]        ,
    _var_all  : &[E]        ,
    _var_der  : &mut [E]    ,
    _const_data : ConstData<V> ,
) { panic!(); }
// ---------------------------------------------------------------------------
// RustSrc
/// Generate source code corresponding to forward_dyp and forward_var
/// evaluation.
///
/// * res_type :
///   This is the type of the dependent object being computed and must be
///   ADType::DynamicP or ADType::Variable.
///
/// * dyp_n_dom :
///   is the number of domain dynamic parameters.
///
/// * var_n_dom :
///   is the number of domain variables.
///
/// * const_data :  see [ConstData]
///
/// * return
///   The return value is the rust source code from this operation.
///
pub(crate) type RustSrc<V> = fn(
    _res_type  : ADType      ,
    _dyp_n_dom : usize       ,
    _var_n_dom : usize       ,
    _const_data : ConstData<V> ,
) -> String;
//
// panic_rust_src
/// Default [RustSrc] function will panic.
pub(crate) fn panic_rust_src<V>(
    _res_type   : ADType      ,
    _dyp_n_dom  : usize       ,
    _var_n_dom  : usize       ,
    _const_data : ConstData<V> ,
) -> String
{ panic!() }
// ----------------------------------------------------------------------------
// ReverseDepend
/// Reverse dependency analysis; i.e., which arguments does a result depend on.
///
/// * depend :
///   On input, depend contains the the dependencies given the dependent values
///   with index greater than res.
///   In addition, depend\[res\] is true.
///   Upon return,
///   depend contains the the dependencies given the dependent values
///   with index greater or equal res.
///
/// * res_type :
///   is ADType::DynamicP or ADType::Variable
///   and is the type of the result for this operation.
///
/// * Other Arguments : see [ConstData]
pub(crate) type ReverseDepend = fn(
    _depend   : &mut optimize::Depend ,
    _bool_all : &[bool]               ,
    _arg      : &[IndexT]             ,
    _arg_type : &[ADType]             ,
    _res      : usize                 ,
    _res_type : ADType                ,
);
//
// panic_reverse_depend
pub(crate) fn panic_reverse_depend(
    _depend   : &mut optimize::Depend ,
    _bool_all : &[bool]               ,
    _arg      : &[IndexT]             ,
    _arg_type : &[ADType]             ,
    _res      : usize                 ,
    _res_type : ADType                ,
) { panic!(); }
// ---------------------------------------------------------------------------
/// Information for one operator
#[derive(Clone)]
pub struct OpFns<V> {
    //
    /// name the user sees for this operator
    pub(crate) name : &'static str,
    //
    /// dependent dynamic parameter V evaluation for this operator
    pub(crate) forward_dyp_value : ForwardDyp<V, V>,
    //
    /// dependent dynamic parameter `AD<V>` evaluation for this operator
    pub(crate) forward_dyp_ad    : ForwardDyp<V, AD<V> >,
    //
    /// zero order forward mode V evaluation for this operator
    pub(crate) forward_var_value : ForwardVar<V, V>,
    //
    /// zero order forward mode `AD<V>` evaluation for this operator
    pub(crate) forward_var_ad  : ForwardVar<V, AD<V> >,
    //
    /// first order forward mode V evaluation for this operator
    pub(crate) forward_der_value : ForwardDer<V, V>,
    //
    /// first order forward mode `AD<V>` evaluation for this operator
    pub(crate) forward_der_ad  : ForwardDer<V, AD<V> >,
    //
    /// first order reverse mode V evaluation for this operator
    pub(crate) reverse_der_value : ReverseDer<V, V>,
    //
    /// first order reverse mode `AD<V>` evaluation for this operator
    pub(crate) reverse_der_ad  : ReverseDer<V, AD<V> >,
    //
    /// generate rust source code for this operator
    pub(crate) rust_src        : RustSrc<V>,
    //
    /// reverse dependency analysis for this operator
    pub(crate) reverse_depend  : ReverseDepend,
}
// ---------------------------------------------------------------------------
// op_fns_vec
/// returns the vector of length [NUMBER_OP]
/// that maps each operator id to it's [OpFns] .
///
pub(crate) fn op_fns_vec<V>() -> Vec< OpFns<V> >
where
    // add_assign, sub_assign
    for<'a> V : std::ops::AddAssign<&'a V> ,
    for<'a> V : std::ops::SubAssign<&'a V> ,
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
    V     : Clone + From<f32> + FConst + PartialEq,
    for<'a> &'a V : FUnary<Output=V>,
    V     : ThisThreadTape + GlobalAtomCallbackVec,
    for<'a> &'a V : FBinary<&'a V, Output = V> ,
    AD<V> : From<V>,
{
    let empty = OpFns {
        name               : "panic",
        forward_dyp_value  : panic_dyp::<V, V>,
        forward_dyp_ad     : panic_dyp::<V, AD<V>>,
        forward_var_value  : panic_var::<V, V>,
        forward_var_ad     : panic_var::<V, AD<V>>,
        forward_der_value  : panic_der::<V, V>,
        forward_der_ad     : panic_der::<V, AD<V>>,
        reverse_der_value  : panic_der::<V, V>,
        reverse_der_ad     : panic_der::<V, AD<V>>,
        rust_src           : panic_rust_src,
        reverse_depend     : panic_reverse_depend,
    };
    let mut result : Vec< OpFns<V> > = vec![empty ; NUMBER_OP as usize];
    //
    // binary operators
    crate::op::binary::add::set_op_fns::<V>(&mut result);
    crate::op::binary::sub::set_op_fns::<V>(&mut result);
    crate::op::binary::mul::set_op_fns::<V>(&mut result);
    crate::op::binary::div::set_op_fns::<V>(&mut result);
    crate::op::binary::num_cmp::set_op_fns::<V>(&mut result);
    crate::op::binary::atan2::set_op_fns::<V>(&mut result);
    crate::op::binary::hypot::set_op_fns::<V>(&mut result);
    crate::op::binary::powf::set_op_fns::<V>(&mut result);
    //
    // unary operators
    crate::op::unary::square::set_op_fns::<V>(&mut result);
    crate::op::unary::ln_1p::set_op_fns::<V>(&mut result);
    crate::op::unary::exp_m1::set_op_fns::<V>(&mut result);
    crate::op::unary::ln::set_op_fns::<V>(&mut result);
    crate::op::unary::sqrt::set_op_fns::<V>(&mut result);
    crate::op::unary::tanh::set_op_fns::<V>(&mut result);
    crate::op::unary::tan::set_op_fns::<V>(&mut result);
    crate::op::unary::sinh::set_op_fns::<V>(&mut result);
    crate::op::unary::cosh::set_op_fns::<V>(&mut result);
    crate::op::unary::abs::set_op_fns::<V>(&mut result);
    crate::op::unary::signum::set_op_fns::<V>(&mut result);
    crate::op::unary::exp::set_op_fns::<V>(&mut result);
    crate::op::unary::minus::set_op_fns::<V>(&mut result);
    crate::op::unary::cos::set_op_fns::<V>(&mut result);
    crate::op::unary::sin::set_op_fns::<V>(&mut result);
    //
    // call, no_op, powi
    crate::op::call::set_op_fns::<V>(&mut result);
    crate::op::no_op::set_op_fns::<V>(&mut result);
    crate::op::powi::set_op_fns::<V>(&mut result);
    //
    result
}
// ---------------------------------------------------------------------------
//
// sealed::GlobalOpFnsVec
pub(crate) mod sealed {
    //! The sub-module sealed is used to seal traits in this package
    //
    use super::OpFns;
    //
    #[cfg(doc)]
    use crate::doc_generic_v;
    //
    pub trait GlobalOpFnsVec
    where
        Self : Sized + 'static,
    {
        /// Returns a reference to the map from operator id to [OpFns]
        ///
        /// ```text
        ///     op_fns_vec = &*GlobalOpFnsVec::get()
        /// ```
        ///
        /// * Self : must be a value type V in [doc_generic_v]
        ///
        /// * op_fns_vec :
        ///   is the global vector of operator functions.
        ///
        fn get() -> &'static Vec< OpFns<Self> >;
    }
}
// impl_global_op_fns_vec!
/// Implement GlobalOpFnsVec for the value type *V* ; see [doc_generic_v]
///
/// This macro can be invoked from anywhere given the following use statements:
/// ```text
///     use std::thread::LocalKey;
///     use std::cell::RefCell;
///     use crate::ad::AD;
/// ```
macro_rules! impl_global_op_fns_vec{ ($V:ty) => {
    impl crate::op::info::sealed::GlobalOpFnsVec for $V {
        #[doc = concat!(
            "Operator functions used to evaluate `",
            stringify!($V), "`, and `AD<", stringify!($V), ">` operations"
        ) ]
        fn get() -> &'static Vec< crate::op::info::OpFns<$V> > {
            pub static OP_FNS_VEC :
                LazyLock< Vec< crate::op::info::OpFns<$V> > > =
                    LazyLock::new(
                        || crate::op::info::op_fns_vec::<$V>()
                    );
            &*OP_FNS_VEC
        }
    }
} }
pub(crate) use impl_global_op_fns_vec;
