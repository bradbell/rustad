// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell

//! This module defines the operator indices as public usize constants


// set_operator_indices
macro_rules! set_operator_indices {
    //
    // first match
    ( $( #[$doc:meta] $name:ident),*,) => {
        set_operator_indices!(
            @ 0usize,
            $( #[$doc] $name, )*
        );
    };
    //
    // recursive match
    (
        @ $index:expr,
        #[$doc:meta] $name:ident,
        $( #[$docs:meta] $names:ident, )*
    ) => {
        #[$doc]
        pub const $name : usize = $index;
        set_operator_indices!(
            @ $index + 1usize,
            $( #[$docs] $names, )*
        );
    };
    //
    // last recursive match
    (@ $index:expr,) => { }
}

// Public usize constants for each operator.
set_operator_indices!(
    /// variable + variable
    ADD_VV_OP,
    /// variable + constant
    ADD_VC_OP,
    /// number of operators
    NUMBER_OP,
);
