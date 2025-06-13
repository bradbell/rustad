// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! operations for specific operators
//
#[cfg(doc)]
use crate::ad_tape::Tape;
//
#[cfg(doc)]
use crate::ad::AD;
//
//
/// This macro implements the the following operations:
/// <pre>
///     AD    op AD
///     Float op AD
///     AD    op Float
/// </pre>
/// # Trait
/// is the std::ops trait for this operator; e.g., Add .
///
/// # op
/// is the token for this operator; e.g., + .
///
/// # record_*trait*
/// The function record_*trait* must be defined locally where
/// *trait* is a lower case version of Trait and it supports:
/// <pre>
///     (new_tape_id, new_var_id) = record_trait(tape, &lhs, &rhs)
/// </pre>
///
/// ## tape
/// is the [Tape] where this operation will be placed or found.
///
/// ## lhs
/// is the left side [AD] operand for this operation.
///
/// ## rhs
/// is the right side [AD] operand for this operation.
///
/// ## new_tape_id
/// is the tape_id for the *tape* . If it is zero,
/// the result is a constant and was not recorded.
///
/// ## new_var_id
/// is the variableindex for the result of this operatrion.
///
macro_rules! impl_binary_operator { ($Trait:ident, $op:tt) => {paste::paste! {
    impl std::ops::Add<AD> for AD {
        type Output = AD;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " AD") ]
        fn [< $Trait:lower >] (self, rhs : AD) -> AD {
            let new_value = self.value $op rhs.value;
            let ( new_tape_id, new_var_index) =
            THIS_THREAD_TAPE.with_borrow_mut(
                |tape| [< record_ $Trait:lower >] (tape, &self, &rhs)
            );
            AD {
                tape_id   : new_tape_id,
                var_index : new_var_index,
                value     : new_value,
            }
        }
    }
    impl std::ops::$Trait<AD> for Float {
        type Output = AD;
        //
        #[ doc = concat!(" compute Float ", stringify!($op), " AD") ]
        fn [< $Trait:lower >] (self, rhs : AD) -> AD {
            AD::from(self) $op rhs
        }
    }
    //
    impl std::ops::$Trait<Float> for AD {
        type Output = AD;
        //
        #[ doc = concat!(" compute AD ", stringify!($op), " Float") ]
        fn [< $Trait:lower >] (self, rhs : Float) -> AD {
            self $op AD::from(rhs)
        }
    }
} } }
//
use crate::Float;
use crate::Index;
use id::NUMBER_OP;
//
// id
pub mod id;
//
#[cfg(test)]
use id::{ ADD_CV_OP, ADD_VC_OP, ADD_VV_OP };
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
    assert_eq!( "add_cv", op_info_vec[ADD_CV_OP].name );
    assert_eq!( "add_vc", op_info_vec[ADD_VC_OP].name );
    assert_eq!( "add_vv", op_info_vec[ADD_VV_OP].name );
}
