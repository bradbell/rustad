// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
// --------------------------------------------------------------------------
//! Operator that calls an atomic function
//!
//! Link to [parent module](super)
//!
//! # ZERO_ONE_OP
//!
//! # Operator Arguments
//! | Index    | Meaning |
//! | -------  | ------- |
//! | 0        | Index in bool_all of first boolean for this operator         |
//! | 1        | Index in str_all of beginning of message for this operator   |
//! | 2        | Index in str_all of the end of message for this operator     |
//! | 3        | Variable, dynamic, or constant index for value being checked |
//!
//! # Operator Booleans
//! | Index    | Meaning |
//! | -------- | ------- |
//! | 0        | is false (true) if this operator checks for zero (one)      |
//! | 1        | if true, operator should panic if the result is different   |
//! | 2        | is the result of the check when this operation was recorded |
//!
// --------------------------------------------------------------------------
// use
use crate::{
    AD,
    FloatValue,
};
use crate::op::id;
use crate::ad::ADType;
use crate::ad::zero_one::push_zero_one_message;
use crate::op::info::ConstData;
use crate::op::info::{
    OpFns,
    panic_dyp,
    panic_var,
    panic_der,
    panic_rust_src,
    panic_reverse_depend,
};
// --------------------------------------------------------------------------
// zero_one_forward_dyp_value
/// Zero One operator V evaluation of dynamic parameters;
/// see [ForwardDyp](crate::op::info::ForwardDyp)
fn zero_one_forward_dyp_value<V> (
    dyp_all    : &mut [V]      ,
    const_data : ConstData<V>  )
where
    V : FloatValue,
{   //
    let ConstData{bool_all, str_all, arg, arg_type, ..} = const_data;
    //
    debug_assert!( arg_type.len() == 4 );
    for arg_type_i in arg_type.iter().take(3) {
        debug_assert!( *arg_type_i == ADType::Empty );
    }
    debug_assert!( arg_type[3] == ADType::DynamicP );
    //
    // check_one, check_result
    let start        = arg[0] as usize;
    let check_one    = bool_all[start];
    let panic        = bool_all[start + 1];
    let check_result = bool_all[start + 2];
    //
    // value
    let index = arg[3] as usize;
    let value = &dyp_all[index];
    //
    // same
    let same = if check_one {
        check_result == value.is_one()
    } else {
        check_result == value.is_zero()
    };
    if same {
        return;
    }
    //
    // message
    let start   = arg[1] as usize;
    let end     = arg[2] as usize;
    let message = &str_all[start .. end];
    if panic {
        panic!( "{}", message );
    } else {
        push_zero_one_message( message.to_string() );
    }
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the ZERO_ONE_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for POWI_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] )
where
    V : FloatValue,
{
    op_fns_vec[id::ZERO_ONE_OP as usize] = OpFns{
        name              : "zero_one",
        forward_dyp_value : zero_one_forward_dyp_value::<V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : panic_rust_src,
        reverse_depend    : panic_reverse_depend,
    };
}
