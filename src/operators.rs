// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell


macro_rules! set_operator_indices {
    //
    // first match
    ($($name:ident),*,) => {
        set_operator_indices!(@ 0usize, $($name,)*);
    };
    //
    // recursive match
    (@ $index:expr, $name:ident, $($tail:ident,)*) => {
        pub const $name : usize = $index;
        set_operator_indices!(@ $index + 1usize, $($tail,)*);
    };
    //
    // last recursive match
    (@ $index:expr,) => {
        pub const NUMBER_OP : usize = $index;
    };
}

// Public usize constants for each operator.
set_operator_indices!(
    ADD_VV_OP,
    ADD_VC_OP,
);
