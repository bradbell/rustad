// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Define operator identifiers as `pub(crate) usize` constants:
//! [parent module](super)
//
/// Sets all the opeerator identifier values
macro_rules! set_operator_ids {
    //
    // first match
    (   #[$doc:meta] $name:ident,
        $( #[$docs:meta] $names:ident, )*
    ) => {
        #[$doc]
        pub(crate) const $name : usize = 0;
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
        pub(crate) const $name : usize = $previous + 1usize;
        set_operator_ids!(
            @ $name,
            $( #[$docs] $names, )*
        );
    };
    //
    // last recursive match
    (@ $index:expr,) => { }
}

// Public usize constants for each operator.
set_operator_ids!(
    /// constant + variable
    ADD_CV_OP,
    /// variable + constant
    ADD_VC_OP,
    /// variable + variable
    ADD_VV_OP,
    /// Call a chckpoint function
    CALL_OP,
    /// Place holder for results of call operator
    CALL_RES_OP,
    /// constant * variable
    MUL_CV_OP,
    /// variable * constant
    MUL_VC_OP,
    /// variable * variable
    MUL_VV_OP,
    /// number of operator identifiers
    NUMBER_OP,
);
