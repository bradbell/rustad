// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
use crate::Index;
use crate::Float;
//
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
