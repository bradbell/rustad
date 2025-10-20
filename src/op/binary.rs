// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Utilities used by the binary operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use crate::IndexT;
//
// ---------------------------------------------------------------------------
//
// binary_cv_arg_var_index
pub(crate) fn binary_cv_arg_var_index(
    arg_var_index : &mut Vec<IndexT> ,
    _flag         : &Vec<bool>       ,
    arg           : &[IndexT]        ,
) {
    arg_var_index.resize(1, 0 as IndexT);
    arg_var_index[0] = arg[1];
}
//
// binary_vc_arg_var_index
pub(crate) fn binary_vc_arg_var_index(
    arg_var_index : &mut Vec<IndexT> ,
    _flag         : &Vec<bool>       ,
    arg           : &[IndexT]        ,
) {
    arg_var_index.resize(1, 0 as IndexT);
    arg_var_index[0] = arg[0];
}
//
// binary_vv_arg_var_index
pub(crate) fn binary_vv_arg_var_index(
    arg_var_index : &mut Vec<IndexT> ,
    _flag         : &Vec<bool>       ,
    arg           : &[IndexT]        ,
) {
    arg_var_index.resize(2, 0 as IndexT);
    arg_var_index[0] = arg[0];
    arg_var_index[1] = arg[1];
}
// ---------------------------------------------------------------------------
// eval_binary_forward_0
/// Evaluation of zero order forward for binary operators.
///
/// * V      : see [doc_generic_v](crate::doc_generic_v)
/// * E      : see [doc_generic_e](crate::doc_generic_e)
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
/// [IndexT] must be defined in any module that uses eval_binary_forward_0
macro_rules! eval_binary_forward_0 { ($Name:ident, $op:tt) => { paste::paste! {
    #[doc = concat!(
        " E zero order forward for constant ", stringify!( $op ),
        " variable; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _cv_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        con         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
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
        " E zero order forward variable ", stringify!( $op ),
        " constant; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vc_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        con         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
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
        " variable; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vv_forward_0 >] <V, E> (
        var_zero    : &mut Vec<E> ,
        _con        : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
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
// ---------------------------------------------------------------------------
// binary_rust_src
/// Rust source code for binary operators.
///
/// * Name   :  is Add , Sub , Mul , or Div  ,
/// * op     : is the corresponding operator; e.g. + for Add.
///
/// This defines the following functions in the current module:
/// ```text
///     {name}_cv_rust_src
///     {name}_vc_rust_src
///     {name}_vv_rust_src
/// ```
/// where {name} is a lower case version of Name and
/// v (c) means the corresponding operand is a variable (constant) .
///
/// [IndexT] must be defined in any module that uses binary_rust_src
macro_rules! binary_rust_src { ($Name:ident, $op:tt) => { paste::paste! {
    #[doc = concat!(
        " rust source code for constant ", stringify!( $op ),
        " variable; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _cv_rust_src >] (
        n_domain    : usize       ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        res         : usize       ) -> String
    {
        assert_eq!( arg.len(), 2);
        assert!( n_domain <= res );
        let lhs     = arg[0] as usize;
        let mut rhs = arg[1] as usize;
        let res     = res - n_domain;
        let op      = stringify!($op);
        if rhs < n_domain {
            format!("dep[{res}] = &con[{lhs}] {op} domain[{rhs}];")
        } else {
            rhs = rhs - n_domain;
            format!("dep[{res}] = &con[{lhs}] {op} &dep[{rhs}];")
        }
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " constant; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vc_rust_src >] (
        n_domain    : usize       ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        res         : usize       ) -> String
    {
        assert_eq!( arg.len(), 2);
        assert!( n_domain <= res );
        let mut lhs = arg[0] as usize;
        let rhs    = arg[1] as usize;
        let res    = res - n_domain;
        let op     = stringify!($op);
        if lhs < n_domain {
            format!("dep[{res}] = domain[{lhs}] {op} &con[{rhs}];")
        } else {
            lhs = lhs - n_domain;
            format!("dep[{res}] = &dep[{lhs}] {op} &con[{rhs}];")
        }
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " variable; see [ForwardZero](crate::op::info::ForwardZero)"
    ) ]
    fn [< $Name:lower _vv_rust_src >] (
        n_domain    : usize       ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        res         : usize       ) -> String
    {
        assert_eq!( arg.len(), 2);
        assert!( n_domain <= res );
        let mut lhs = arg[0] as usize;
        let mut rhs = arg[1] as usize;
        let res     = res - n_domain;
        let op     = stringify!($op);
        if lhs < n_domain {
            if rhs < n_domain {
                format!("dep[{res}] = domain[{lhs}] {op} domain[{rhs}];")
            } else {
                rhs = rhs - n_domain;
                format!("dep[{res}] = domain[{lhs}] {op} &dep[{rhs}];")
            }
        } else {
            lhs = lhs - n_domain;
            if rhs < n_domain {
                format!("dep[{res}] = &dep[{lhs}] {op} domain[{rhs}];")
            } else {
                rhs = rhs - n_domain;
                format!("dep[{res}] = &dep[{lhs}] {op} &dep[{rhs}];")
            }
        }
    }
} } }
pub(crate) use binary_rust_src;
