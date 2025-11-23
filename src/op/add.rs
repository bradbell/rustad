// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Evaluate the Add operators
//!
//! Link to [parent module](super)
//!
//! * V : see [doc_generic_v](crate::doc_generic_v)
//! * E : see [doc_generic_e](crate::adfn::doc_generic_e)
//!
//! * [op::id](crate::op::id)
//!     * ADD_PV_OP : parameter + variable
//!     * ADD_VP_OP : variable + parameter
//!     * ADD_VV_OP : variable + variable
//!
//! * arg
//!     * arg\[0\]:  Variable or parameter index of left operand.
//!     * arg\[1\]:  Variable or parameter index of right operand.
// --------------------------------------------------------------------------
// use
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
};
//
use crate::op::binary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_dyp,
    panic_var,
    panic_der,
    no_rust_src,
};
use crate::op::id::{
    ADD_PP_OP,
    ADD_PV_OP,
    ADD_VP_OP,
    ADD_VV_OP,
};
#[cfg(doc)]
use crate::op::info::{
    ForwardDer,
    ReverseDer,
};
// -------------------------------------------------------------------------
// add_pv_rust_src
// add_vp_rust_src
// add_vv_rust_src
binary::binary_rust_src!(Add, +);
// -------------------------------------------------------------------------
// add_forward_dyp
// add_pv_forward_0
// add_vp_forward_0
// add_vv_forward_0
binary::eval_binary_forward_0!(Add, +);
// ---------------------------------------------------------------------------
// forward_1
// ---------------------------------------------------------------------------
//
// add_pv_forward_1
/// first order forward for parameter + variable; see [ForwardDer]
fn add_pv_forward_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    E             : Clone,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    var_der[ res ] = var_der[rhs].clone();
}
//
// add_vp_forward_1
/// first order forward for variable + parameter; see [ForwardDer]
fn add_vp_forward_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    E             : Clone,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    var_der[ res ] = var_der[lhs].clone();
}
//
// add_vv_forward_1
/// first order forward for variable + variable; see [ForwardDer]
fn add_vv_forward_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    var_der[res] = &var_der[lhs]  + &var_der[rhs];
}
// ---------------------------------------------------------------------------
// reverse_1
// ---------------------------------------------------------------------------
//
// add_pv_reverse_1
/// first order reverse for parameter + variable; see [ReverseDer]
fn add_pv_reverse_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let rhs = arg[1] as usize;
    //
    // var_der[rhs] += &var_der[res];
    let sum      = &var_der[rhs] + &var_der[res];
    var_der[rhs] = sum;
}
//
// add_vp_reverse_1
/// first order reverse for variable + parameter; see [ReverseDer]
fn add_vp_reverse_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    //
    // var_der[lhs] += &var_der[res];
    let sum      = &var_der[lhs] + &var_der[res];
    var_der[lhs] = sum;
}
//
// add_vv_reverse_1
/// first order reverse for variable + variable; see [ReverseDer]
fn add_vv_reverse_1 <V, E>(
    _dyp_both  :   &Vec<E>     ,
    _var_both  :   &Vec<E>     ,
    var_der    :   &mut Vec<E> ,
    _cop       :   &Vec<V>     ,
    _flag_all  :   &Vec<ADType>,
    arg        :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    res        :   usize       )
where
    for<'a> &'a E : std::ops::Add<&'a E, Output = E> ,
{
    debug_assert!( arg.len() == 2);
    let lhs = arg[0] as usize;
    let rhs = arg[1] as usize;
    //
    // var_der[lhs] += &var_der[res];
    // var_der[rhs] += &var_der[res];
    let sum      = &var_der[rhs] + &var_der[res];
    var_der[rhs] = sum;
    let sum      = &var_der[lhs] + &var_der[res];
    var_der[lhs] = sum;
}
// ---------------------------------------------------------------------------
// set_op_info
//
no_rust_src!(Add);
//
/// Set the operator information for all the Add operators.
///
/// * op_info_vec :
/// The map from [op::id](crate::op::id) to operator information.
/// The the map results for ADD_PV_OP, ADD_VP_OP, and ADD_VV_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut Vec< OpInfo<V> > )
where
    for<'a> &'a V : std::ops::Add<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V : std::ops::Add<&'a V, Output = V> ,
    for<'a> V     : Clone + From<f32> + PartialEq +
                    ThisThreadTape + std::ops::AddAssign<&'a V>,
{
    op_info_vec[ADD_PP_OP as usize] = OpInfo{
        name              : "add_pp",
        forward_dyp_value : add_forward_dyp::<V, V>,
        forward_dyp_ad    : add_forward_dyp::<V, AD<V> >,
        forward_var_value : panic_var::<V, V>,
        forward_var_ad    : panic_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : rust_src_none,
    };
    op_info_vec[ADD_PV_OP as usize] = OpInfo{
        name              : "add_pv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : add_pv_forward_0::<V, V>,
        forward_var_ad    : add_pv_forward_0::<V, AD<V> >,
        forward_der_value : add_pv_forward_1::<V, V>,
        forward_der_ad    : add_pv_forward_1::<V, AD<V> >,
        reverse_der_value : add_pv_reverse_1::<V, V>,
        reverse_der_ad    : add_pv_reverse_1::<V, AD<V> >,
        rust_src          : add_pv_rust_src,
    };
    op_info_vec[ADD_VP_OP as usize] = OpInfo{
        name              : "add_vp",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : add_vp_forward_0::<V, V>,
        forward_var_ad    : add_vp_forward_0::<V, AD<V> >,
        forward_der_value : add_vp_forward_1::<V, V>,
        forward_der_ad    : add_vp_forward_1::<V, AD<V> >,
        reverse_der_value : add_vp_reverse_1::<V, V>,
        reverse_der_ad    : add_vp_reverse_1::<V, AD<V> >,
        rust_src          : add_vp_rust_src,
    };
    op_info_vec[ADD_VV_OP as usize] = OpInfo{
        name              : "add_vv",
        forward_dyp_value : panic_dyp::<V, V>,
        forward_dyp_ad    : panic_dyp::<V, AD<V> >,
        forward_var_value : add_vv_forward_0::<V, V>,
        forward_var_ad    : add_vv_forward_0::<V, AD<V> >,
        forward_der_value : add_vv_forward_1::<V, V>,
        forward_der_ad    : add_vv_forward_1::<V, AD<V> >,
        reverse_der_value : add_vv_reverse_1::<V, V>,
        reverse_der_ad    : add_vv_reverse_1::<V, AD<V> >,
        rust_src          : add_vv_rust_src,
    };
}
