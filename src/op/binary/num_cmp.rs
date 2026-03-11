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
    FBinary,
};
use crate::ad::ADType;
use crate::op::id;
use crate::op::info::OpInfo;
use crate::op::binary::common;
//
// ---------------------------------------------------------------------------
// function_by_cmp
/// Define numeric compare operator functions by name of comparison
///
/// * V      : see [doc_generic_v](crate::doc_generic_v)
/// * E      : see [doc_generic_e](crate::doc_generic_e)
/// * cmp    : abbreviation for this comparison; i.e.,  lt, le, eq, ne, ge, or gt
///
/// This defines the following functions in the current module:
/// ```text
///     num_{cmp}_rust_src
///     {cmp}_forward_dyp<V, E>
///     {cmp}_forward_var<V, E>
/// ```
///
macro_rules! function_by_cmp { ($cmp:ident) => { paste::paste! {
    //
    // num_cmp_rust_src
    crate::op::binary::common::binary_rust_src!( [< num_ $cmp >] );
    //
    // cmp_forward_dyp
    #[doc = concat!(
        " E evaluation of FBinary::num_", stringify!( $cmp ),
        "; see [ForwardDyp](crate::op::info::ForwardDyp)"
    ) ]
    fn [< $cmp _forward_dyp >] <V, E> (
        dyp_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : FBinary<&'a V, Output = V> + FBinary<&'a E, Output = E>,
        for<'a> &'a E : FBinary<&'a E, Output = E> + FBinary<&'a V, Output = E>,
    {
        debug_assert!( arg.len() == 2);
        debug_assert!(
            ! ( arg_type[0].is_constant() && arg_type[1].is_constant() )
        );
        // lhs, rhs
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        //
        match( arg_type[0], arg_type[1] ) {
            (ADType::DynamicP, ADType::DynamicP) => {
                let left  = &dyp_both[lhs];
                let right = &dyp_both[rhs];
                dyp_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            (ADType::DynamicP, ADType::ConstantP) => {
                let left  = &dyp_both[lhs];
                let right = &cop[rhs];
                dyp_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            (ADType::ConstantP, ADType::DynamicP) => {
                let left  = &cop[lhs];
                let right = &dyp_both[rhs];
                dyp_both[ res ] = left. [<num_ $cmp >] ( right );
            },

            _ => { debug_assert!( false,
                    "forward_dyp: compare: invalid argument types"
            ); },
        };
    }
    //
    // cmp_forward_var
    #[doc = concat!(
        " E evaluation of FBinary::num_", stringify!( $cmp ),
        "; see [ForwardVar](crate::op::info::ForwardVar)"
    ) ]
    fn [< $cmp _forward_var >] <V, E> (
        dyp_both    : &[E]        ,
        var_both    : &mut [E]    ,
        cop         : &[V]        ,
        _flag_all   : &[bool]     ,
        arg         : &[IndexT]   ,
        arg_type    : &[ADType]   ,
        res         : usize       )
    where
        for<'a> &'a V : FBinary<&'a V, Output = V> + FBinary<&'a E, Output = E>,
        for<'a> &'a E : FBinary<&'a E, Output = E> + FBinary<&'a V, Output = E>,
    {
        debug_assert!( arg.len() == 2);
        //
        // lhs, rhs
        let lhs = arg[0] as usize;
        let rhs = arg[1] as usize;
        //
        // var_both[res]
        match( arg_type[0], arg_type[1] ) {
            // variable op constant
            (ADType::Variable, ADType::ConstantP) => {
                let left  = &var_both[lhs];
                let right = &cop[rhs];
                var_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            // variable op dynamic
            (ADType::Variable, ADType::DynamicP) => {
                let left  = &var_both[lhs];
                let right = &dyp_both[rhs];
                var_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            // variable op variable
            (ADType::Variable, ADType::Variable) => {
                let left  = &var_both[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            // constant op variable
            (ADType::ConstantP, ADType::Variable) => {
                let left  = &cop[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [<num_ $cmp >] ( right );
            },
            // dynamic op variable
            (ADType::DynamicP, ADType::Variable) => {
                let left  = &dyp_both[lhs];
                let right = &var_both[rhs];
                var_both[ res ] = left. [< num_ $cmp >] ( right );
            },
            _ => { debug_assert!(false,
                "forward_var: compare: invalid argument types"
            ); },
        };
    }
} } }
function_by_cmp!( lt );
function_by_cmp!( le );
function_by_cmp!( eq );
function_by_cmp!( ne );
function_by_cmp!( ge );
function_by_cmp!( gt );
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
/// Set the operator information for the FBinary operators
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for the following operators are set:
///   LT_OP, LE_OP, EQ_OP, NE_OP, GE_OP, GT_OP .
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    AD<V> : From<V>,
    V     : Clone + From<f32>,
    for<'a> &'a V     : FBinary<&'a V, Output = V>,
    for<'a> &'a V     : FBinary<&'a AD<V>, Output = AD<V> >,
    for<'a> &'a V     : FBinary<&'a AD<V>, Output = AD<V> >,
    for<'a> &'a AD<V> : FBinary<&'a V, Output = AD<V> >,
    for<'a> &'a AD<V> : FBinary<&'a AD<V>, Output = AD<V> >,
{
    op_info_vec[id::LT_OP as usize] = OpInfo{
        name              : "num_lt",
        forward_dyp_value : lt_forward_dyp::<V, V>,
        forward_dyp_ad    : lt_forward_dyp::<V, AD<V> >,
        forward_var_value : lt_forward_var::<V, V>,
        forward_var_ad    : lt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_lt_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[id::LE_OP as usize] = OpInfo{
        name              : "num_le",
        forward_dyp_value : le_forward_dyp::<V, V>,
        forward_dyp_ad    : le_forward_dyp::<V, AD<V> >,
        forward_var_value : le_forward_var::<V, V>,
        forward_var_ad    : le_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_le_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[id::EQ_OP as usize] = OpInfo{
        name              : "num_eq",
        forward_dyp_value : eq_forward_dyp::<V, V>,
        forward_dyp_ad    : eq_forward_dyp::<V, AD<V> >,
        forward_var_value : eq_forward_var::<V, V>,
        forward_var_ad    : eq_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_eq_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[id::NE_OP as usize] = OpInfo{
        name              : "num_ne",
        forward_dyp_value : ne_forward_dyp::<V, V>,
        forward_dyp_ad    : ne_forward_dyp::<V, AD<V> >,
        forward_var_value : ne_forward_var::<V, V>,
        forward_var_ad    : ne_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_ne_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[id::GE_OP as usize] = OpInfo{
        name              : "num_ge",
        forward_dyp_value : ge_forward_dyp::<V, V>,
        forward_dyp_ad    : ge_forward_dyp::<V, AD<V> >,
        forward_var_value : ge_forward_var::<V, V>,
        forward_var_ad    : ge_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_ge_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_info_vec[id::GT_OP as usize] = OpInfo{
        name              : "num_gt",
        forward_dyp_value : gt_forward_dyp::<V, V>,
        forward_dyp_ad    : gt_forward_dyp::<V, AD<V> >,
        forward_var_value : gt_forward_var::<V, V>,
        forward_var_ad    : gt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_gt_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
