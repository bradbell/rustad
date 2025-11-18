// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Utilities used by the binary operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
#[cfg(doc)]
use crate::IndexT;
//
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
        _flag       : &Vec<ADType>,
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
        _flag       : &Vec<ADType>,
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
        _flag       : &Vec<ADType>,
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
        _flag       : &Vec<ADType>,
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
///     {name}_pp_rust_src
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
        " variable; see [RustSrc](crate::op::info::RustSrc)"
    ) ]
    fn [< $Name:lower _pv_rust_src >]<V> (
        _not_used   : V           ,
        res_type    : ADType      ,
        dyp_n_dom   : usize       ,
        var_n_dom   : usize       ,
        _flag       : &Vec<ADType>,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res       : usize       ) -> String
    {   //
        debug_assert!( arg.len() == 2);
        debug_assert!( res_type.is_variable() );
        debug_assert!( arg_type[0].is_parameter() );
        debug_assert!( arg_type[1].is_variable() );
        debug_assert!( var_n_dom <= res );
        //
        // lhs_str
        let mut lhs = arg[0] as usize;
        let lhs_str : String;
        if arg_type[0].is_constant() {
            lhs_str = format!("&cop[{lhs}]");
        } else if lhs < dyp_n_dom {
            lhs_str = format!("dyp_dom[{lhs}]");
        } else {
            lhs = lhs - dyp_n_dom;
            lhs_str = format!("&dyp_dep[{lhs}]");
        }
        //
        // rhs_str
        let mut rhs = arg[1] as usize;
        let rhs_str : String;
        if rhs < var_n_dom  {
            rhs_str = format!("var_dom[{rhs}]");
        } else {
            rhs = rhs - var_n_dom;
            rhs_str = format!("&var_dep[{rhs}]");
        }
        //
        // res_str
        let res              = res - var_n_dom;
        let res_str : String = format!("var_dep[{res}]");
        //
        // op_str
        let op_str  = stringify!($op);
        //
        // src
        let src = String::from("   ");
        let src = src + &res_str +
            " = " + &lhs_str + " " + op_str + " " + &rhs_str + ";\n";
        src
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " parameter; see [RustSrc](crate::op::info::RustSrc)"
    ) ]
    fn [< $Name:lower _vp_rust_src >]<V> (
        _not_used   : V           ,
        res_type    : ADType      ,
        dyp_n_dom   : usize       ,
        var_n_dom   : usize       ,
        _flag       : &Vec<ADType>,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res       : usize       ) -> String
    {   //
        debug_assert!( arg.len() == 2);
        debug_assert!( res_type.is_variable() );
        debug_assert!( arg_type[0].is_variable() );
        debug_assert!( arg_type[1].is_parameter() );
        debug_assert!( var_n_dom <= res );
        //
        // lhs_str
        let mut lhs = arg[0] as usize;
        let lhs_str : String;
        if lhs < var_n_dom  {
            lhs_str = format!("var_dom[{lhs}]");
        } else {
            lhs = lhs - var_n_dom;
            lhs_str = format!("&var_dep[{lhs}]");
        }
        //
        // rhs_str
        let mut rhs = arg[0] as usize;
        let rhs_str : String;
        if arg_type[0].is_constant() {
            rhs_str = format!("&cop[{rhs}]");
        } else if rhs < dyp_n_dom {
            rhs_str = format!("dyp_dom[{rhs}]");
        } else {
            rhs = rhs - dyp_n_dom;
            rhs_str = format!("&dyp_dep[{rhs}]");
        }
        //
        // res_str
        let res              = res - var_n_dom;
        let res_str : String = format!("var_dep[{res}]");
        //
        // op_str
        let op_str  = stringify!($op);
        //
        // src
        let src = String::from("   ");
        let src = src + &res_str +
            " = " + &lhs_str + " " + op_str + " " + &rhs_str + ";\n";
        src
    }
    #[doc = concat!(
        " rust source code for variable ", stringify!( $op ),
        " variable; see [RustSrc](crate::op::info::RustSrc)"
    ) ]
    fn [< $Name:lower _vv_rust_src >]<V> (
        _not_used   : V           ,
        res_type    : ADType      ,
        _dyp_n_dom  : usize       ,
        var_n_dom   : usize       ,
        _flag       : &Vec<ADType>,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res       : usize       ) -> String
    {   //
        debug_assert!( arg.len() == 2);
        debug_assert!( res_type.is_variable() );
        debug_assert!( arg_type[0].is_variable() );
        debug_assert!( arg_type[1].is_variable() );
        debug_assert!( var_n_dom <= res );
        //
        // lhs_str
        let mut lhs = arg[0] as usize;
        let lhs_str : String;
        if lhs < var_n_dom  {
            lhs_str = format!("var_dom[{lhs}]");
        } else {
            lhs = lhs - var_n_dom;
            lhs_str = format!("&var_dep[{lhs}]");
        }
        //
        // rhs_str
        let mut rhs = arg[1] as usize;
        let rhs_str : String;
        if rhs < var_n_dom  {
            rhs_str = format!("var_dom[{rhs}]");
        } else {
            rhs = rhs - var_n_dom;
            rhs_str = format!("&var_dep[{rhs}]");
        }
        //
        // res_str
        let res              = res - var_n_dom;
        let res_str : String = format!("var_dep[{res}]");
        //
        // op_str
        let op_str  = stringify!($op);
        //
        // src
        let src = String::from("   ");
        let src = src + &res_str +
            " = " + &lhs_str + " " + op_str + " " + &rhs_str + ";\n";
        src
    }
} } }
pub(crate) use binary_rust_src;
