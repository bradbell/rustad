// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! This module defines the objects that evaluate unary operators.
//!
//! Link to [parent module](super)
//!
//! # Operators :
//! [comment]: <> (BEGIN_SORT_THIS_LINE_PLUS_1)
//! ABS_OP,
//! COSH_OP,
//! COS_OP,
//! EXP_OP,
//! MINUS_OP,
//! SIGNUM_OP,
//! SINH_OP,
//! SIN_OP,
//! SQRT_OP,
//! TANH_OP,
//! TAN_OP,
//! [comment]: <> (END_SORT_THIS_LINE_MINUS_1)
//!
//! # Operator Arguments
//! | Index | Meaning |
//! | ----- | ------- |
//! | 0     | Variable, dynamic, or constant index for operator argument |
// ---------------------------------------------------------------------------
// sub-modules
// BEGIN_SORT_THIS_LINE_PLUS_1
pub mod abs;
pub mod common;
pub mod cos;
pub mod cosh;
pub mod exp;
pub mod ln;
pub mod minus;
pub mod signum;
pub mod sin;
pub mod sinh;
pub mod sqrt;
pub mod tan;
pub mod tanh;
// END_SORT_THIS_LINE_MINUS_1
