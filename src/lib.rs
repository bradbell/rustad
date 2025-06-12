// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
//
//! The rustad Automatic Differentiation Package
//
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.6.11");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.11" );
// ----------------------------------------------------------------------------
//
// utility
pub mod utility;
//
// operator_id
// ADD_VC_OP, ADD_VV_OP, ... , NUMBER_OP
pub(crate) mod operator_id;
use operator_id::*;
//
// Index
/// Type used for indexing vectors in the tape.
/// It must be able to represent the total number of
/// operators, constants, and arguments to operators.
pub type Index = usize;
//
// Float
/// Floating point Type used for AD operations.
pub type Float = f64;
//
// ForwardZeroFn
/// Type for fuunctions that evaluate zero order forward mode for one
/// operator in the operation sequence.
pub type ForwardZeroFn = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// AD
pub mod ad;
use ad::AD;
//
// ad_tape
pub(crate) mod ad_tape;
//
// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : ForwardZeroFn,
}
//
// panic_eval_fn
fn panic_eval_fn(
    _vec: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// function
pub mod function;
//
// add_op
mod add_op;
//
// OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), fun : panic_eval_fn };
    let mut result    = vec![empty ; NUMBER_OP ];
    result[ADD_VC_OP] =
        OpInfo{ name : "add_vc".to_string() , fun : add_op::eval_add_vc_fn };
    result[ADD_VV_OP] =
        OpInfo{ name : "add_vv".to_string() , fun : add_op::eval_add_vv_fn };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );

#[test]
fn test_op_info() {
    let op_info_vec = &*OP_INFO_VEC;
    assert_eq!( "add_vc", op_info_vec[ADD_VC_OP].name );
    assert_eq!( "add_vv", op_info_vec[ADD_VV_OP].name );
}
