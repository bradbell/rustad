// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Utilities that are common to all the  uniary operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use crate::IndexT;
use crate::adfn::optimize;
use crate::ad::ADType;
//
/* ---------------------------------------------------------------------------
TODO: Uncomment when optimizer handles CmpAsLhs operators.
pub(crate) fn is_unary_op(op_id : u8) -> bool {
    match op_id {
        id::SIN_OP => true ,
        //
        _         => false,
    }
}
*/
// ---------------------------------------------------------------------------
// eval_forward_var
/// Evaluation of zero order forward for unary operators.
///
/// * V      : see [doc_generic_v](crate::doc_generic_v)
/// * E      : see [doc_generic_e](crate::doc_generic_e)
/// * name   : is sin, ...
///
/// This defines the following functions in the the current module:
/// ```text
///     {name}_forward_dyp<E>
///     {name}_forward_var<E>
///     {name}_rust_src<V>
/// ```
///
macro_rules! forward_dyp{ ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        " E evaluation of ", stringify!( $name ), " for dynamic parameters",
        "; see [ForwardDer](crate::op::info::ForwardDyp)"
    ) ]
    fn [< $name _forward_dyp >] <V, E> (
        dyp_both    : &mut [E]    ,
        _cop        : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        E : FloatCore ,
    {   //
        // index
        let index = arg[0] as usize;
        debug_assert!( index < res );
        //
        debug_assert!( arg.len() == 1);
        debug_assert!( arg_type[0].is_dynamic() );
        dyp_both[ res ] = dyp_both[index].$name();
    }
}}}
pub(crate) use forward_dyp;
//
macro_rules! forward_var{ ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        " E evaluation of ", stringify!( $name ), " for variables",
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $name _forward_var >] <V, E> (
        _dyp_both   : &[E]        ,
        var_both    : &mut [E]    ,
        _cop        : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        E : FloatCore ,
    {
        //
        // index
        let index = arg[0] as usize;
        debug_assert!( index < res );
        //
        debug_assert!( arg.len() == 1);
        debug_assert!( arg_type[0].is_variable() );
        var_both[ res ] = var_both[index].$name();
    }
}}}
pub(crate) use forward_var;
//
macro_rules! rust_src { ($name:ident) => { paste::paste! {
    //
    #[doc = concat!(
        " rust source code gnerattion for ", stringify!( $op ),
        "; see [RustSrc](crate::op::info::RustSrc)"
    ) ]
    fn [< $name _rust_src >]<V> (
        _not_used   : V           ,
        res_type    : ADType      ,
        dyp_n_dom   : usize       ,
        var_n_dom   : usize       ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       ) -> String
    {   //
        // index
        let mut index = arg[0] as usize;
        debug_assert!( index < res );
        //
        debug_assert!( arg.len() == 1);
        debug_assert!( res_type == arg_type[0] );
        debug_assert!( res_type.is_dynamic() || res_type.is_variable());
        //
        let lhs_str : String;
        let rhs_str : String;
        if res_type.is_dynamic() {
            debug_assert!( dyp_n_dom <= res );
            let res  = res - dyp_n_dom;
            lhs_str  = format!("dyp_dom[{res}]");
            if index < dyp_n_dom {
                rhs_str = format!("dyp_dom[{index}]");
            } else {
                index  -= dyp_n_dom;
                rhs_str = format!("dyp_dep[{index}]");
            }
        } else {
            debug_assert!( var_n_dom <= res );
            let res  = res - var_n_dom;
            lhs_str  = format!("var_dom[{res}]");
            if index < var_n_dom {
                rhs_str = format!("var_dom[{index}]");
            } else {
                index  -= var_n_dom;
                rhs_str = format!("var_dep[{index}]");
            }
        }
        let rhs_str = rhs_str + stringify!($name) + "()";
        //
        // src
        let src = String::from("   ");
        let src = src + &lhs_str + " = " + &rhs_str + ";\n";
        src
    }
}}}
pub(crate) use rust_src;
//
/// Reverse dependency analysis for a unary operators;
/// see [ReverseDepend](crate::op::info::ReverseDepend)
pub(crate) fn reverse_depend(
    depend    : &mut optimize::Depend ,
    _flag_all : &[bool]               ,
    arg       : &[IndexT]             ,
    arg_type  : &[ADType]             ,
    res       : usize                 ,
    res_type  : ADType                ,
) { //
    debug_assert_eq!(arg.len(), 1);
    debug_assert_eq!(arg_type.len(), 1);
    debug_assert!( res_type == arg_type[0] );
    //
    // index
    let index = arg[0] as usize;
    debug_assert!( index < res );
    //
    if res_type.is_variable() {
        depend.var[index] = true;
    } else {
        depend.dyp[index] = true;
    }
}
