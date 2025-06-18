// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
//
//! Operator that calls an ADFun (Under Construction):  [parent module](super)
//!
//! # Operator Id
//!  CALL_OP
//!
//! # Operator Arguments:
//! | Index   | Meaning |
//! | ------- | ------- |
//! | 0       | Index that identifies the ADFun object being called |
//! | 1       | Index of the first boolean for this operator |
//! | 2       | Number of arguments to the function being called (narg) |
//! | 3       | Number of results for the function being called (nres) |
//! | 4       | Variable or constant index for first argument to call |
//! | 5       | Variable or constant index for second argument to call |
//! | ...     | ... |
//! | 3+narg  | Variable or constant index for last argument to call |
//!
//! # Operator Booleans:
//! | Index   | Meaning |
//! | ------- | ------- |
//! | 0       | true (false) if first call argument is a variable (constant) |
//! | 1       | true (false) if second call argument is a variable (constant) |
//! | ...     | ... |
//! | narg-1  | true (false) if last call argument is a variable (constant) |
//!
//
/*
use crate::{Index, Float};
p
fn float_forward_0_call(
    var_zero: &mut Vec<Float>,
    con:      &Vec<Float>,
    arg:      &[Index],
    flag:     &Vec<bool>,
    res:      Index)
{
}
*/
