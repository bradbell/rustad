// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Utilities used by the binary operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use crate::numvec::tape::Tindex;
//
// ---------------------------------------------------------------------------
//
// binary_cv_arg_var_index
pub(crate) fn binary_cv_arg_var_index(
    arg_var_index : &mut Vec<Tindex> ,
    _flag         : &Vec<bool>       ,
    arg           : &[Tindex]        ,
) {
    arg_var_index.resize(1, 0 as Tindex);
    arg_var_index[0] = arg[1];
}
//
// binary_vc_arg_var_index
pub(crate) fn binary_vc_arg_var_index(
    arg_var_index : &mut Vec<Tindex> ,
    _flag         : &Vec<bool>       ,
    arg           : &[Tindex]        ,
) {
    arg_var_index.resize(1, 0 as Tindex);
    arg_var_index[0] = arg[0];
}
//
// binary_vv_arg_var_index
pub(crate) fn binary_vv_arg_var_index(
    arg_var_index : &mut Vec<Tindex> ,
    _flag         : &Vec<bool>       ,
    arg           : &[Tindex]        ,
) {
    arg_var_index.resize(2, 0 as Tindex);
    arg_var_index[0] = arg[0];
    arg_var_index[1] = arg[1];
}
// ---------------------------------------------------------------------------
// eval_binary_forward_0
/// Evaluation of zero order forward for binary operators.
///
/// * V      : see [doc_generic_v](crate::numvec::doc_generic_v)
/// * E      : see [doc_generic_e](crate::numvec::doc_generic_e)
/// * Name   :  is Add , Sub , Mul , or Div  ,
/// * op     : is the corresponding operator; e.g. + for Add.
///
/// This defines the following functions in the current module:
/// ```text
///     {name}_cv_forward_0<V, E>
///     {name}_vc_forward_0<V, E>
///     {name}_vv_forward_0<V, E>
/// ```
/// where {name} is a lower case version of Name and
/// v (c) means the corresponding operand is a variable (constant) .
///
/// [Tindex] must be defined in any module that uses eval_binary_forward_0
macro_rules! eval_binary_forward_0 { ($Name:ident, $op:tt) => { paste::paste! {
    #[doc = concat!(
        " zero order forward for constant ", stringify!( $op ),
        " variable; see [ForwardZero](crate::numvec::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _cv_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        con         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[Tindex]   ,
        res         : usize       )
    where
        for<'a> &'a V : std::ops::$Name<&'a E, Output = E> ,
    {
        assert_eq!( arg.len(), 2);
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        var_zero[ res ] = &con[lhs] $op &var_zero[rhs];
    }
    #[doc = concat!(
        " zero order forward variable ", stringify!( $op ),
        " constant; see [ForwardZero](crate::numvec::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vc_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        con         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[Tindex]   ,
        res         : usize       )
    where
        for<'a> &'a E : std::ops::$Name<&'a V, Output = E> ,
    {
        assert_eq!( arg.len(), 2);
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        var_zero[ res ] = &var_zero[lhs] $op &con[rhs];
    }
    #[doc = concat!(
        " E zero order forward variable ", stringify!( $op ),
        " variable; see [ForwardZero](crate::numvec::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vv_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        _con        : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[Tindex]   ,
        res         : usize       )
    where
        for<'a> &'a E : std::ops::$Name<&'a E, Output = E> ,
    {
        assert_eq!( arg.len(), 2);
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        var_zero[ res ] = &var_zero[lhs] $op &var_zero[rhs];
    }
} } }
pub(crate) use eval_binary_forward_0;
