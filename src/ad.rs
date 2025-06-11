// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
//! AD an automatic differentiation floating point type
//
use crate::Index;
use crate::Float;
//
/// AD acts like the Float type, can record functions and store
/// it in an [ADFun](crate::function::ADFun) object.
///
/// # variable
/// An AD object is a variable if it one of the
/// [domain](crate::function::domain)
/// variables or its value depends on the value of a domain variable.
///
/// # constant
/// If an AD object is not a variable it is referred to as a constant.
#[derive(Copy, Clone)]
pub struct AD {
    //
    // tape_id
    ///
    /// An AD object is a variable if the following two conditions hold:
    /// 1. [THIS_THREAD_TAPE](crate::THIS_THREAD_TAPE)
    ///    is currently recording.
    /// 2. This threads tape and the AD object have the same *tape_id* .
    pub(crate) tape_id   : Index,
    //
    // var_index
    /// If this AD object is a variable, var_index is its index in the tape.
    pub(crate) var_index : Index,
    //
    // value
    /// This is the value of this AD variable or constant.
    pub(crate) value     : Float,
}
impl From<Float> for AD {
    fn from(this_value : Float) -> Self {
        Self {
            tape_id   : 0,
            var_index : 0,
            value     : this_value,
        }
    }
}
