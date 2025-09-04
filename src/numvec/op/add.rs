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
    ADD_VC_OP,
    ADD_CV_OP,
};
// --------------------------------------------------------------------------
// add_vv
// --------------------------------------------------------------------------
// add_vv_forward_0_value
/// E Evaluation of zero order forward for variable + variable
fn add_vv_forward_0 <V, E>(
    var_zero  : &mut Vec<E> ,
    _con      : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    arg       : &[usize]    ,
    res       : usize       )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &var_zero[lhs] + &var_zero[rhs];
}
// --------------------------------------------------------------------------
// add_vc
// --------------------------------------------------------------------------
// add_vc_forward_0_value
/// V Evaluation zero order forward: variable + constant
fn add_vc_forward_0_value <V>(
    var_zero  : &mut Vec<V> ,
    con       : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    arg       : &[usize]    ,
    res       : usize       )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &var_zero[lhs] + &con[rhs];
}
// add_vc_forward_0_ad
/// ``AD`` < *V* > Evaluation zero order forward: variable + constant
fn add_vc_forward_0_ad <V>(
    var_zero  : &mut Vec< AD<V> > ,
    con       : &Vec<V>           ,
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
    var_zero[ res ] = &var_zero[lhs] + &con[rhs];
}
// --------------------------------------------------------------------------
// add_cv
// --------------------------------------------------------------------------
// add_cv_forward_0_value
/// V Evaluation zero order forward: constant + variable
fn add_cv_forward_0_value <V>(
    var_zero  : &mut Vec<V> ,
    con       : &Vec<V>     ,
    _flag     : &Vec<bool>  ,
    arg       : &[usize]    ,
    res       : usize       )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &con[lhs] + &var_zero[rhs];
}
// add_cv_forward_0_ad
/// ``AD`` < *V* > Evaluation zero order forward: constant + variable
fn add_cv_forward_0_ad <V>(
    var_zero  : &mut Vec< AD<V> > ,
    con       : &Vec<V>           ,
    _flag     : &Vec<bool>        ,
    arg       : &[usize]          ,
    res       : usize             )
where
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    V             : Clone + ThisThreadTape ,
{
    debug_assert!( arg.len() == 2);
    let lhs  = arg[0] as usize;
    let rhs  = arg[1] as usize;
    var_zero[ res ] = &con[lhs] + &var_zero[rhs];
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
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    V             : Clone + ThisThreadTape ,
{
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name              : "add_vv",
        forward_0_value   : add_vv_forward_0::<V, V>,
        forward_0_ad      : add_vv_forward_0::<V, AD<V> >,
    };
    op_info_vec[ADD_VC_OP as usize] = OpInfo{
        name              : "add_vc",
        forward_0_value   : add_vc_forward_0_value::<V>,
        forward_0_ad      : add_vc_forward_0_ad::<V>,
    };
    op_info_vec[ADD_CV_OP as usize] = OpInfo{
        name              : "add_cv",
        forward_0_value   : add_cv_forward_0_value::<V>,
        forward_0_ad      : add_cv_forward_0_ad::<V>,
    };
}
