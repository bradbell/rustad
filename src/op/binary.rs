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
// binary_pp_arg_var_index
pub(crate) fn binary_pp_arg_var_index(
    arg_var_index : &mut Vec<IndexT> ,
    _flag         : &Vec<bool>       ,
    _arg          : &[IndexT]        ,
) {
    arg_var_index.resize(0, 0 as IndexT);
}
//
// binary_pv_arg_var_index
pub(crate) fn binary_pv_arg_var_index(
    arg_var_index : &mut Vec<IndexT> ,
    _flag         : &Vec<bool>       ,
    arg           : &[IndexT]        ,
) {
    arg_var_index.resize(1, 0 as IndexT);
    arg_var_index[0] = arg[1];
}
//
// binary_vp_arg_var_index
pub(crate) fn binary_vp_arg_var_index(
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
///     {name}_pv_forward_0<V, E>
///     {name}_vp_forward_0<V, E>
///     {name}_vv_forward_0<V, E>
/// ```
/// where {name} is a lower case version of Name and
/// v (p) means the corresponding operand is a variable (parameter) .
///
/// [IndexT] must be defined in any module that uses eval_binary_forward_0
macro_rules! eval_binary_forward_0 { ($Name:ident, $op:tt) => { paste::paste! {
    #[doc = concat!(
        " E zero order forward for parameter ", stringify!( $op ),
        " parameter; see [ForwardDyp](crate::op::info::ForwardDyp)"
    ) ]
    fn [< $Name:lower _forward_dyp >] <V, E> (
        dyp_both    : &mut Vec<E> ,
        cop         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : std::ops::$Name<&'a E, Output = E> ,
        for<'a> &'a E : std::ops::$Name<&'a V, Output = E> ,
        for<'a> &'a E : std::ops::$Name<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!(
            ! ( arg_type[0].is_constant() && arg_type[1].is_constant() )
        );
        let lhs       = arg[0] as usize;
        let rhs       = arg[1] as usize;
        if arg_type[0].is_constant() {
            dyp_both[ res ] = &cop[lhs] $op &dyp_both[rhs];
        } else if arg_type[1].is_constant() {
            dyp_both[ res ] = &dyp_both[lhs] $op &cop[rhs];
        } else {
            dyp_both[ res ] = &dyp_both[lhs] $op &dyp_both[rhs];
        };
    }
    #[doc = concat!(
        " E zero order forward for parameter ", stringify!( $op ),
        " variable; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _pv_forward_0 >] <V, E> (
        dyp_both    : &Vec<E>     ,
        var_both    : &mut Vec<E> ,
        cop         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : std::ops::$Name<&'a E, Output = E> ,
        for<'a> &'a E : std::ops::$Name<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!( ! arg_type[1].is_constant() );
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        if arg_type[0].is_constant() {
            var_both[ res ] = &cop[lhs] $op &var_both[rhs];
        } else {
            var_both[ res ] = &dyp_both[lhs] $op &var_both[rhs];
        }
    }
    #[doc = concat!(
        " E zero order forward variable ", stringify!( $op ),
        " parameter; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _vp_forward_0 >] <V, E> (
        dyp_both    : &Vec<E>     ,
        var_both    : &mut Vec<E> ,
        cop         : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a E : std::ops::$Name<&'a V, Output = E> ,
        for<'a> &'a E : std::ops::$Name<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!( ! arg_type[0].is_constant() );
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        if arg_type[1].is_constant() {
            var_both[ res ] = &var_both[lhs] $op &cop[rhs];
        } else {
            var_both[ res ] = &var_both[lhs] $op &dyp_both[rhs];
        }
    }
    #[doc = concat!(
        " E zero order forward variable ", stringify!( $op ),
        " variable; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _vv_forward_0 >] <V, E> (
        _dyp_both   : &Vec<E>     ,
        var_both    : &mut Vec<E> ,
        _cop        : &Vec<V>     ,
        _flag       : &Vec<bool>  ,
        arg         : &[IndexT]   ,
        _arg_type   : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a E : std::ops::$Name<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        var_both[ res ] = &var_both[lhs] $op &var_both[rhs];
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
///     {name}_pv_rust_src
///     {name}_vp_rust_src
///     {name}_vv_rust_src
/// ```
/// where {name} is a lower case version of Name and
/// v (p) means the corresponding operand is a variable (parameter) .
///
/// [IndexT] must be defined in any module that uses binary_rust_src
macro_rules! binary_rust_src { ($Name:ident, $op:tt) => { paste::paste! {
    #[doc = concat!(
        " rust source code for parameter ", stringify!( $op ),
        " variable; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _pv_rust_src >]<V> (
        _not_used   : V           ,
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
        let src     = if rhs < n_domain {
            format!("dep[{res}] = &cop[{lhs}] {op} domain[{rhs}];")
        } else {
            rhs = rhs - n_domain;
            format!("dep[{res}] = &cop[{lhs}] {op} &dep[{rhs}];")
        };
        let src = String::from("   ") + &src + "\n";
        src
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " parameter; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _vp_rust_src >]<V> (
        _not_used   : V           ,
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
        let src    = if lhs < n_domain {
            format!("dep[{res}] = domain[{lhs}] {op} &cop[{rhs}];")
        } else {
            lhs = lhs - n_domain;
            format!("dep[{res}] = &dep[{lhs}] {op} &cop[{rhs}];")
        };
        let src = String::from("   ") + &src + "\n";
        src
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " variable; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $Name:lower _vv_rust_src >]<V> (
        _not_used   : V           ,
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
        let src    = if lhs < n_domain {
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
        };
        let src = String::from("   ") + &src + "\n";
        src
    }
} } }
pub(crate) use binary_rust_src;
