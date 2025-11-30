// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! Implement the [ADfn] optimization methods.
//!
//! Link to [parent module](super)
// ---------------------------------------------------------------------------
// use
//
#[cfg(doc)]
use crate::ADfn;
//
// -----------------------------------------------------------------------
mod reverse_depend;
mod dead_code;
// -----------------------------------------------------------------------
// Depend
/// Which constants, dynamic parameters, and variables the
/// range for an [ADfn] depends on.
///
/// TODO: change to private when reverse_depend gets changes to private.
pub struct Depend {
    // cop
    /// Constant parameters dependency; length [ADfn::cop_len].
    pub(crate) cop : Vec<bool> ,
    //
    // dyp
    /// Dynamic parameters dependency; length [ADfn::dyp_len].
    pub(crate) dyp : Vec<bool> ,
    //
    // var
    /// Variable dependency; length [ADfn::var_len].
    pub(crate) var : Vec<bool> ,
}
