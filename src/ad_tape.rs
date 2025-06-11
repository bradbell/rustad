// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ---------------------------------------------------------------------------
//
use crate::Index;
use crate::Float;
//
// TapeInfo
pub struct TapeInfo {
    pub tape_id        : Index,
    pub recording      : bool,
    pub n_domain       : Index,
    pub n_var          : Index,
    pub op_all         : Vec<Index>,
    pub op2arg         : Vec<Index>,
    pub arg_all        : Vec<Index>,
    pub con_all        : Vec<Float>,
}
impl TapeInfo {
    pub fn new() -> Self {
        Self {
            tape_id       : 0,
            recording     : false,
            n_domain      : 0,
            n_var         : 0,
            op_all        : Vec::new() ,
            op2arg        : Vec::new() ,
            arg_all       : Vec::new() ,
            con_all       : Vec::new() ,
        }
    }
}
//
// THIS_THREAD_TAPE
thread_local! {
    /// Thread local storage used to record functions
    pub(crate) static THIS_THREAD_TAPE: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
