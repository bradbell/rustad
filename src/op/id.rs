// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Define operator identifiers as `pub(crate) u8` constants
//!
//! Link to [parent module](super)
//
// check_binary_op_id
/// For name equal ADD, SUB, MUL, DIV, check that:
/// ```text
///     name_PV_OP == name_PP_OP + 1
///     name_VP_OP == name_PP_OP + 2
///     name_VV_OP == name_PP_OP + 3
/// ```
#[allow(dead_code)]
pub(crate) fn doc_binary_op_id() { }
//
const _ : () = {
    // ADD
    assert!( ADD_PV_OP == ADD_PP_OP + 1 );
    assert!( ADD_VP_OP == ADD_PP_OP + 2 );
    assert!( ADD_VV_OP == ADD_PP_OP + 3 );
    // SUB
    assert!( SUB_PV_OP == SUB_PP_OP + 1 );
    assert!( SUB_VP_OP == SUB_PP_OP + 2 );
    assert!( SUB_VV_OP == SUB_PP_OP + 3 );
    // MUL
    assert!( MUL_PV_OP == MUL_PP_OP + 1 );
    assert!( MUL_VP_OP == MUL_PP_OP + 2 );
    assert!( MUL_VV_OP == MUL_PP_OP + 3 );
    // DIV
    assert!( DIV_PV_OP == DIV_PP_OP + 1 );
    assert!( DIV_VP_OP == DIV_PP_OP + 2 );
    assert!( DIV_VV_OP == DIV_PP_OP + 3 );
};
//
// set_operator_ids
/// Macro that sets all the opeerator identifier values
macro_rules! set_operator_ids {
    //
    // first match
    (   #[$doc:meta] $name:ident,
        $( #[$docs:meta] $names:ident, )*
    ) => {
        #[$doc]
        pub(crate) const $name : u8 = 0;
        set_operator_ids!(
            @ $name,
            $( #[$docs] $names, )*
        );
    };
    //
    // recursive match
    (
        @ $previous:ident,
        #[$doc:meta] $name:ident,
        $( #[$docs:meta] $names:ident, )*
    ) => {
        #[$doc]
        pub(crate) const $name : u8 = $previous + 1u8;
        set_operator_ids!(
            @ $name,
            $( #[$docs] $names, )*
        );
    };
    //
    // last recursive match
    (@ $index:expr,) => { }
}
//
// Public u8 constants for each operator.
// See test at end mod.rs that check that every operator has a different name.
// This ensures that the number of operators is less that u8::MAX.
set_operator_ids!(
    // Unary Operators
    /// ln_1p
    LN_1P_OP,
    /// exp_m1
    EXP_M1_OP,
    /// ln
    LN_OP,
    /// sqrt
    SQRT_OP,
    /// tanh
    TANH_OP,
    /// tan
    TAN_OP,
    /// sinh
    SINH_OP,
    /// cosh
    COSH_OP,
    /// abs
    ABS_OP,
    /// signum
    SIGNUM_OP,
    /// exp
    EXP_OP,
    /// minus
    MINUS_OP,
    /// cos
    COS_OP,
    /// sine
    SIN_OP,
    //
    // ADD
    /// parameter + parameter
    ADD_PP_OP,
    /// parameter + variable
    ADD_PV_OP,
    /// variable + parameter
    ADD_VP_OP,
    /// variable + variable
    ADD_VV_OP,
    //
    // SUB
    /// parameter - parameter
    SUB_PP_OP,
    /// parameter - variable
    SUB_PV_OP,
    /// variable - parameter
    SUB_VP_OP,
    /// variable - variable
    SUB_VV_OP,
    //
    // MUL
    /// parameter * parameter
    MUL_PP_OP,
    /// parameter * variable
    MUL_PV_OP,
    /// variable * parameter
    MUL_VP_OP,
    /// variable * variable
    MUL_VV_OP,
    //
    // DIV
    /// parameter / parameter
    DIV_PP_OP,
    /// parameter / variable
    DIV_PV_OP,
    /// variable / parameter
    DIV_VP_OP,
    /// variable / variable
    DIV_VV_OP,
    //
    /// powi(lhs, rhs)
    POWI_OP,
    //
    /// powf(lhs, rhs)
    POWF_OP,
    //
    /// lhs lt rhs
    LT_OP,
    /// lhs le rhs
    LE_OP,
    /// lhs eq rhs
    EQ_OP,
    /// lhs ne rhs
    NE_OP,
    /// lhs ge rhs
    GE_OP,
    /// lhs gt rhs
    GT_OP,
    //
    // CALL
    /// callback to an atomic function
    CALL_OP,
    /// place holder for results of a call operator
    CALL_RES_OP,
    //
    // NO_OP
    /// callback to a no-op operations
    NO_OP,
    //
    /// number of valid operator ids
    NUMBER_OP,
);
