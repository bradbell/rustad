// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025-26 Bradley M. Bell
//
//! This pub(crate) module defines the objects used to evaluate an
//! operation sequence.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// sub-modules
//
pub mod id;
pub mod info;
pub mod unary;
pub mod binary;
//
pub mod call;
pub mod no_op;
pub mod powi;
