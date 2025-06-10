// SPDX-License-Identifier: EPL-2.0 OR GPL-2.0-or-later
// SPDX-FileCopyrightText: Bradley M. Bell <bradbell@seanet.com>
// SPDX-FileContributor: 2025 Bradley M. Bell
// ----------------------------------------------------------------------------
// YEAR_MONTH_DAY
/// is the date corresponding to this version of the software as
/// *year*.*month*.*day* .
///
/// # Example
/// ```
/// let date = *rustad::YEAR_MONTH_DAY;
/// assert_eq!(date, "2025.6.10");
/// ```
pub const YEAR_MONTH_DAY: std::sync::LazyLock<&str> =
   std::sync::LazyLock::new( || "2025.6.10" );
// ----------------------------------------------------------------------------
//
// utility
pub mod utility;
//
// ADD_VV_OP, ADD_VC_OP, ...
// define all the operator indices
pub(crate) mod operator_id;
use operator_id::*;
// ----------------------------------------------------------------------------
// Index
/// Type used for indexing vectors in the tape.
/// It must be able to represent the total number of
/// operators, constants, and arguments to operators.
pub type Index = usize;
//
// Float
/// Floating point Type used for AD operations.
pub type Float = f64;
//
// ForwardZeroFn
/// Type used for fuunctions that evaluate zero order forward mode
pub type ForwardZeroFn = fn(
        _var: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index
);
// ----------------------------------------------------------------------------
//
// AD
pub mod ad;
use ad::AD;
//
// OpInfo
#[derive(Clone)]
pub struct OpInfo {
    pub name : String,
    pub fun : ForwardZeroFn,
}

//
// panic_eval_fn
fn panic_eval_fn(
    _vec: &mut Vec<Float>, _con: &Vec<Float>, _arg: &[Index], _res: Index) {
    panic!();
}
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
    pub static THIS_THREAD_TAPE: std::cell::RefCell<TapeInfo> =
        std::cell::RefCell::new( TapeInfo::new() );
}
//
// ad_fun
pub mod ad_fun;
//
// add_op
mod add_op;
//
// OP_INFO_VEC
fn op_info_vec() -> Vec<OpInfo> {
    let empty         = OpInfo{ name: "".to_string(), fun : panic_eval_fn };
    let mut result    = vec![empty ; NUMBER_OP ];
    result[ADD_VC_OP] =
        OpInfo{ name : "add_vc".to_string() , fun : add_op::eval_add_vc_fn };
    result[ADD_VV_OP] =
        OpInfo{ name : "add_vv".to_string() , fun : add_op::eval_add_vv_fn };
    result
}
pub static OP_INFO_VEC: std::sync::LazyLock< Vec<OpInfo> > =
   std::sync::LazyLock::new( || op_info_vec() );

#[test]
fn test_op_info() {
    let op_info_vec = &*OP_INFO_VEC;
    assert_eq!( "add_vc", op_info_vec[ADD_VC_OP].name );
    assert_eq!( "add_vv", op_info_vec[ADD_VV_OP].name );
}
