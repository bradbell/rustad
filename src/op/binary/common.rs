// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Utilities used by the binary operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use crate::IndexT;
use crate::adfn::optimize;
use crate::ad::ADType;
use crate::op::id;
//
// ---------------------------------------------------------------------------
pub(crate) fn is_binary_op(op_id : u8) -> bool {
    match op_id {
        id::ADD_PP_OP => true ,
        id::ADD_PV_OP => true ,
        id::ADD_VP_OP => true ,
        id::ADD_VV_OP => true ,
        //
        id::SUB_PP_OP => true ,
        id::SUB_PV_OP => true ,
        id::SUB_VP_OP => true ,
        id::SUB_VV_OP => true ,
        //
        id::MUL_PP_OP => true ,
        id::MUL_PV_OP => true ,
        id::MUL_VP_OP => true ,
        id::MUL_VV_OP => true ,
        //
        id::DIV_PP_OP => true ,
        id::DIV_PV_OP => true ,
        id::DIV_VP_OP => true ,
        id::DIV_VV_OP => true ,
        //
        id::LT_OP     => true  ,
        id::LE_OP     => true  ,
        id::EQ_OP     => true  ,
        id::NE_OP     => true  ,
        id::GE_OP     => true  ,
        id::GT_OP     => true  ,
        //
        _         => false,
    }
}
// ---------------------------------------------------------------------------
// binary_forward_var
/// Evaluation of zero order forward for binary operators;
/// see [num_cmp](crate::op::binary::num_cmp)
/// for numerical comparison operators.
///
///
/// * V      : see [doc_generic_v](crate::doc_generic_v)
/// * E      : see [doc_generic_e](crate::doc_generic_e)
/// * Trait  : is Add , Sub , Mul , Div, or Powf
/// * name   : is the name of a function in this trait
/// * op     : is the corresponding operator; e.g. + for Add.
///
/// This defines the following functions in the current module:
/// ```text
///     {name}_forward_dyp<V, E>
///     {name}_pv_forward_var<V, E>
///     {name}_vp_forward_var<V, E>
///     {name}_vv_forward_var<V, E>
/// ```
/// where v (p) means the corresponding operand is a variable (parameter) .
///
/// [IndexT] must be defined in any module that uses binary_forward_var
macro_rules! binary_forward_var { ($Trait:ident, $name:ident) =>
{ paste::paste! {
    #[doc = concat!(
        " E evaluation of ", stringify!($name), "(parameter, parameter)",
        "; see [ForwardDyp](crate::op::info::ForwardDyp)"
    ) ]
    fn [< $name _forward_dyp >] <V, E> (
        dyp_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : $Trait<&'a E, Output = E> ,
        for<'a> &'a E : $Trait<&'a V, Output = E> ,
        for<'a> &'a E : $Trait<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!(
            ! ( arg_type[0].is_constant() && arg_type[1].is_constant() )
        );
        debug_assert!(
            ! ( arg_type[0].is_variable() || arg_type[1].is_variable() )
        );
        let lhs       = arg[0] as usize;
        let rhs       = arg[1] as usize;
        if arg_type[0].is_constant() {
            dyp_both[res] = (&cop[lhs]).$name (&dyp_both[rhs]);
        } else if arg_type[1].is_constant() {
            dyp_both[res] = (&dyp_both[lhs]).$name (&cop[rhs]);
        } else {
            dyp_both[res] = (&dyp_both[lhs]).$name (&dyp_both[rhs]);
        };
    }
    #[doc = concat!(
        " E evaluation of ", stringify!($name), "(parameter, variable)",
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $name _pv_forward_var >] <V, E> (
        dyp_both    : &[E]        ,
        var_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : $Trait<&'a E, Output = E> ,
        for<'a> &'a E : $Trait<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!( ! arg_type[1].is_constant() );
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        if arg_type[0].is_constant() {
            var_both[res] = (&cop[lhs]).$name (&var_both[rhs]);
        } else {
            var_both[res] = (&dyp_both[lhs]).$name (&var_both[rhs]);
        }
    }
    #[doc = concat!(
        " E evaluation of ", stringify!($name), "(variable, parameter)",
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $name _vp_forward_var >] <V, E> (
        dyp_both    : &[E]        ,
        var_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a E : $Trait<&'a V, Output = E> ,
        for<'a> &'a E : $Trait<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!( ! arg_type[0].is_constant() );
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        if arg_type[1].is_constant() {
            var_both[res] = (&var_both[lhs]).$name (&cop[rhs]);
        } else {
            var_both[res] = (&var_both[lhs]).$name (&dyp_both[rhs]);
        }
    }
    #[doc = concat!(
        " E evaluation of ", stringify!($name), "(variable, variable)",
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $name _vv_forward_var >] <V, E> (
        _dyp_both   : &[E]        ,
        var_both    : &mut [E]    ,
        _cop        : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        _arg_type   : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a E : $Trait<&'a E, Output = E> ,
    {
        debug_assert!( arg.len() == 2);
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        var_both[res] = (&var_both[lhs]).$name (&var_both[rhs]);
    }
} } }
pub(crate) use binary_forward_var;
// ---------------------------------------------------------------------------
// binary_rust_src
/// Rust source code for binary operators.
///
/// * name :
///   add, sub, mul, div, powf,
///   num_lt, num_le, num_eq, num_ne, num_ge, num_gt
///
/// This defines the following functions in the current module:
/// ```text
///     {name}_rust_src
/// ```
///
/// [IndexT] must be defined in any module that uses binary_rust_src
macro_rules! binary_rust_src { ($name:ident) => { paste::paste! {
    #[doc = concat!(
        " rust source for ", stringify!( $name ),
        "; see [RustSrc](crate::op::info::RustSrc)"
    ) ]
    fn [< $name _rust_src >]<V> (
        _not_used  : V            ,
        res_type    : ADType      ,
        dyp_n_dom   : usize       ,
        var_n_dom   : usize       ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       ) -> String
    {   //
        debug_assert!( arg.len() == 2);
        //
        // lhs_str
        let lhs_str : String;
        let mut lhs = arg[0] as usize;
        match arg_type[0] {
            //
            // ConstantP
            ADType::ConstantP => {
                lhs_str = format!("(&cop[{lhs}])");
            },
            //
            // DynamicP
            ADType::DynamicP => {
                if lhs < dyp_n_dom  {
                    lhs_str = format!("dyp_dom[{lhs}]");
                } else {
                    lhs -= dyp_n_dom;
                    lhs_str = format!("(&dyp_dep[{lhs}])");
                }
            },
            //
            // Variable
            ADType::Variable => {
                if lhs < var_n_dom  {
                    lhs_str = format!("var_dom[{lhs}]");
                } else {
                    lhs -= var_n_dom;
                    lhs_str = format!("(&var_dep[{lhs}])");
                }
            },
            //
            _ => {
                panic!("binary_rust_src: invalid arg_type[0]");
            },
        }
        //
        // rhs_str
        let rhs_str : String;
        let mut rhs = arg[1] as usize;
        match arg_type[1] {
            //
            // ConstantP
            ADType::ConstantP => {
                rhs_str = format!("&cop[{rhs}]");
            },
            //
            // DynamicP
            ADType::DynamicP => {
                if rhs < dyp_n_dom  {
                    rhs_str = format!("dyp_dom[{rhs}]");
                } else {
                    rhs -= dyp_n_dom;
                    rhs_str = format!("&dyp_dep[{rhs}]");
                }
            },
            //
            // Variable
            ADType::Variable => {
                if rhs < var_n_dom  {
                    rhs_str = format!("var_dom[{rhs}]");
                } else {
                    rhs -= var_n_dom;
                    rhs_str = format!("&var_dep[{rhs}]");
                }
            },
            //
            _ => {
                panic!("binary_rust_src: invalid arg_type[1]");
            },
        }
        //
        // res_str
        let res_str : String = if res_type.is_dynamic() {
            let res = res - dyp_n_dom;
            format!("dyp_dep[{res}]")
        } else {
            debug_assert!( res_type.is_variable() );
            let res = res - var_n_dom;
            format!("var_dep[{res}]")
        };
        //
        // op_name
        let op_name = stringify!( $name );
        //
        // src
        let src = String::from("   ");
        let src = src + &res_str +
            " = " + &lhs_str + "." + op_name + "(" + &rhs_str + ");\n";
        src
    }
} } }
pub(crate) use binary_rust_src;
// ---------------------------------------------------------------------------
// binary_reverse_depend
/// Reverse dependency analysis for a binary operator;
/// see [ReverseDepend](crate::op::info::ReverseDepend)
///
pub(crate) fn binary_reverse_depend(
    depend    : &mut optimize::Depend ,
    _flag_all : &[bool]               ,
    arg       : &[IndexT]             ,
    arg_type  : &[ADType]             ,
    res       : usize                 ,
    res_type  : ADType                ,
) { //
    debug_assert_eq!(arg.len(), 2);
    debug_assert_eq!(arg_type.len(), 2);
    //
    if res_type.is_variable() {
        debug_assert!( depend.var[res] );
        for i_arg in 0 .. 2 {
            let index = arg[i_arg] as usize;
            match arg_type[i_arg] {
                //
                ADType::ConstantP => { depend.cop[index] = true; },
                ADType::DynamicP  => { depend.dyp[index] = true; },
                ADType::Variable  => { depend.var[index] = true; },
                _ => { panic!("in binary operator reverse_depend"); },
            }
        }
    } else {
        debug_assert!( res_type.is_dynamic() );
        debug_assert!( depend.dyp[res] );
        for i_arg in 0 .. 2 {
            let index = arg[i_arg] as usize;
            match arg_type[i_arg] {
                //
                ADType::ConstantP => { depend.cop[index] = true; },
                ADType::DynamicP  => { depend.dyp[index] = true; },
                _ => { panic!("in binary operator reverse_depend"); },
            }
        }
    }
}
