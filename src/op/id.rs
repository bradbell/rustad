// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! Define operator identifiers as `pub(crate) u8` constants
//!
//! Link to [parent module](super)
//
/// Sets all the opeerator identifier values
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

// Public u8 constants for each operator.
// See test at end mod.rs that check that every operator has a different name.
// This ensures that the number of operators is less that u8::MAX.
set_operator_ids!(
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
    /// lhs.num_lt(rhs)
    LT_OP,
    /// lhs.num_le(rhs)
    LE_OP,
    /// lhs.num_eq(rhs)
    EQ_OP,
    /// lhs.num_ne(rhs)
    NE_OP,
    /// lhs.num_ge(rhs)
    GE_OP,
    /// lhs.num_gt(rhs)
    GT_OP,
    /// lhs.not()
    NOT_OP,
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
