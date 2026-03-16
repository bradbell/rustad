// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
//! This module defines the powi operator
//!
//! Link to [parent module](super)
//!
//! * Operator : POWI_OP
//!
//! # Operator Arguments
//! | Index | Meaning |
//! | ----- | ------- |
//! | 0     | Variable, dynamic, or constant index for left hand side       |
//! | 1     | is the absolute value of the exponent this call               |
//! | 2     | is 0 (1) if the exponent for this call is positive (negative) |
// ---------------------------------------------------------------------------
use std::ops::{
    Mul,
    AddAssign,
};
use crate::{
    AD,
    FConst,
    FUnary,
    IndexT,
};
use crate::tape::sealed::ThisThreadTape;
use crate::ad::ADType;
use crate::adfn::optimize;
use crate::op::info::OpFns;
use crate::op::id::POWI_OP;
// ---------------------------------------------------------------------------
//
// pow_forward_dyp
fn powi_forward_dyp <V, E> (
    dyp_all     : &mut [E]    ,
    _cop        : &[V]        ,
    _bool_all   : &[bool]     ,
    arg         : &[IndexT]   ,
    arg_type    : &[ADType]   ,
    res         : usize       )
where
    E : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
{   //
    // index
    let index    = arg[0] as usize;
    debug_assert!( index < res );
    debug_assert!( arg.len() == 3);
    debug_assert!( arg_type[0].is_dynamic() );
    debug_assert!( arg_type[1].is_empty() );
    debug_assert!( arg_type[2].is_empty() );
    debug_assert!( arg[1] as usize <= i32::MAX as usize );
    //
    let positive    = arg[2] == 0;
    let exponent    = if positive { arg[1] as i32 } else { - (arg[1] as i32) };
    dyp_all[ res ] = dyp_all[index].powi(exponent);
}
//
// powi_forward_var
fn powi_forward_var <V, E> (
    _dyp_all    : &[E]        ,
    var_all     : &mut [E]    ,
    _cop        : &[V]        ,
    _bool_all   : &[bool]     ,
    arg         : &[IndexT]   ,
    arg_type    : &[ADType]   ,
    res         : usize       )
where
    E : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
{   //
    // index
    let index    = arg[0] as usize;
    debug_assert!( index < res );
    debug_assert!( arg.len() == 3);
    debug_assert!( arg_type[0].is_variable() );
    debug_assert!( arg_type[1].is_empty() );
    debug_assert!( arg_type[2].is_empty() );
    debug_assert!( arg[1] as usize <= i32::MAX as usize );
    //
    let positive     = arg[2] == 0;
    let exponent     = if positive { arg[1] as i32 } else { - (arg[1] as i32) };
    var_all[ res ] = var_all[index].powi(exponent);
}
//
// powi_rust_src
fn powi_rust_src<V> (
    _not_used   : V           ,
    res_type    : ADType      ,
    dyp_n_dom   : usize       ,
    var_n_dom   : usize       ,
    _bool_all   : &[bool]     ,
    arg         : &[IndexT]   ,
    arg_type    : &[ADType]   ,
    res         : usize       ) -> String
{   //
    debug_assert!( (arg[0] as usize) < res );
    debug_assert!( arg.len() == 3);
    debug_assert!( res_type == arg_type[0] );
    debug_assert!( res_type.is_dynamic() || res_type.is_variable());
    //
    // lhs_str
    let mut lhs = arg[0] as usize;
    let lhs_str : String;
    if res_type.is_dynamic() {
        if lhs < dyp_n_dom {
            lhs_str = format!("dyp_dom[{lhs}]");
        } else {
            lhs    -= dyp_n_dom;
            lhs_str = format!("dyp_dep[{lhs}]");
        }
    } else {
        debug_assert!( res_type.is_variable() );
        if lhs < var_n_dom {
            lhs_str = format!("var_dom[{lhs}]");
        } else {
            lhs    -= var_n_dom;
            lhs_str = format!("var_dep[{lhs}]");
        }
    }
    //
    // res_str
    let res_str : String;
    if res_type.is_dynamic() {
        if res < dyp_n_dom {
            res_str = format!("dyp_dom[{res}]");
        } else {
            let res = res - dyp_n_dom;
            res_str = format!("dyp_dep[{res}]");
        }
    } else {
        debug_assert!( res_type.is_variable() );
        if res < var_n_dom {
            res_str = format!("var_dom[{res}]");
        } else {
            let res = res - var_n_dom;
            res_str = format!("var_dep[{res}]");
        }
    }
    //
    // rhs_str
    let positive = arg[2] == 0;
    let exponent = if positive { arg[1] as i32 } else { - (arg[1] as i32) };
    let rhs_str  = format!("{exponent}");
    //
    // src
    String::from("   ") + &res_str + " = " +
        &lhs_str + ".powi(" + &rhs_str + ");\n"
}
//
// powi_reverse_depend
/// Reverse dependency analysis for powi operator;
/// see [ReverseDepend](crate::op::info::ReverseDepend)
pub(crate) fn powi_reverse_depend(
    depend    : &mut optimize::Depend ,
    _bool_all : &[bool]               ,
    arg       : &[IndexT]             ,
    arg_type  : &[ADType]             ,
    res       : usize                 ,
    res_type  : ADType                ,
) { //
    debug_assert_eq!(arg.len(), 3);
    debug_assert_eq!(arg_type.len(), 3);
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
//
// powi_forward_der
/// First order forward mode for powi(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn powi_forward_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : From<f32>,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a E : Mul<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 3 );
    debug_assert!( arg_type[0].is_variable() );
    let positive  = arg[2] == 0;
    let rhs       = if positive { arg[1] as i32 } else { - (arg[1] as i32) };
    let lhs       = arg[0] as usize;
    if rhs == 0 {
        var_der[res] = FConst::zero();
    } else {
        let dpowi    = &( var_all[lhs].powi(rhs-1) ) *  &V::from(rhs as f32);
        var_der[res] = &dpowi *  &var_der[lhs];
    }
}
// powi_reverse_der
/// First order reverse mode for powi(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn powi_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    var_all    :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    V             : From<f32>,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> E     : AddAssign<&'a E> ,
    for<'a> &'a E : Mul<&'a E, Output=E>,
    for<'a> &'a E : Mul<&'a V, Output=E>,
{
    debug_assert!( arg.len() == 3 );
    debug_assert!( arg_type[0].is_variable() );
    let positive  = arg[2] == 0;
    let rhs       = if positive { arg[1] as i32 } else { - (arg[1] as i32) };
    let lhs       = arg[0] as usize;
    if rhs != 0 {
        let dpowi    = &( var_all[lhs].powi(rhs-1) ) * &V::from(rhs as f32);
        let term     = &dpowi * &var_der[res];
        var_der[lhs] += &term;
    }
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the POWI_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for POWI_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a AD<V> : Mul<&'a V, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    V                 : Clone + FConst + ThisThreadTape + From<f32>,
    for<'a> &'a V     : FUnary<Output=V>,
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
{
    op_fns_vec[POWI_OP as usize] = OpFns{
        name              : "powi",
        forward_dyp_value : powi_forward_dyp::<V, V>,
        forward_dyp_ad    : powi_forward_dyp::<V, AD<V> >,
        forward_var_value : powi_forward_var::<V, V>,
        forward_var_ad    : powi_forward_var::<V, AD<V> >,
        forward_der_value : powi_forward_der::<V, V>,
        forward_der_ad    : powi_forward_der::<V, AD<V> >,
        reverse_der_value : powi_reverse_der::<V, V>,
        reverse_der_ad    : powi_reverse_der::<V, AD<V> >,
        rust_src          : powi_rust_src,
        reverse_depend    : powi_reverse_depend,
    };
}
