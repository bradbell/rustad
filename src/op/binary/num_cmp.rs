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
    FConst,
    FBinary,
};
use crate::ad::ADType;
use crate::op::id;
use crate::op::info::OpFns;
use crate::op::info::ConstData;
use crate::op::binary::common;
// ---------------------------------------------------------------------------
// For each name, define
// {name}_rust_src
// {name}_forward_dyp<V, E>
// {name}_forward_var<V, E>
//
common::f_binary_function!( num_lt );
common::f_binary_function!( num_le );
common::f_binary_function!( num_eq );
common::f_binary_function!( num_ne );
common::f_binary_function!( num_ge );
common::f_binary_function!( num_gt );
// ---------------------------------------------------------------------------
// zero_forward_der
fn zero_forward_der<V, E>  (
    _dyp_all  : &[E]        ,
    _var_all  : &[E]        ,
    var_der   : &mut [E]    ,
    const_data : ConstData<V> ,
)
where
    E : FConst,
{
    let ConstData {res, ..} = const_data;
    //
    var_der [ res ] = FConst::zero();
}
// ---------------------------------------------------------------------------
// zero_reverse_der
fn zero_reverse_der<V, E>  (
    _dyp_all  : &[E]        ,
    _var_all  : &[E]        ,
    _var_der  : &mut [E]    ,
    _const_data : ConstData<V> ,
) {  }
// ---------------------------------------------------------------------------
// set_op_fns
//
/// Set the operator functions for the FBinary operators
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for the following operators are set:
///   LT_OP, LE_OP, EQ_OP, NE_OP, GE_OP, GT_OP .
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] )
where
    AD<V> : FConst,
    V     : Clone + FConst,
    for<'a> &'a V     : FBinary<&'a V, Output = V>,
    for<'a> &'a V     : FBinary<&'a AD<V>, Output = AD<V> >,
    for<'a> &'a V     : FBinary<&'a AD<V>, Output = AD<V> >,
    for<'a> &'a AD<V> : FBinary<&'a V, Output = AD<V> >,
    for<'a> &'a AD<V> : FBinary<&'a AD<V>, Output = AD<V> >,
{
    op_fns_vec[id::LT_OP as usize] = OpFns{
        name              : "num_lt",
        forward_dyp_value : num_lt_forward_dyp::<V, V>,
        forward_dyp_ad    : num_lt_forward_dyp::<V, AD<V> >,
        forward_var_value : num_lt_forward_var::<V, V>,
        forward_var_ad    : num_lt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_lt_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[id::LE_OP as usize] = OpFns{
        name              : "num_le",
        forward_dyp_value : num_le_forward_dyp::<V, V>,
        forward_dyp_ad    : num_le_forward_dyp::<V, AD<V> >,
        forward_var_value : num_le_forward_var::<V, V>,
        forward_var_ad    : num_le_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_le_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[id::EQ_OP as usize] = OpFns{
        name              : "num_eq",
        forward_dyp_value : num_eq_forward_dyp::<V, V>,
        forward_dyp_ad    : num_eq_forward_dyp::<V, AD<V> >,
        forward_var_value : num_eq_forward_var::<V, V>,
        forward_var_ad    : num_eq_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_eq_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[id::NE_OP as usize] = OpFns{
        name              : "num_ne",
        forward_dyp_value : num_ne_forward_dyp::<V, V>,
        forward_dyp_ad    : num_ne_forward_dyp::<V, AD<V> >,
        forward_var_value : num_ne_forward_var::<V, V>,
        forward_var_ad    : num_ne_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_ne_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[id::GE_OP as usize] = OpFns{
        name              : "num_ge",
        forward_dyp_value : num_ge_forward_dyp::<V, V>,
        forward_dyp_ad    : num_ge_forward_dyp::<V, AD<V> >,
        forward_var_value : num_ge_forward_var::<V, V>,
        forward_var_ad    : num_ge_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_ge_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
    op_fns_vec[id::GT_OP as usize] = OpFns{
        name              : "num_gt",
        forward_dyp_value : num_gt_forward_dyp::<V, V>,
        forward_dyp_ad    : num_gt_forward_dyp::<V, AD<V> >,
        forward_var_value : num_gt_forward_var::<V, V>,
        forward_var_ad    : num_gt_forward_var::<V, AD<V> >,
        forward_der_value : zero_forward_der::<V, V>,
        forward_der_ad    : zero_forward_der::<V, AD<V> >,
        reverse_der_value : zero_reverse_der::<V, V>,
        reverse_der_ad    : zero_reverse_der::<V, AD<V> >,
        rust_src          : num_gt_rust_src,
        reverse_depend    : common::binary_reverse_depend,
    };
}
