// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! operations for specific operators
//
//
/// This macro implements the Float op AD and AD op Float cases
/// by folding them into the AD + AD case.
///
/// # trait
/// is the std::ops trait for this operator; e.g., Add .
///
/// # op
/// is the token for this operator; e.g., + .
///
macro_rules! fold_binary_operator {
    ( $trait:ident , $op:tt ) => {
        //
        paste::paste! {
            impl std::ops::$trait<AD> for Float {
                type Output = AD;
                //
                #[ doc = concat!(" compute Float ", stringify!($op), " AD") ]
                fn [< $trait:lower >] (self, rhs : AD) -> AD {
                    AD::from(self) $op rhs
                }
            }
            //
            impl std::ops::$trait<Float> for AD {
                type Output = AD;
                //
                #[ doc = concat!(" compute AD ", stringify!($op), " Float") ]
                fn [< $trait:lower >] (self, rhs : Float) -> AD {
                    self $op AD::from(rhs)
                }
            }
        }
    }
}
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
// ForwardZero
/// Evaluate zero order forward mode for operation in the operation sequence.
pub type ForwardZero = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ForwardZeroBinary
/// This is a [ForwardZero] with the following extra conditions:
///
/// # op
/// we use the notation *op* for this operator's symbol; e.g. + for addition.
///
/// # arg
/// is a slice of size two.  We use the notation
/// <pre>
///     lhs = arg[0]
///     rhs = arg[1]
/// </pre>
///
/// # res
/// is the index in *var* where the result for this operator is placed.
///
/// # var
/// is the vector of the zero order values for all the variables.
/// If both left and right are variables:
/// <pre>
///     var[res] = var[lhs] op var[rhs]
/// </pre>
/// If left is a variable and the right is a constant:
/// <pre>
///     var[res] = var[lhs] op con[rhs]
/// </pre>
/// If left is a constant and the right is a variable:
/// <pre>
///     var[res] = con[lhs] op left[rhs]
/// </pre>
#[cfg(doc)]
pub type ForwardZeroBinary = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// panic_fn
/// default [ForwardZero] function that will panic if it does not get replaced.
fn panic_fn(
    _vec: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// OpInfo
/// information connected to each operator id.
#[derive(Clone)]
pub struct OpInfo {
    pub name      : String,
    pub forward_0 : ForwardZero,
}
//
// op_info_vec
/// set the value of OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), forward_0 : panic_fn };
    let mut result    = vec![empty ; NUMBER_OP ];
    add::set_op_info(&mut result);
    result
}
//
// OP_INFO_VEC
/// mapping from each operator [id] to it's [OpInfo]
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );

#[test]
fn test_op_info() {
    let op_info_vec = &*OP_INFO_VEC;
    assert_eq!( "add_vc", op_info_vec[ADD_VC_OP].name );
    assert_eq!( "add_vv", op_info_vec[ADD_VV_OP].name );
}
