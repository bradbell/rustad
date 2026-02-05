// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Evaluate the sin operator
//!
//! Link to [parent module](super)
// --------------------------------------------------------------------------
// use
//
use crate::ad::ADType;
use crate::{
    IndexT,
    AD,
    FloatCore,
};
//
use crate::op::unary;
use crate::tape::sealed::ThisThreadTape;
use crate::op::info::{
    OpInfo,
    panic_der,
};
use crate::op::id::SIN_OP;
// -------------------------------------------------------------------------
unary::forward_dyp!(sin);
unary::forward_var!(sin);
unary::rust_src!(sin);
// ---------------------------------------------------------------------------
// set_op_info
/// Set the operator information for all the SIN_OP operator.
///
/// * op_info_vec :
///   The map from [op::id](crate::op::id) to operator information.
///   The the map results for SIN_OP are set.
pub fn set_op_info<V>( op_info_vec : &mut [OpInfo<V>] )
where
    V : Clone + FloatCore + ThisThreadTape ,
{
    op_info_vec[SIN_OP as usize] = OpInfo{
        name              : "sin",
        forward_dyp_value : sin_forward_dyp::<V, V>,
        forward_dyp_ad    : sin_forward_dyp::<V, AD<V> >,
        forward_var_value : sin_forward_var::<V, V>,
        forward_var_ad    : sin_forward_var::<V, AD<V> >,
        forward_der_value : panic_der::<V, V>,
        forward_der_ad    : panic_der::<V, AD<V> >,
        reverse_der_value : panic_der::<V, V>,
        reverse_der_ad    : panic_der::<V, AD<V> >,
        rust_src          : sin_rust_src,
        reverse_depend    : unary::reverse_depend,
    };
}
