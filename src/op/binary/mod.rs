// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2026 Bradley M. Bell
//
//! This module defines the routines that evaluate binary operators.
//!
//! Link to [parent module](super)
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
pub mod hypot;
pub mod powf;
