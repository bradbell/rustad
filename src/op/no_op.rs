// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the No Op operator
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
// --------------------------------------------------------------------------
// use
//
use crate::{
    IndexT,
    AD,
};
use crate::ad::ADType;
use crate::adfn::optimize;
//
use crate::op::info::OpInfo;
use crate::op::id::NO_OP;
//
// no_op_dyp
fn no_op_dyp<V, E> (
    _dyp_both : &mut [E]    ,
    _cop      : &[V]        ,
    _flag_all : &[bool]     ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// no_op_var
fn no_op_var<V, E> (
    _dyp_both : &[E]        ,
    _var_both : &mut [E]    ,
    _cop      : &[V]        ,
    _flag_all : &[bool]     ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) { }
//
// no_op_der
fn no_op_der<V, E>  (
    _dyp_both : &[E]        ,
    _var_both : &[E]        ,
    _var_der  : &mut [E]    ,
    _cop      : &[V]        ,
    _flag_all : &[bool]     ,
    _arg      : &[IndexT]   ,
    _arg_type : &[ADType]   ,
    _res      : usize       ,
) {  }
//
// no_op_rust_src
pub fn no_op_rust_src<V>(
    _not_used : V           ,
    _res_type  : ADType      ,
    _dyp_n_dom : usize       ,
    _var_n_dom : usize       ,
    _flag_all  : &[bool]     ,
    _arg       : &[IndexT]   ,
    _arg_type  : &[ADType]   ,
    _res       : usize       ,
) -> String
{ String::new()  }
//
// no_op_reverse_depend
pub fn no_op_reverse_depend<V>(
    _depend   : &mut optimize::Depend ,
    _flag_all : &[bool]               ,
    _arg      : &[IndexT]             ,
    _arg_type : &[ADType]             ,
    _res      : usize                 ,
    _res_type : ADType                ,
) { }
//
/// Set the operator information for all the Sub operators.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for SUB_PV_OP, SUB_VP_OP, and SUB_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
{
    op_info_vec[NO_OP as usize] = OpInfo{
        name              : "no_op",
        forward_dyp_value : no_op_dyp::<V, V>,
        forward_dyp_ad    : no_op_dyp::<V, AD<V> >,
        forward_var_value : no_op_var::<V, V>,
        forward_var_ad    : no_op_var::<V, AD<V> >,
        forward_der_value : no_op_der::<V, V>,
        forward_der_ad    : no_op_der::<V, AD<V> >,
        reverse_der_value : no_op_der::<V, V>,
        reverse_der_ad    : no_op_der::<V, AD<V> >,
        rust_src          : no_op_rust_src,
        reverse_depend    : no_op_reverse_depend::<V>,
    };
}
