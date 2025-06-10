// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
use crate::Index;
use crate::Float;
//
/// The AD type acts like the Float type.
/// It has the addition capability to recoerd functions and
/// store it in an [ADFun](crate::ADFun) object.
///
/// # variable
/// An AD object is a variable if the following two conditions hold:
/// 1. [This threads recorder](crate::THIS_THREAD_RECORDER)
///    is currently recording
/// 2. The threads recorder and the AD object have the same *tape_id*
///
/// # constant
/// If an AD object is not a variable it is referred to as a constant.
#[derive(Copy, Clone)]
pub struct AD {
    pub tape_id   : Index,
    pub var_index : Index,
    pub value     : Float,
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
