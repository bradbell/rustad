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
use crate::op::no_op::{
    no_op_dyp,
    no_op_var,
    no_op_der,
};
use crate::op::info::{
    OpFns,
    panic_reverse_depend,
    ConstData,
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
// --------------------------------------------------------------------------
// zero_one_forward_var_value
/// Zero One operator V evaluation of dynamic parameters;
/// see [ForwardDyp](crate::op::info::ForwardDyp)
fn zero_one_forward_var_value<V> (
    _dyp_all   : &[V]          ,
    var_all    : &mut [V]      ,
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
    debug_assert!( arg_type[3] == ADType::Variable );
    //
    // check_one, check_result
    let start        = arg[0] as usize;
    let check_one    = bool_all[start];
    let panic        = bool_all[start + 1];
    let check_result = bool_all[start + 2];
    //
    // value
    let index = arg[3] as usize;
    let value = &var_all[index];
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
// --------------------------------------------------------------------------
// zero_one_rust_src
fn zero_one_rust_src<V> (
    res_type    : ADType      ,
    dyp_n_dom   : usize       ,
    var_n_dom   : usize       ,
    const_data : ConstData<V> ) -> String
{   //
    let ConstData{bool_all, str_all, arg, arg_type, ..} = const_data;
    //
    debug_assert!( arg_type.len() == 4 );
    for arg_type_i in arg_type.iter().take(3) {
        debug_assert!( *arg_type_i == ADType::Empty );
    }
    debug_assert!( arg_type[3] == res_type);
    debug_assert!( res_type.is_dynamic() || res_type.is_variable());
    //
    // check_one, panic, check_result
    let start        = arg[0] as usize;
    let check_one    = bool_all[start];
    let panic        = bool_all[start + 1];
    let check_result = bool_all[start + 2];
    //
    // message
    let start   = arg[1] as usize;
    let end     = arg[2] as usize;
    let message = &str_all[start .. end];
    //
    // fn_name
    let fn_name = if check_one { "is_one" } else { "is_zero" };  
    //
    // panic
    if ! panic {
        panic!(
            "rust_src: {}: Can't convert to src because both panic and ignore \
            are false.\nmessage = {}",
            fn_name, message
        );
    }
    //
    // arg_str
    let arg_str : String;
    let mut index = arg[3] as usize;
    if res_type.is_dynamic() {
        if index < dyp_n_dom {
            arg_str = format!("dyp_dom[{index}]");
        } else {
            index  -= dyp_n_dom;
            arg_str = format!("dyp_dep[{index}]");
        }
    } else {
        if index < var_n_dom {
            arg_str = format!("var_dom[{index}]");
        } else {
            index  -= var_n_dom;
            arg_str = format!("var_dep[{index}]");
        }
    }
    //
    // result_str
    let result_str = arg_str + "." + fn_name + "()";
    //
    // src
    let mut src = String::new();
    if check_result == true {
        src = src + 
            "   let zero_one_check = true;\n";
    } else {
        src = src + 
            "   let zero_one_check = false;\n";
    }
    src = src +
        "   let zero_one_result = " + &result_str + ";\n" +
        "   let zero_one_message = \"" + &message + "\";\n" +
        "   if zero_one_result != zero_one_check {\n" +
        "       panic!(\"{}\", zero_one_message);\n" +
        "   }\n";
    src
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
        forward_dyp_ad    : no_op_dyp::<V, AD<V> >,
        forward_var_value : zero_one_forward_var_value::<V>,
        forward_var_ad    : no_op_var::<V, AD<V> >,
        forward_der_value : no_op_der::<V, V>,
        forward_der_ad    : no_op_der::<V, AD<V> >,
        reverse_der_value : no_op_der::<V, V>,
        reverse_der_ad    : no_op_der::<V, AD<V> >,
        rust_src          : zero_one_rust_src,
        reverse_depend    : panic_reverse_depend,
    };
}
