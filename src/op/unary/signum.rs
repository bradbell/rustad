// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the signum operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// z   = signum(x)
// z_x = 0
// --------------------------------------------------------------------------
// use
//
use std::ops::{
    Mul,
    AddAssign,
};
//
use crate::{
    IndexT,
    AD,
    FConst,
    FUnary,
};
//
use crate::ad::ADType;
use crate::op::unary::common;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::OpFns;
use crate::op::info::ConstData;
use crate::op::id::SIGNUM_OP;
// -------------------------------------------------------------------------
// signum_forward_dyp
common::forward_dyp!(signum);
//
// sim_forward_var
common::forward_var!(signum);
//
// signum_rust_src
common::rust_src!(signum);
//
// signum_forward_der
/// First order forward mode for signum(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn signum_forward_der<V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    var_der    :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    arg        :   &[IndexT]   ,
    arg_type   :   &[ADType]   ,
    res        :   usize       )
where
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    var_der[res] = FConst::zero();
}
// signum_reverse_der
/// First order reverse mode for signum(variable);
/// see [ForwardDer](crate::op::info::ForwardDer)
fn signum_reverse_der<V, E>(
    _dyp_all   :   &[E]        ,
    _var_all   :   &[E]        ,
    _var_der   :   &mut [E]    ,
    _cop       :   &[V]        ,
    _bool_all  :   &[bool]     ,
    _arg       :   &[IndexT]   ,
    _arg_type  :   &[ADType]   ,
    _res       :   usize       )
where
    for<'a> E     : AddAssign<&'a E> ,
    E             : FConst ,
    for<'a> &'a E : FUnary<Output=E>,
    for<'a> &'a E : Mul<&'a E, Output=E>,
{
    /*
    debug_assert!( arg.len() == 1 );
    debug_assert!( arg_type[0].is_variable() );
    let index       = arg[0] as usize;
    var_der[index] += &FConst::zero();
    */
}
// ---------------------------------------------------------------------------
// set_op_fns
/// Set the operator functions for all the SIGNUM_OP operator.
///
/// * op_fns_vec :
///   The map from [op::id](crate::op::id) to operator functions.
///   The the map results for SIGNUM_OP are set.
pub fn set_op_fns<V>( op_fns_vec : &mut [OpFns<V>] ) where
    for<'a> &'a AD<V> : Mul<&'a AD<V>, Output = AD<V> > ,
    for<'a> &'a V     : Mul<&'a V, Output = V> ,
    //
    for<'a> V         : AddAssign<&'a V>,
    for<'a> AD<V>     : AddAssign<&'a AD<V> >,
    //
    V                 : Clone + FConst + ThisThreadTape ,
    for<'a> &'a V     : FUnary<Output=V>,
{
    op_fns_vec[SIGNUM_OP as usize] = OpFns{
        name              : "signum",
        forward_dyp_value : signum_forward_dyp::<V, V>,
        forward_dyp_ad    : signum_forward_dyp::<V, AD<V> >,
        forward_var_value : signum_forward_var::<V, V>,
        forward_var_ad    : signum_forward_var::<V, AD<V> >,
        forward_der_value : signum_forward_der::<V, V>,
        forward_der_ad    : signum_forward_der::<V, AD<V> >,
        reverse_der_value : signum_reverse_der::<V, V>,
        reverse_der_ad    : signum_reverse_der::<V, AD<V> >,
        rust_src          : signum_rust_src,
        reverse_depend    : common::reverse_depend,
    };
}
