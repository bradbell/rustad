// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! operations for specific operators
//
use crate::Float;
use crate::Index;
use id::NUMBER_OP;
//
// id
pub mod id;
//
#[cfg(test)]
use id::{ADD_VC_OP, ADD_VV_OP};
//
// add
pub mod add;
//
// ForwardZeroFn
/// Type for fuunctions that evaluate zero order forward mode for one
/// operator in the operation sequence.
pub type ForwardZeroFn = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// panic_fn
fn panic_fn(
    _vec: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub forward_0 : ForwardZeroFn,
}
//
// OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), forward_0 : panic_fn };
    let mut result    = vec![empty ; NUMBER_OP ];
    add::set_op_info(&mut result);
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
