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
use std::cmp::PartialEq;
//
use crate::{
    AD,
    ADType,
    IndexT,
};
use crate::op::id::NUMBER_OP;
use crate::tape::sealed::ThisThreadTape;
use crate::atom::sealed::AtomEvalVec;
//
#[cfg(doc)]
use crate::{
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
/// * dyp_both :
/// vector of all the dynamic parameters in the following order:
/// the domain dynamic parameters followed by the dependent dynamic parameters.
///
/// * var_both :
/// vector of all the variables in the following order:
/// the domain variables followed by the dependent variables.
///
/// * cop :
/// vector of all the constant values used by operators.
///
/// * flag :
/// vector of all the boolean values used by operators.
///
/// * arg :
/// The arguments for this operator as a sub-vector of all the arguments.
///
/// * arg_type :
///     *   If arg_type\[i\] is ConstantP, then arg\[i\]
///         is an index in the  constant parameter vector.
///     *   If arg_type\[i\] is DynamicP, then arg\[i\]
///         is an index in dyp_both.
///     *   If arg_type\[i\] is Variable, then arg\[i\]
///         is an index in var_both.
///
/// * res :
/// If this is a dynamic parameter operator (variable operator),
/// res is the dyp_both (var_both) index for the value being computed.
///
#[cfg(doc)]
pub fn doc_common_arguments() {}
// ---------------------------------------------------------------------------
// ForwardDyp
/// Evaluation of dependent dynamic parameters.
///
/// * Arguments :  see [doc_common_arguments] .
/// In addition, there is the following extra condition:
///
/// * dyp_both :
/// This is an input for dynamic parameters less than *res* and an output
/// for the results of this operator.
///
pub type ForwardDyp<V, E> = fn(
    _dyp_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
);
// panic_dyp
/// Default [ForwardDyp] function will panic.
/// This can be used for dynamic parameter calculations by operators
/// that only have a variable argument (because they should not be in the
/// dynamic parameter operation sequence).
pub fn panic_dyp<V, E> (
    _dyp_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ForwardVar
/// Evaluation of variables.
///
/// * Arguments :  see [doc_common_arguments] .
/// In addition, there is the following extra condition:
///
/// * var_both :
/// This is an input for variable indices less than *res* and an output
/// for the results of this operator.
///
pub type ForwardVar<V, E> = fn(
    _dyp_both : &Vec<E>     ,
    _var_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
);
// panic_var
/// Default [ForwardVar] function will panic.
/// This can be used for variable calculations by operators
/// that only have parameter arguments (because they should not be in the
/// variable operation sequence).
pub fn panic_var<V, E> (
    _dyp_both : &Vec<E>     ,
    _var_both : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { panic!(); }
// ---------------------------------------------------------------------------
// ForwardDer
/// Evaluation of first order forward mode.
///
/// * var_der :
///     is the vector of directional derivatives.
///
///     * n_var : is the number of variables; i.e, var_both.len() .
///     * i_var : is a variable index; 0 <= i_var < n_var .
///     * n_dir : is the number of directions; i.e, var_der.len() / n_var .
///     * j_dir : is a direction index; 0 <= j_dir < n_dir .
///
///     The element var_dir\[ i_var * n_dir + j_dir]
///     is the component of the directional derivative corresponding to
///     variable i_var and direction j_dir.
///     This is an input for i_var <= res and an output for the results
///     of this operator.
///
/// * Other Arguments :  see [doc_common_arguments]
pub type ForwardDer<V, E> = fn(
    _dyp_both : &Vec<E>     ,
    _var_both : &Vec<E>     ,
    _var_der  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
);
// panic_der
/// Default [ForwardDer] function will panic.
/// This can be used for variable calculations by operators
/// that only have parameter arguments (because they should not be in the
/// variable operation sequence).
pub fn panic_der<V, E>  (
    _dyp_both : &Vec<E>     ,
    _var_both : &Vec<E>     ,
    _var_der  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { panic!(); }
//
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
    _var_both : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
);
// panic_one
/// Default [ForwardDer] and [ReverseOne] function will panic.
/// This can be used for variable calculations by operators
/// that only have parameter arguments (because they should not be in the
/// variable operation sequence).
pub fn panic_one<V, E> (
    _var_both : &Vec<E>     ,
    _var_one  : &mut Vec<E> ,
    _cop      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    _arg      : &[IndexT]   ,
    _res      : usize       ,
) { panic!(); }
// --------------------------------------------------------------------------
// no_forward_der_value
/// defines forward_der_value_none `<V>`
///
/// The types IndexT and ADType must be in scope where this macro is used.
macro_rules! no_forward_der_value{ ($Op:ident) => {
    pub fn forward_der_value_none<V> (
        _dyp_both : &Vec<V>     ,
        _var_both : &Vec<V>     ,
        _var_one  : &mut Vec<V> ,
        _cop      : &Vec<V>     ,
        _flag     : &Vec<bool>  ,
        _arg      : &[IndexT]   ,
        _arg_type : &[ADType]   ,
        _res      : usize       ,
    ) { panic!( concat!(
        stringify!($Op) ,
        ": forward_der_value not implemented for this operator" ,
    ))}
}}
pub(crate) use no_forward_der_value;
//
// no_forward_der_ad
/// defines forward_der_ad_none `<V>`
///
/// The types IndexT and ADType must be in scope where this macro is used.
macro_rules! no_forward_der_ad{ ($Op:ident) => {
    pub fn forward_der_ad_none<V> (
        _dyp_both : &Vec< AD<V> >     ,
        _var_both : &Vec< AD<V> >     ,
        _var_der  : &mut Vec< AD<V> > ,
        _cop      : &Vec<V>           ,
        _flag     : &Vec<bool>        ,
        _arg      : &[IndexT]         ,
        _arg_type : &[ADType]         ,
        _res      : usize             ,
    ) { panic!( concat!(
        stringify!($Op) ,
        ": forward_der_ad not implemented for this operator" ,
    ))}
}}
pub(crate) use no_forward_der_ad;
//
// no_reverse_one_value
/// defines reverse_one_value_none `<V>`
///
/// The type IndexT must be in scope where this macro is used.
macro_rules! no_reverse_one_value{ ($Op:ident) => {
    pub fn reverse_one_value_none<V> (
        _var_both : &Vec<V>     ,
        _var_one  : &mut Vec<V> ,
        _cop      : &Vec<V>     ,
        _flag     : &Vec<bool>  ,
        _arg      : &[IndexT]   ,
        _res      : usize       ,
    ) { panic!( concat!(
        stringify!($Op) ,
        ": forward_one_value not implemented for this operator" ,
    ))}
}}
pub(crate) use no_reverse_one_value;
//
// no_reverse_one_ad
/// defines reverse_one_ad_none `<V>`
///
/// The type IndexT must be in scope where this macro is used.
macro_rules! no_reverse_one_ad{ ($Op:ident) => {
    pub fn reverse_one_ad_none<V> (
        _var_both : &Vec< AD<V> >     ,
        _var_one  : &mut Vec< AD<V> > ,
        _cop      : &Vec<V>           ,
        _flag     : &Vec<bool>        ,
        _arg      : &[IndexT]         ,
        _res      : usize             ,
    ) { panic!( concat!(
        stringify!($Op) ,
        ": forward_one_ad not implemented for this operator" ,
    ))}
}}
pub(crate) use no_reverse_one_ad;
//
// no_rust_src
/// defines rust_src_none `<V>`
///
/// The type IndexT must be in scope where this macro is used.
macro_rules! no_rust_src{ ($Op:ident) => {
    pub fn rust_src_none<V>(
        _not_used : V           ,
        _res_type  : ADType      ,
        _dyp_n_dom : usize       ,
        _var_n_dom : usize       ,
        _flag      : &Vec<bool>  ,
        _arg       : &[IndexT]   ,
        _arg_type  : &[ADType]   ,
        _res       : usize       ,
    ) -> String
    { panic!( concat!(
        stringify!($Op) ,
        ": rust_src not implemented for this operator" ,
    ))}
}}
pub(crate) use no_rust_src;
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
/// Default [ArgVarIndex] function will panic.
fn panic_arg_var_index(
    _arg_var_index : &mut Vec<IndexT> ,
    _flag          : &Vec<bool>       ,
    _arg           : &[IndexT]        ,
) { panic!() }
// ---------------------------------------------------------------------------
// RustSrc
/// Generate source code corresponding to forward_dyp and forward_var
/// evaluation.
///
/// * not_used :
/// This argument is only used to determine the value type V.
///
/// * res_type :
/// This is the type of the dependent object being computed and must be
/// ADType::DynamicP or ADType::Variable.
///
/// * dyp_n_dom :
/// is the number of domain dynamic parameters.
///
/// * var_n_dom :
/// is the number of domain variables.
///
/// * Other Arguments :  see [doc_common_arguments]
///
/// * return
/// The return value is the rust source code from this operation.
///
pub type RustSrc<V> = fn(
    _not_used : V           ,
    _res_type  : ADType      ,
    _dyp_n_dom : usize       ,
    _var_n_dom : usize       ,
    _flag      : &Vec<bool>  ,
    _arg       : &[IndexT]   ,
    _arg_type  : &[ADType]   ,
    _res       : usize       ,
) -> String;
//
// panic_rust_src
/// Default [RustSrc] function will panic.
pub fn panic_rust_src<V>(
    _not_used   : V           ,
    _res_type   : ADType      ,
    _dyp_n_dom  : usize       ,
    _var_n_dom  : usize       ,
    _flag       : &Vec<bool>  ,
    _arg        : &[IndexT]   ,
    _arg_type   : &[ADType]   ,
    _op_index   : usize       ,
) -> String
{ panic!() }
// ---------------------------------------------------------------------------
/// Information for one operator
#[derive(Clone)]
pub struct OpInfo<V> {
    //
    /// name the user sees for this operator
    pub name : &'static str,
    //
    /// dependent dynamic parameter V evaluation for this operator
    pub forward_dyp_value : ForwardDyp<V, V>,
    //
    /// dependent dynamic parameter `AD<V>` evaluation for this operator
    pub forward_dyp_ad    : ForwardDyp<V, AD<V> >,
    //
    /// zero order forward mode V evaluation for this operator
    pub forward_var_value : ForwardVar<V, V>,
    //
    /// zero order forward mode `AD<V>` evaluation for this operator
    pub forward_var_ad  : ForwardVar<V, AD<V> >,
    //
    /// first order forward mode V evaluation for this operator
    pub forward_der_value : ForwardDer<V, V>,
    //
    /// first order forward mode `AD<V>` evaluation for this operator
    pub forward_der_ad  : ForwardDer<V, AD<V> >,
    //
    /// first order reverse mode V evaluation for this operator
    pub reverse_1_value : ReverseOne<V, V>,
    //
    /// first order reverse mode `AD<V>` evaluation for this operator
    pub reverse_1_ad    : ReverseOne<V, AD<V> >,
    //
    /// generate rust source code for this operator
    pub rust_src        : RustSrc<V>,
    //
    /// get indices for variables that are arguments to this function
    pub arg_var_index   : ArgVarIndex,
}
// ---------------------------------------------------------------------------
// op_info_vec
/// returns the vector of length [NUMBER_OP]
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
    V : Clone + From<f32> + PartialEq + ThisThreadTape + AtomEvalVec
{
    let empty = OpInfo {
        name               : &"panic",
        forward_dyp_value  : panic_dyp::<V, V>,
        forward_dyp_ad     : panic_dyp::<V, AD<V>>,
        forward_var_value  : panic_var::<V, V>,
        forward_var_ad     : panic_var::<V, AD<V>>,
        forward_der_value  : panic_der::<V, V>,
        forward_der_ad     : panic_der::<V, AD<V>>,
        reverse_1_value    : panic_one::<V, V>,
        reverse_1_ad       : panic_one::<V, AD<V>>,
        rust_src           : panic_rust_src,
        arg_var_index      : panic_arg_var_index,
    };
    let mut result : Vec< OpInfo<V> > = vec![empty ; NUMBER_OP as usize];
    crate::op::add::set_op_info::<V>(&mut result);
    crate::op::sub::set_op_info::<V>(&mut result);
    crate::op::mul::set_op_info::<V>(&mut result);
    crate::op::div::set_op_info::<V>(&mut result);
    crate::op::call::set_op_info::<V>(&mut result);
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
///     use crate::ad::AD;
/// ```
macro_rules! impl_global_op_info_vec{ ($V:ty) => {
    #[doc = concat!(
        "Operator information used when evaluating `",
        stringify!($V), "`, and `AD<", stringify!($V), ">` operations"
    ) ]
    impl crate::op::info::GlobalOpInfoVec for $V {
        fn get() -> &'static LazyLock<
            Vec< crate::op::info::OpInfo<$V> >
        > {
            pub static OP_INFO_VEC :
                LazyLock< Vec< crate::op::info::OpInfo<$V> > > =
                    LazyLock::new(
                        || crate::op::info::op_info_vec::<$V>()
                    );
            &OP_INFO_VEC
        }
    }
} }
pub(crate) use impl_global_op_info_vec;
