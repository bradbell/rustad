// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
//! This module defines the objects that evaluate binary operators.
//!
//! Link to [parent module](super)
//!
//! # Arithmetic Operators :
//! ADD_XX_OP, SUB_XX_OP, MUL_XX_OP, DIV_XX_OP
//! where XX is PP, PV, or VP.
//!
//! # Compare Operators :
//! LT_OP, LE_OP, EQ_OP, NE_OP, GE_OP, GT_OP
//!
//! # Operator Arguments
//! | Index | Meaning |
//! | ----- | ------- |
//! | 0     | Variable, dynamic, or constant index for left hand side  |
//! | 1     | Variable, dynamic, or constant index for right hand side |
// ---------------------------------------------------------------------------
// sub-modules
pub mod common;
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod num_cmp;
pub mod powf;
