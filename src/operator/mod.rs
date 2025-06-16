// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! operations for specific operators
//
// AD
use crate::AD;
//
#[cfg(doc)]
use crate::ad_tape::Tape;
//
#[cfg(doc)]
use crate::ad_tape::THIS_THREAD_TAPE;
// ---------------------------------------------------------------------------
// impl_binary_operator
/// This macro implements the the following binary operations:
/// <pre>
///     AD    op AD
///     Float op AD
///     AD    op Float
/// </pre>
/// This include storing the operation in the [THIS_THREAD_TAPE] .
///
/// # Trait
/// is the std::ops trait for this operator; e.g., Add .
///
/// # op
/// is the token for this operator; e.g., + .
///
macro_rules! impl_binary_operator { ($Trait:ident, $op:tt) => {paste::paste! {
    //
    #[ doc = concat!(" record an ", stringify!($Trait), " operation ") ]
    fn [< record_ $Trait:lower >] (tape: &mut Tape, lhs: &AD, rhs: &AD) ->
    (Index, Index) {
        let mut new_tape_id   = 0;
        let mut new_var_index = 0;
        if tape.recording {
            let var_lhs    = lhs.tape_id == tape.tape_id;
            let var_rhs    = rhs.tape_id == tape.tape_id;
            if var_lhs || var_rhs {
                new_tape_id   = tape.tape_id;
                new_var_index = tape.n_var;
                tape.n_var   += 1;
                tape.op2arg.push( tape.arg_all.len() );
                if var_lhs && var_rhs {
                    tape.id_all.push( [< $Trait:upper _VV_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( rhs.var_index );
                } else if var_lhs {
                    tape.id_all.push( [< $Trait:upper _VC_OP >] );
                    tape.arg_all.push( lhs.var_index );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( rhs.value );
                } else {
                    tape.id_all.push( [< $Trait:upper _CV_OP >] );
                    tape.arg_all.push( tape.con_all.len() );
                    tape.con_all.push( lhs.value );
                    tape.arg_all.push( rhs.var_index );
                }
            }
        }
        (new_tape_id, new_var_index)
    }
    //
    impl std::ops::$Trait<AD> for AD {
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
    //
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
// ---------------------------------------------------------------------------
//
use crate::Float;
use crate::Index;
use id::NUMBER_OP;
//
// id
pub mod id;
//
#[cfg(test)]
use id::{
    ADD_CV_OP,
    ADD_VC_OP,
    ADD_VV_OP,
    MUL_CV_OP,
    MUL_VC_OP,
    MUL_VV_OP,
};
//
// operatorsd
pub mod add;
pub mod mul;
// ---------------------------------------------------------------------------
//
// ForwardZero
/// Evaluate zero order forward for one operation in the operation sequence.
pub type ForwardZero = fn(_var_zero: &mut Vec<Float>,
    _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ForwardOne
/// Evaluate first order forward for one operation in the operation sequence.
pub type ForwardOne = fn(_var_one: &mut Vec<Float>,
    _var_zero: &Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ReverseOne
/// Evaluate first order reverse for one operation in the operation sequence.
pub type ReverseOne = fn(_partial: &mut Vec<Float>,
    _var_zero: &Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ADForwardZero
/// Evaluate zero order forward for one operation in the operation sequence.
pub type ADForwardZero = fn(_var_zero: &mut Vec<AD>,
    _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ADForwardOne
/// Evaluate first order reverse for one operation in the operation sequence.
pub type ADForwardOne = fn(_var_one: &mut Vec<AD>,
    _var_zero: &Vec<AD>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ADReverseOne
/// Evaluate first order reverse for one operation in the operation sequence.
pub type ADReverseOne = fn(_var_one: &mut Vec<AD>,
    _var_zero: &Vec<AD>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
// ---------------------------------------------------------------------------
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
/// is the index in *var_zero* where the result for this operator is placed.
///
/// # var_zero
/// is the vector of the zero order values for all the variables.
/// If both left and right are variables:
/// <pre>
///     var_zero[res] = var_zero[lhs] op var_zero[rhs]
/// </pre>
/// If left is a variable and the right is a constant:
/// <pre>
///     var_zero[res] = var_zero[lhs] op con[rhs]
/// </pre>
/// If left is a constant and the right is a variable:
/// <pre>
///     var_zero[res] = con[lhs] op var_zero[rhs]
/// </pre>
#[cfg(doc)]
pub type ForwardZeroBinary = fn(_var_zero: &mut Vec<Float>,
    _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ForwardOneBinary
/// This is a [ForwardOne] with the following extra conditions:
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
/// is the index in *var_one* where the result for this operator is placed.
///
/// # var_zero
/// is the vector of the zero order values for all the variables.
/// If both left and right are variables:
/// <pre>
///     var_zero[res] = var_zero[lhs] op var_zero[rhs]
/// </pre>
/// If left is a variable and the right is a constant:
/// <pre>
///     var_zero[res] = var_zero[lhs] op con[rhs]
/// </pre>
/// If left is a constant and the right is a variable:
/// <pre>
///     var_zero[res] = con[lhs] op var_zero[rhs]
/// </pre>
///
/// # var_one
/// is the vector of directional derivatives.
/// The directional deriative var_one\[res\] is computed using the
/// its value of var_one\[i\] for indices *i* less tham *res* .
#[cfg(doc)]
pub type ForwardOneBinary = fn(_var_one: &mut Vec<Float>,
    _var_zero : &Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
//
// ReverseOneBinary
/// This is a [ReverseOne] with the following extra conditions:
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
/// is the index in *var_one* where the result for this operator is placed.
///
/// # var_zero
/// is the vector of the zero order values for all the variables.
/// If both left and right are variables:
/// <pre>
///     var_zero[res] = var_zero[lhs] op var_zero[rhs]
/// </pre>
/// If left is a variable and the right is a constant:
/// <pre>
///     var_zero[res] = var_zero[lhs] op con[rhs]
/// </pre>
/// If left is a constant and the right is a variable:
/// <pre>
///     var_zero[res] = con[lhs] op var_zero[rhs]
/// </pre>
///
/// # partial
/// Reverse mode computes the partial derivatives of a scalar function of the
/// range vector.
/// On input *partial* contains the derivative w.r.t. the variables
/// up to and including *res* .
/// Upon return, the variable with index *res* has been removed by
/// expressing it as a function of the variables with lower indices.
#[cfg(doc)]
pub type ReverseOneBinary = fn(_var_one: &mut Vec<Float>,
    _var_zero : &Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
// ---------------------------------------------------------------------------
//
// panic_zero
/// default [ForwardZero] function that will panic if it does not get replaced.
fn panic_zero( _var_zero: &mut Vec<Float>,
    _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// panic_one
/// default [ForwardOne] or [ReverseOne] function that will panic
/// if it does not get replaced.
fn panic_one( _var_one: &mut Vec<Float>,
    _var_zero : &Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// ad_panic_zero
/// default [ADForwardZero] function, will panic if it does not get replaced.
fn ad_panic_zero( _var_zero: &mut Vec<AD>,
    _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// ad_panic_one
/// default [ADForwardOne] or [ADReverseOne] function that will panic
/// if it does not get replaced.
fn ad_panic_one( _var_one: &mut Vec<AD>,
    _var_zero : &Vec<AD>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
//
// ---------------------------------------------------------------------------
//
// OpInfo
/// information connected to each operator id.
#[derive(Clone)]
pub struct OpInfo {
    pub name         : String,
    pub forward_0    : ForwardZero,
    pub forward_1    : ForwardOne,
    pub reverse_1    : ReverseOne,
    pub ad_forward_0 : ADForwardZero,
    pub ad_forward_1 : ADForwardOne,
    pub ad_reverse_1 : ADReverseOne,
}
//
// op_info_vec
/// set the value of OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{
        name: "".to_string(),
        forward_0    : panic_zero,
        forward_1    : panic_one,
        reverse_1    : panic_one,
        ad_forward_0 : ad_panic_zero,
        ad_forward_1 : ad_panic_one,
        ad_reverse_1 : ad_panic_one,
    };
    let mut result    = vec![empty ; NUMBER_OP ];
    add::set_op_info(&mut result);
    mul::set_op_info(&mut result);
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
    //
    assert_eq!( "mul_cv", op_info_vec[MUL_CV_OP].name );
    assert_eq!( "mul_vc", op_info_vec[MUL_VC_OP].name );
    assert_eq!( "mul_vv", op_info_vec[MUL_VV_OP].name );
}
