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
use crate::numvec::AD;
use crate::numvec::op::id::NUMBER_OP;
//
#[cfg(doc)]
use crate::numvec::ad::doc_generic_v;
#[cfg(doc)]
use crate::numvec::adfn::doc_generic_e;
//
// ---------------------------------------------------------------------------
// doc_common_arguments
/// Common arguments for operator evaluation functions.
///
/// * var_zero :
/// vector of zero order results for all the variable by variable index.
///
/// * con_all :
/// vector of all the constant values used by operators.
///
/// * flag_all :
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
/// Evaluation of zero order forward mpode.
///
/// * V : see [doc_generic_v]
/// * E : see [doc_generic_e]
///
/// * var_zero :
/// is the vector of zero order variable values.
/// This is an input for variable indices less than *res* and an output
/// for the results of this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardZero<V, E> = fn(
    _var_zero : &mut Vec<E> ,
    _con_all  : &Vec<V>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[usize]    ,
    _res      : usize       ,
);
// panic_zero
/// default [ForwardZero] function will panic
fn panic_zero<V,E> (
    _var_zero : &mut Vec<E> ,
    _con_all  : &Vec<V>     ,
    _flag_all : &Vec<bool>  ,
    _arg      : &[usize]    ,
    _res      : usize       ,
) { panic!(); }
// ---------------------------------------------------------------------------
/// Information for one operator
#[derive(Clone)]
pub struct OpInfo<V> {
    //
    /// name the user sees for this operator
    pub name : &'static str,
    //
    /// zero order forward mode *V* evaluation for this operator
    pub forward_0 : ForwardZero<V, V>,
    //
    /// zero order forward mode ``AD`` < *V* > evaluation for this operator
    pub ad_forward_0 : ForwardZero<V, AD<V> >,
}
// ---------------------------------------------------------------------------
// op_info_vec
/// returns the vector of length [NUMBER_OP]
/// that maps each operator id to it's [OpInfo] .
///
pub fn op_info_vec<V>() -> Vec< OpInfo<V> >
where
    OpInfo<V> : Clone ,
{
    let empty = OpInfo {
        name         : &"",
        forward_0    : panic_zero,
        ad_forward_0 : panic_zero,
    };
    let result : Vec< OpInfo<V> > = vec![empty ; NUMBER_OP as usize];
    // TODO: add this calls
    // crate::numvec::op::add::set_op_info(&mut result);
    // crate::numvec::op::sub::set_op_info(&mut result);
    // crate::numvec::op::mul::set_op_info(&mut result);
    // crate::numvec::op::div::set_op_info(&mut result);
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
                    LazyLock::new( || crate::numvec::op::info::op_info_vec() );
            &OP_INFO_VEC
        }
    }
} }
pub(crate) use impl_global_op_info_vec;
