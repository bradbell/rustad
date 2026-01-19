// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Utilities used by the comparison operators.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
//
use crate::{
    AD,
    IndexT,
    CompareAsLeft,
    CompareAsRight,
};
use crate::adfn::optimize;
use crate::ad::ADType;
use crate::op::id;
use crate::op::info::{
    OpInfo,
    no_rust_src,
};
//
// ---------------------------------------------------------------------------
/*
TODO: Uncomment when optimizer handles CompareAsLeft operators.
pub(crate) fn is_compare_op(op_id : u8) -> bool {
    match op_id {
        id::LT_OP => true ,
        id::LE_OP => true ,
        id::EQ_OP => true ,
        id::NE_OP => true ,
        id::GE_OP => true ,
        id::GT_OP => true ,
        //
        _         => false,
    }
}
*/
// ---------------------------------------------------------------------------
// eval_compare_forward_fun
/// Evaluation of forward function values for compare operators.
///
/// * V      : see [doc_generic_v](crate::doc_generic_v)
/// * E      : see [doc_generic_e](crate::doc_generic_e)
/// * name   : is  lt, le, eq, ne, ge, or gt
///
/// This defines the following function in the current module:
/// ```text
///     {name}_forward_dyp<V, E>
///     {name}_forward_var<V, E>
/// ```
///
/// [IndexT] must be defined in any module that uses eval_compare_op
macro_rules! eval_compare_forward_fun { ($name:ident) => { paste::paste! {
    #[doc = concat!(
        " E zero order forward for dynamic parameter num_",
        stringify!( $name ),
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
        V : CompareAsLeft<V> + CompareAsRight<E>,
        E : CompareAsLeft<E> + CompareAsLeft<V>,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!(
            ! ( arg_type[0].is_constant() && arg_type[1].is_constant() )
        );
        // lhs, rhs
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        //
        match( arg_type[0].clone(), arg_type[1].clone() ) {
            (ADType::DynamicP, ADType::DynamicP) => {
                let left  = &dyp_both[lhs];
                let right = &dyp_both[rhs];
                dyp_both[ res ] = left. [< left_ $name >] ( right );
            },
            (ADType::DynamicP, ADType::ConstantP) => {
                let left  = &dyp_both[lhs];
                let right = &cop[rhs];
                dyp_both[ res ] = left. [< left_ $name >] ( right );
            },
            (ADType::ConstantP, ADType::DynamicP) => {
                let left  = &cop[lhs];
                let right = &dyp_both[rhs];
                dyp_both[ res ] = left. [< $name _right >] ( right );
            },

            _ => { debug_assert!( false,
                    "forward_dyp: compare: invalid argument types"
            ); },
        };
    }
    #[doc = concat!(
        " E zero order forward variable num_", stringify!( $name ),
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $name _forward_var >] <V, E> (
        dyp_both    : &[E]        ,
        var_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        V : CompareAsLeft<V> + CompareAsRight<E>,
        E : CompareAsLeft<E> + CompareAsLeft<V>,
    {
        debug_assert!( arg.len() == 2);
        //
        // lhs, rhs
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        //
        // var_both[res]
        match( arg_type[0].clone(), arg_type[1].clone() ) {
            // variable op constant
            (ADType::Variable, ADType::ConstantP) => {
                let left  = &var_both[lhs];
                let right = &cop[rhs];
                var_both[ res ] = left. [< left_ $name >] ( right );
            },
            // variable op dynamic
            (ADType::Variable, ADType::DynamicP) => {
                let left  = &var_both[lhs];
                let right = &dyp_both[rhs];
                var_both[ res ] = left. [< left_ $name >] ( right );
            },
            // variable op variable
            (ADType::Variable, ADType::Variable) => {
                let left  = &var_both[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [< left_ $name >] ( right );
            },
            // constant op variable
            (ADType::ConstantP, ADType::Variable) => {
                let left  = &cop[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [< $name _right >] ( right );
            },
            // dynamic op variable
            (ADType::DynamicP, ADType::Variable) => {
                let left  = &dyp_both[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [< left_ $name >] ( right );
            },
            _ => { debug_assert!(false,
                "forward_var: compare: invalid argument types"
            ); },
        };
    }
} } }
eval_compare_forward_fun!( lt );
eval_compare_forward_fun!( le );
eval_compare_forward_fun!( eq );
eval_compare_forward_fun!( ne );
eval_compare_forward_fun!( ge );
eval_compare_forward_fun!( gt );
// ---------------------------------------------------------------------------
// binary_reverse_depend
/// Reverse dependency analysis for a compare operator;
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
                _ => { panic!("in compare operator reverse_depend"); },
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
                _ => { panic!("in compare operator reverse_depend"); },
            }
        }
    }
}
// ---------------------------------------------------------------------------
// zero_forward_der
fn zero_forward_der<V, E>  (
    _dyp_both : &[E]        ,
    _var_both : &[E]        ,
    var_der   : &mut [E]    ,
    _cop      : &[V]        ,
    _flag_all : &[bool]     ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    res        : usize      ,
)
where
    E : From<V>,
    V : From<f32>,
{
    var_der [ res ] = V::from(0.0f32).into();
}
// ---------------------------------------------------------------------------
// zero_reverse_der
fn zero_reverse_der<V, E>  (
    _dyp_both : &[E]        ,
    _var_both : &[E]        ,
    _var_der  : &mut [E]    ,
    _cop      : &[V]        ,
    _flag_all : &[bool]     ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _re       : usize      ,
) {  }
// ---------------------------------------------------------------------------
// set_op_info
//
// rust_src_none
no_rust_src!(CompareAsLeft);
//
/// Set the operator information for the CompareAsLeft operators
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for the following operators are set:
///   LT_OP, LE_OP, EQ_OP, NE_OP, GE_OP, GT_OP .
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    V     : Clone + From<f32>,
    V     :  CompareAsLeft<V>  + CompareAsRight< AD<V> >,
    V     :  CompareAsRight<V> + CompareAsRight< AD<V> >,
    AD<V> : From<V>,
    AD<V> :  CompareAsLeft<V> + CompareAsLeft< AD<V> >,
{
    op_info_vec[id::LT_OP as usize] = OpInfo{
        name              : "lt",
        forward_dyp_value : lt_forward_dyp::<V, V>,
        forward_dyp_ad    : lt_forward_dyp::<V, AD<V> >,
        forward_var_value : lt_forward_var::<V, V>,
        forward_var_ad    : lt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
    op_info_vec[id::LE_OP as usize] = OpInfo{
        name              : "le",
        forward_dyp_value : le_forward_dyp::<V, V>,
        forward_dyp_ad    : le_forward_dyp::<V, AD<V> >,
        forward_var_value : le_forward_var::<V, V>,
        forward_var_ad    : le_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
    op_info_vec[id::EQ_OP as usize] = OpInfo{
        name              : "eq",
        forward_dyp_value : eq_forward_dyp::<V, V>,
        forward_dyp_ad    : eq_forward_dyp::<V, AD<V> >,
        forward_var_value : eq_forward_var::<V, V>,
        forward_var_ad    : eq_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
    op_info_vec[id::NE_OP as usize] = OpInfo{
        name              : "ne",
        forward_dyp_value : ne_forward_dyp::<V, V>,
        forward_dyp_ad    : ne_forward_dyp::<V, AD<V> >,
        forward_var_value : ne_forward_var::<V, V>,
        forward_var_ad    : ne_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
    op_info_vec[id::GE_OP as usize] = OpInfo{
        name              : "ge",
        forward_dyp_value : ge_forward_dyp::<V, V>,
        forward_dyp_ad    : ge_forward_dyp::<V, AD<V> >,
        forward_var_value : ge_forward_var::<V, V>,
        forward_var_ad    : ge_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
    op_info_vec[id::GT_OP as usize] = OpInfo{
        name              : "gt",
        forward_dyp_value : gt_forward_dyp::<V, V>,
        forward_dyp_ad    : gt_forward_dyp::<V, AD<V> >,
        forward_var_value : gt_forward_var::<V, V>,
        forward_var_ad    : gt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : rust_src_none,
        reverse_depend    : binary_reverse_depend,
    };
}
