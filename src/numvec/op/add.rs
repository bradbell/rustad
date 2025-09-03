// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the add operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::numvec::ad::doc_generic_v)
//! * E : see [doc_generic_e](crate::numvec::adfn::doc_generic_e)
//!
//! * [op::id](crate::numvec::op::id)
//!     * ADD_CV_OP : constant + variable
//!     * ADD_VC_OP : variable + constant
//!     * ADD_VV_OP : variable + variable
//!
//! * arg
//!     * arg\[0\]:  Variable or constant index of left operand.
//!     * arg\[1\]:  Variable or constant index of right operand.
// --------------------------------------------------------------------------
// use
//
use crate::numvec::tape::sealed::ThisThreadTape;
use crate::numvec::ad::AD;
use crate::numvec::op::info::OpInfo;
use crate::numvec::op::id::{
    ADD_VV_OP,
};
//
// --------------------------------------------------------------------------
// forward_0_add_vv_value
/// V Evaluation zero order forward: variable + variable
fn forward_0_add_vv_value <V>(
    var_zero  : &mut Vec<V> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    arg       : &[usize]    ,
    res       : usize       )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &var_zero[lhs] + &var_zero[rhs];
}
// forward_0_add_vv_ad
/// ``AD`` < *V* > Evaluation zero order forward: variable + variable
fn forward_0_add_vv_ad <V>(
    var_zero  : &mut Vec< AD<V> > ,
    _con      : &Vec<V>           ,
    _flag     : &Vec<bool>        ,
    arg       : &[usize]          ,
    res       : usize             )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &var_zero[lhs] + &var_zero[rhs];
}
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the add operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::numvec::op::id) to operator information.
/// The the map results for ADD_CV_OP, ADD_VC_OP, and ADD_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name              : "add_vv",
        forward_0_value   : forward_0_add_vv_value::<V>,
        forward_0_ad      : forward_0_add_vv_ad::<V>,
    };
}
